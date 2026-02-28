// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package common

import (
	"context"
	"fmt"
	"runtime/debug"
	"time"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog"
	"github.com/taurusgroup/multi-party-sig/pkg/protocol"
)

const STARTUP_PAUSE = time.Second * 10

// BaseTss contains common variables and methods to
// all tss processes.
type BaseFrostTss struct {
	Host          host.Host
	SID           string
	Communication comm.Communication
	Log           zerolog.Logger
	Peers         []peer.ID
	Handler       *protocol.MultiHandler
	Done          chan bool

	Cancel context.CancelFunc
}

// ProcessInboundMessages processes messages from tss parties and updates local party accordingly.
func (k *BaseFrostTss) ProcessInboundMessages(ctx context.Context, msgChan chan *comm.WrappedMessage) (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("%s", string(debug.Stack()))
		}
	}()

	for {
		select {
		case wMsg := <-msgChan:
			{
				k.Log.Debug().Msgf("processed inbound message from %s", wMsg.From)

				msg := &protocol.Message{}
				err := msg.UnmarshalBinary(wMsg.Payload)
				if err != nil {
					return err
				}
				if !k.Handler.CanAccept(msg) {
					continue
				}
				go k.Handler.Accept(msg)
			}
		case <-ctx.Done():
			return nil
		}
	}
}

// ProcessOutboundMessages sends messages received from tss out channel to target peers.
// On context cancel stops listening to channel and exits.
func (k *BaseFrostTss) ProcessOutboundMessages(ctx context.Context, outChn chan tss.Message, messageType comm.MessageType) error {
	// delay sending messages until everyone is ready to accept them
	time.Sleep(STARTUP_PAUSE)

	for {
		select {
		case msg, ok := <-k.Handler.Listen():
			{
				if !ok {
					k.Done <- true
					return nil
				}

				msgBytes, err := msg.MarshalBinary()
				if err != nil {
					return err
				}

				peers, err := k.BroadcastPeers(msg)
				if err != nil {
					return err
				}

				k.Log.Debug().Msgf("sending message %s to %s", msg, peers)

				err = k.Communication.Broadcast(peers, msgBytes, messageType, k.SessionID())
				if err != nil {
					return err
				}
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}

func (k *BaseFrostTss) BroadcastPeers(msg *protocol.Message) ([]peer.ID, error) {
	if msg.Broadcast {
		return k.Peers, nil
	} else {
		if string(msg.To) == "" {
			return k.Peers, nil
		}

		p, err := peer.Decode(string(msg.To))
		if err != nil {
			return nil, err
		}
		return []peer.ID{p}, nil
	}
}

func (b *BaseFrostTss) SessionID() string {
	return b.SID
}
