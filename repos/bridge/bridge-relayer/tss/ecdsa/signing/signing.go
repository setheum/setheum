// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package signing

import (
	"context"
	"encoding/json"
	"fmt"
	"math/big"
	"reflect"
	"time"

	tssCommon "github.com/binance-chain/tss-lib/common"
	"github.com/binance-chain/tss-lib/ecdsa/signing"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"golang.org/x/exp/slices"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	errors "github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/common"
	"github.com/ChainSafe/sygma-relayer/tss/util"
)

type SaveDataFetcher interface {
	GetKeyshare() (keyshare.ECDSAKeyshare, error)
	LockKeyshare()
	UnlockKeyshare()
}

type Signing struct {
	common.BaseTss
	coordinator    bool
	key            keyshare.ECDSAKeyshare
	msg            *big.Int
	resultChn      chan interface{}
	subscriptionID comm.SubscriptionID
}

func NewSigning(
	msg *big.Int,
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

	partyStore := make(map[string]*tss.PartyID)
	return &Signing{
		BaseTss: common.BaseTss{
			PartyStore:    partyStore,
			Host:          host,
			Communication: comm,
			Peers:         key.Peers,
			SID:           sessionID,
			Log:           log.With().Str("SessionID", sessionID).Str("messageID", messageID).Str("Process", "signing").Logger(),
			Cancel:        func() {},
		},
		key: key,
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

	if !util.IsParticipant(s.Host.ID(), peerSubset) {
		return &errors.SubsetError{Peer: s.Host.ID()}
	}

	s.Peers = peerSubset
	parties := common.PartiesFromPeers(s.Peers)
	s.PopulatePartyStore(parties)
	pCtx := tss.NewPeerContext(parties)
	tssParams, err := tss.NewParameters(tss.S256(), pCtx, s.PartyStore[s.Host.ID().Pretty()], len(parties), s.key.Threshold)
	if err != nil {
		return err
	}

	sigChn := make(chan tssCommon.SignatureData)
	outChn := make(chan tss.Message)
	kdd := big.NewInt(0)
	s.Party, err = signing.NewLocalParty(
		s.msg,
		tssParams,
		s.key.Key,
		kdd,
		outChn,
		sigChn,
		new(big.Int).SetBytes([]byte(s.SID)))
	if err != nil {
		return err
	}

	msgChn := make(chan *comm.WrappedMessage)
	s.subscriptionID = s.Communication.Subscribe(s.SessionID(), comm.TssKeySignMsg, msgChn)

	p := pool.New().WithContext(ctx).WithCancelOnError()
	p.Go(func(ctx context.Context) error { return s.ProcessOutboundMessages(ctx, outChn, comm.TssKeySignMsg) })
	p.Go(func(ctx context.Context) error { return s.ProcessInboundMessages(ctx, msgChn) })
	p.Go(func(ctx context.Context) error { return s.processEndMessage(ctx, sigChn) })
	p.Go(func(ctx context.Context) error { return s.monitorSigning(ctx) })

	s.Log.Info().Msgf("Started signing process for message %s", s.msg.Text(16))

	tssError := s.Party.Start()
	if tssError != nil {
		return tssError
	}

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
func (s *Signing) processEndMessage(ctx context.Context, endChn chan tssCommon.SignatureData) error {
	defer s.Cancel()
	for {
		select {
		//nolint
		case sig := <-endChn:
			{
				s.Log.Info().Msg("Successfully generated signature")

				if s.coordinator {
					s.resultChn <- &sig
				} else {
					s.resultChn <- nil
				}

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

// monitorSigning checks if the process is stuck and waiting for peers and sends an error
// if it is
func (s *Signing) monitorSigning(ctx context.Context) error {
	defer s.Cancel()
	waitingFor := make([]*tss.PartyID, 0)
	ticker := time.NewTicker(time.Minute * 3)

	for {
		select {
		case <-ticker.C:
			{
				if len(waitingFor) != 0 && reflect.DeepEqual(s.Party.WaitingFor(), waitingFor) {
					err := &comm.CommunicationError{
						Err: fmt.Errorf("waiting for peers %s", waitingFor),
					}
					return err
				}

				waitingFor = s.Party.WaitingFor()
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}
