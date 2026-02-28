// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package keygen

import (
	"context"
	"encoding/hex"
	"errors"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/tss/frost/common"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"github.com/taurusgroup/multi-party-sig/pkg/party"
	"github.com/taurusgroup/multi-party-sig/pkg/protocol"
	"github.com/taurusgroup/multi-party-sig/protocols/frost"
)

type FrostKeyshareStorer interface {
	StoreKeyshare(keyshare keyshare.FrostKeyshare) error
	LockKeyshare()
	UnlockKeyshare()
	GetKeyshare() (keyshare.FrostKeyshare, error)
}

type Keygen struct {
	common.BaseFrostTss
	storer         FrostKeyshareStorer
	threshold      int
	subscriptionID comm.SubscriptionID
}

func NewKeygen(
	sessionID string,
	threshold int,
	host host.Host,
	comm comm.Communication,
	storer FrostKeyshareStorer,
) *Keygen {
	storer.LockKeyshare()
	return &Keygen{
		BaseFrostTss: common.BaseFrostTss{
			Host:          host,
			Communication: comm,
			Peers:         host.Peerstore().Peers(),
			SID:           sessionID,
			Log:           log.With().Str("SessionID", sessionID).Str("Process", "keygen").Logger(),
			Cancel:        func() {},
			Done:          make(chan bool),
		},
		storer:    storer,
		threshold: threshold,
	}
}

// Run initializes the keygen party and runs the keygen tss process.
//
// Should be run only after all the participating parties are ready.
func (k *Keygen) Run(
	ctx context.Context,
	coordinator bool,
	resultChn chan interface{},
	params []byte,
) error {
	ctx, k.Cancel = context.WithCancel(ctx)

	outChn := make(chan tss.Message)
	msgChn := make(chan *comm.WrappedMessage)
	k.subscriptionID = k.Communication.Subscribe(k.SessionID(), comm.TssKeyGenMsg, msgChn)

	var err error
	k.Handler, err = protocol.NewMultiHandler(
		frost.KeygenTaproot(
			party.ID(k.Host.ID().String()),
			common.PartyIDSFromPeers(append(k.Host.Peerstore().Peers(), k.Host.ID())),
			k.threshold),
		[]byte(k.SessionID()))
	if err != nil {
		return err
	}
	p := pool.New().WithContext(ctx).WithCancelOnError()
	p.Go(func(ctx context.Context) error { return k.ProcessInboundMessages(ctx, msgChn) })
	p.Go(func(ctx context.Context) error { return k.processEndMessage(ctx) })
	p.Go(func(ctx context.Context) error { return k.ProcessOutboundMessages(ctx, outChn, comm.TssKeyGenMsg) })

	return p.Wait()
}

// Stop ends all subscriptions created when starting the tss process and unlocks keyshare.
func (k *Keygen) Stop() {
	k.Communication.UnSubscribe(k.subscriptionID)
	k.storer.UnlockKeyshare()
	k.Cancel()
}

// Ready returns true if all parties from the peerstore are ready.
// Error is returned if excluded peers exist as we need all peers to participate
// in keygen process.
func (k *Keygen) Ready(readyPeers []peer.ID, excludedPeers []peer.ID) (bool, error) {
	if len(excludedPeers) > 0 {
		return false, errors.New("error")
	}

	return len(readyPeers) == len(k.Host.Peerstore().Peers()), nil
}

// ValidCoordinators returns all peers in peerstore
func (k *Keygen) ValidCoordinators() []peer.ID {
	return k.Peers
}

func (k *Keygen) StartParams(readyPeers []peer.ID) []byte {
	return []byte{}
}

func (k *Keygen) Retryable() bool {
	return false
}

// processEndMessage waits for the final message with generated key share and stores it locally.
func (k *Keygen) processEndMessage(ctx context.Context) error {

	for {
		select {
		case <-k.Done:
			{
				result, err := k.Handler.Result()
				if err != nil {
					return err
				}
				taprootConfig := result.(*frost.TaprootConfig)

				err = k.storer.StoreKeyshare(keyshare.NewFrostKeyshare(taprootConfig, k.threshold, k.Peers))
				if err != nil {
					return err
				}

				k.Log.Info().Msgf("Generated public key %s", hex.EncodeToString(taprootConfig.PublicKey))
				k.Cancel()
				return nil
			}
		case <-ctx.Done():
			return nil
		}
	}
}
