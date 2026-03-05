// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package signing

import (
	"context"
	"encoding/hex"
	"encoding/json"

	errors "github.com/ChainSafe/sygma-relayer/tss"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"github.com/taurusgroup/multi-party-sig/pkg/math/curve"
	"github.com/taurusgroup/multi-party-sig/pkg/protocol"
	"github.com/taurusgroup/multi-party-sig/pkg/taproot"
	"github.com/taurusgroup/multi-party-sig/protocols/frost"
	"golang.org/x/exp/slices"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/tss/frost/common"
	"github.com/ChainSafe/sygma-relayer/tss/util"
)

type Signature struct {
	Id        int
	Signature taproot.Signature
}

type SaveDataFetcher interface {
	GetKeyshare() (keyshare.FrostKeyshare, error)
	LockKeyshare()
	UnlockKeyshare()
}

type Signing struct {
	common.BaseFrostTss
	id             int
	coordinator    bool
	key            keyshare.FrostKeyshare
	msg            []byte
	resultChn      chan interface{}
	subscriptionID comm.SubscriptionID
}

func NewSigning(
	id int,
	msg []byte,
	tweak string,
	messageID string,
	sessionID string,
	host host.Host,
	comm comm.Communication,
	fetcher SaveDataFetcher,
) (*Signing, error) {
	fetcher.LockKeyshare()
	defer fetcher.UnlockKeyshare()
	key, err := fetcher.GetKeyshare()
	if err != nil {
		return nil, err
	}

	tweakBytes, err := hex.DecodeString(tweak)
	if err != nil {
		return nil, err
	}

	h := &curve.Secp256k1Scalar{}
	err = h.UnmarshalBinary(tweakBytes)
	if err != nil {
		return nil, err
	}
	key.Key, err = key.Key.Derive(h, nil)
	if err != nil {
		return nil, err
	}

	return &Signing{
		BaseFrostTss: common.BaseFrostTss{
			Host:          host,
			Communication: comm,
			Peers:         key.Peers,
			SID:           sessionID,
			Log:           log.With().Str("SessionID", sessionID).Str("messageID", messageID).Str("Process", "signing").Logger(),
			Cancel:        func() {},
			Done:          make(chan bool),
		},
		key: key,
		id:  id,
		msg: msg,
	}, nil
}

// Run initializes the signing party and runs the signing tss process.
// Params contains peer subset that leaders sends with start message.
func (s *Signing) Run(
	ctx context.Context,
	coordinator bool,
	resultChn chan interface{},
	params []byte,
) error {
	s.coordinator = coordinator
	s.resultChn = resultChn
	ctx, s.Cancel = context.WithCancel(ctx)

	peerSubset, err := s.unmarshallStartParams(params)
	if err != nil {
		return err
	}
	s.Peers = peerSubset
	if !util.IsParticipant(s.Host.ID(), peerSubset) {
		return &errors.SubsetError{Peer: s.Host.ID()}
	}

	msgChn := make(chan *comm.WrappedMessage)
	s.subscriptionID = s.Communication.Subscribe(s.SessionID(), comm.TssKeySignMsg, msgChn)
	s.Handler, err = protocol.NewMultiHandler(
		frost.SignTaproot(
			s.key.Key,
			common.PartyIDSFromPeers(peerSubset),
			s.msg,
		),
		[]byte(s.SessionID()))
	if err != nil {
		return err
	}

	outChn := make(chan tss.Message)
	p := pool.New().WithContext(ctx).WithCancelOnError()
	p.Go(func(ctx context.Context) error { return s.ProcessInboundMessages(ctx, msgChn) })
	p.Go(func(ctx context.Context) error { return s.processEndMessage(ctx) })
	p.Go(func(ctx context.Context) error { return s.ProcessOutboundMessages(ctx, outChn, comm.TssKeySignMsg) })

	s.Log.Info().Msgf("Started signing process for message %s", hex.EncodeToString(s.msg))
	return p.Wait()
}

// Stop ends all subscriptions created when starting the tss process.
func (s *Signing) Stop() {
	s.Log.Info().Msgf("Stopping tss process.")
	s.Communication.UnSubscribe(s.subscriptionID)
	s.Cancel()
}

// Ready returns true if threshold+1 parties are ready to start the signing process.
func (s *Signing) Ready(readyPeers []peer.ID, excludedPeers []peer.ID) (bool, error) {
	readyPeers = s.readyParticipants(readyPeers)
	return len(readyPeers) == s.key.Threshold+1, nil
}

// ValidCoordinators returns only peers that have a valid keyshare
func (s *Signing) ValidCoordinators() []peer.ID {
	return s.key.Peers
}

// StartParams returns peer subset for this tss process. It is calculated
// by sorting hashes of peer IDs and session ID and chosing ready peers alphabetically
// until threshold is satisfied.
func (s *Signing) StartParams(readyPeers []peer.ID) []byte {
	readyPeers = s.readyParticipants(readyPeers)
	peers := []peer.ID{}
	peers = append(peers, readyPeers...)

	sortedPeers := util.SortPeersForSession(peers, s.SessionID())
	peerSubset := []peer.ID{}
	for _, peer := range sortedPeers {
		peerSubset = append(peerSubset, peer.ID)
		if len(peerSubset) == s.key.Threshold+1 {
			break
		}
	}

	paramBytes, _ := json.Marshal(peerSubset)
	return paramBytes
}

func (s *Signing) unmarshallStartParams(paramBytes []byte) ([]peer.ID, error) {
	var peerSubset []peer.ID
	err := json.Unmarshal(paramBytes, &peerSubset)
	if err != nil {
		return []peer.ID{}, err
	}

	return peerSubset, nil
}

// processEndMessage routes signature to result channel.
func (s *Signing) processEndMessage(ctx context.Context) error {
	defer s.Cancel()
	for {
		select {
		case <-s.Done:
			{
				result, err := s.Handler.Result()
				if err != nil {
					return err
				}
				s.Log.Info().Msg("Successfully generated signature")
				signature, _ := result.(taproot.Signature)

				s.resultChn <- Signature{
					Signature: signature,
					Id:        s.id,
				}
				s.Cancel()
				return nil
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}

// readyParticipants returns all ready peers that contain a valid key share
func (s *Signing) readyParticipants(readyPeers []peer.ID) []peer.ID {
	readyParticipants := make([]peer.ID, 0)
	for _, peer := range readyPeers {

		if !slices.Contains(s.key.Peers, peer) {
			continue
		}

		readyParticipants = append(readyParticipants, peer)
	}

	return readyParticipants
}

func (s *Signing) Retryable() bool {
	return true
}
