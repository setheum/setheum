// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"

	comm "github.com/ChainSafe/sygma-relayer/comm"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/network"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/core/protocol"
	madns "github.com/multiformats/go-multiaddr-dns"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
)

const (
	defaultBufferSize = 20480
)

type Libp2pCommunication struct {
	SessionSubscriptionManager
	h             host.Host
	protocolID    protocol.ID
	logger        zerolog.Logger
	streamManager *StreamManager
}

func NewCommunication(h host.Host, protocolID protocol.ID) Libp2pCommunication {
	logger := log.With().Str("Module", "communication").Str("Peer", h.ID().Pretty()).Logger()
	c := Libp2pCommunication{
		SessionSubscriptionManager: NewSessionSubscriptionManager(),
		h:                          h,
		protocolID:                 protocolID,
		logger:                     logger,
		streamManager:              NewStreamManager(),
	}

	// start processing incoming messages
	c.h.SetStreamHandler(c.protocolID, c.StreamHandlerFunc)
	return c
}

/** Communication interface methods **/

func (c Libp2pCommunication) CloseSession(sessionID string) {
	c.streamManager.ReleaseStreams(sessionID)
}

func (c Libp2pCommunication) Broadcast(
	peers peer.IDSlice,
	msg []byte,
	msgType comm.MessageType,
	sessionID string,
) error {
	hostID := c.h.ID()
	wMsg := comm.WrappedMessage{
		MessageType: msgType,
		SessionID:   sessionID,
		Payload:     msg,
		From:        hostID,
	}
	marshaledMsg, err := json.Marshal(wMsg)
	if err != nil {
		c.logger.Error().Err(err).Str("SessionID", sessionID).Msg("unable to marshal message")
		return err
	}
	c.logger.Debug().Str("MsgType", msgType.String()).Str("SessionID", sessionID).Msg(
		"broadcasting message",
	)

	p := pool.New().WithErrors().WithFirstError()
	for _, peerID := range peers {
		if hostID == peerID {
			continue // don't send message to itself
		}

		peerID := peerID
		p.Go(func() error {
			err := c.sendMessage(peerID, marshaledMsg, msgType, sessionID)
			if err != nil {
				return &comm.CommunicationError{
					Peer: peerID,
					Err:  err,
				}
			}

			return nil
		})
	}

	return p.Wait()
}

func (c Libp2pCommunication) Subscribe(
	sessionID string,
	msgType comm.MessageType,
	channel chan *comm.WrappedMessage,
) comm.SubscriptionID {
	subID := c.SubscribeTo(sessionID, msgType, channel)
	c.logger.Trace().Str(
		"SessionID", sessionID).Msgf(
		"subscribed to message type %s", msgType,
	)
	return subID
}

func (c Libp2pCommunication) UnSubscribe(
	subID comm.SubscriptionID,
) {
	c.UnSubscribeFrom(subID)
	c.logger.Trace().Str(
		"SessionID", subID.SessionID()).Str(
		"SubID", subID.SubscriptionIdentifier()).Msgf(
		"unsubscribed from message type %s", subID.MessageType().String(),
	)
}

/** Helper methods **/

func (c Libp2pCommunication) StreamHandlerFunc(s network.Stream) {
	defer func() {
		err := s.Close()
		if err != nil {
			log.Warn().Msgf("Error closing incoming stream because of: %s", err.Error())
		}
	}()
	c.ProcessMessagesFromStream(s)
}

func (c Libp2pCommunication) ProcessMessagesFromStream(s network.Stream) {
	remotePeerID := s.Conn().RemotePeer()
	r := bufio.NewReader(s)
	for {
		msgBytes, err := ReadStream(r)
		if err != nil {
			return
		}

		var wrappedMsg comm.WrappedMessage
		if err := json.Unmarshal(msgBytes, &wrappedMsg); nil != err {
			log.Err(err).Msg("Error unmarshaling message")
			return
		}
		wrappedMsg.From = remotePeerID

		c.logger.Trace().Str(
			"From", wrappedMsg.From.String()).Str(
			"MsgType", wrappedMsg.MessageType.String()).Str(
			"SessionID", wrappedMsg.SessionID).Msg(
			"processed message",
		)

		subscribers := c.GetSubscribers(wrappedMsg.SessionID, wrappedMsg.MessageType)
		for _, sub := range subscribers {
			sub := sub
			go func() {
				sub <- &wrappedMsg
			}()
		}
	}
}

func (c Libp2pCommunication) sendMessage(
	to peer.ID,
	msg []byte,
	msgType comm.MessageType,
	sessionID string,
) error {
	err := c.resolveDNS(to)
	if err != nil {
		return err
	}

	var stream network.Stream
	stream, err = c.streamManager.Stream(sessionID, to)
	if err != nil {
		// try to open the stream again if it failed the first time
		stream, err = c.h.NewStream(context.TODO(), to, c.protocolID)
		if err != nil {
			return err
		}
		c.streamManager.AddStream(sessionID, to, stream)
	}

	err = WriteStream(msg, bufio.NewWriterSize(stream, defaultBufferSize))
	if err != nil {
		c.logger.Error().Str("To", to.String()).Err(err).Msg("unable to send message")
		return err
	}
	c.logger.Trace().Str(
		"To", to.Pretty()).Str(
		"MsgType", msgType.String()).Str(
		"SessionID", sessionID).Msg(
		"message sent",
	)
	return nil
}

func (c Libp2pCommunication) resolveDNS(peerID peer.ID) error {
	pi := c.h.Peerstore().PeerInfo(peerID)
	resolver, err := madns.NewResolver()
	if err != nil {
		return err
	}
	if len(pi.Addrs) == 0 {
		return fmt.Errorf("peer %s has no defined addresses", peerID.Pretty())
	}

	addr, err := resolver.Resolve(context.Background(), pi.Addrs[0])
	if err != nil {
		return err
	}
	err = c.h.Connect(context.TODO(), peer.AddrInfo{
		ID:    peerID,
		Addrs: addr,
	})
	if err != nil {
		return err
	}

	return nil
}
