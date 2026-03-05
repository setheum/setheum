// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package resharing

import (
	"context"
	"encoding/json"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/tss/frost/common"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"github.com/taurusgroup/multi-party-sig/pkg/math/curve"
	"github.com/taurusgroup/multi-party-sig/pkg/party"
	"github.com/taurusgroup/multi-party-sig/pkg/protocol"
	"github.com/taurusgroup/multi-party-sig/pkg/taproot"
	"github.com/taurusgroup/multi-party-sig/protocols/frost"
)

type startParams struct {
	PublicKey          taproot.PublicKey
	VerificationShares map[party.ID]*curve.Secp256k1Point
}
type FrostKeyshareStorer interface {
	GetKeyshare() (keyshare.FrostKeyshare, error)
	StoreKeyshare(keyshare keyshare.FrostKeyshare) error
	LockKeyshare()
	UnlockKeyshare()
}

type Resharing struct {
	common.BaseFrostTss
	key            keyshare.FrostKeyshare
	subscriptionID comm.SubscriptionID
	storer         FrostKeyshareStorer
	newThreshold   int
}

func NewResharing(
	sessionID string,
	threshold int,
	host host.Host,
	comm comm.Communication,
	storer FrostKeyshareStorer,
) *Resharing {
	storer.LockKeyshare()
	var key keyshare.FrostKeyshare
	key, err := storer.GetKeyshare()
	if err != nil {
		// empty key for parties that don't have one
		key = keyshare.FrostKeyshare{
			Key: &frost.TaprootConfig{
				Threshold:          threshold,
				PublicKey:          nil,
				PrivateShare:       &curve.Secp256k1Scalar{},
				VerificationShares: make(map[party.ID]*curve.Secp256k1Point),
				ID:                 party.ID(host.ID().Pretty()),
			},
		}
	}
	key.Key.Threshold = threshold

	return &Resharing{
		BaseFrostTss: common.BaseFrostTss{
			Host:          host,
			Communication: comm,
			Peers:         host.Peerstore().Peers(),
			SID:           sessionID,
			Log:           log.With().Str("SessionID", sessionID).Str("Process", "resharing").Logger(),
			Cancel:        func() {},
			Done:          make(chan bool),
		},
		key:          key,
		storer:       storer,
		newThreshold: threshold,
	}
}

// Run initializes the signing party and runs the resharing tss process.
// Params contains peer subset that leaders sends with start message.
func (r *Resharing) Run(
	ctx context.Context,
	coordinator bool,
	resultChn chan interface{},
	params []byte,
) error {
	ctx, r.Cancel = context.WithCancel(ctx)
	var err error

	outChn := make(chan tss.Message)
	msgChn := make(chan *comm.WrappedMessage)
	r.subscriptionID = r.Communication.Subscribe(r.SessionID(), comm.TssReshareMsg, msgChn)
	startParams, err := r.unmarshallStartParams(params)
	if err != nil {
		return err
	}
	// initialize verification shares for the new relayer
	if len(r.key.Key.VerificationShares) == 0 {
		r.key.Key.VerificationShares = startParams.VerificationShares
		r.key.Key.PublicKey = startParams.PublicKey
	}

	// Add a new verification share for each party that does not already have one
	partyIds := common.PartyIDSFromPeers(append(r.Host.Peerstore().Peers(), r.Host.ID()))
	group := curve.Secp256k1{}
	for _, k := range partyIds {
		if r.key.Key.VerificationShares[k] == nil {
			r.key.Key.VerificationShares[k] = group.NewPoint().(*curve.Secp256k1Point)
		}
	}

	r.Handler, err = protocol.NewMultiHandler(
		frost.RefreshTaproot(
			r.key.Key,
			common.PartyIDSFromPeers(append(r.Host.Peerstore().Peers(), r.Host.ID()))),
		[]byte(r.SessionID()))
	if err != nil {
		return err
	}

	p := pool.New().WithContext(ctx).WithCancelOnError()
	p.Go(func(ctx context.Context) error { return r.ProcessInboundMessages(ctx, msgChn) })
	p.Go(func(ctx context.Context) error { return r.processEndMessage(ctx) })
	p.Go(func(ctx context.Context) error { return r.ProcessOutboundMessages(ctx, outChn, comm.TssReshareMsg) })

	r.Log.Info().Msgf("Started resharing process")
	return p.Wait()
}

// Stop ends all subscriptions created when starting the tss process and unlocks keyshare.
func (r *Resharing) Stop() {
	r.Log.Info().Msgf("Stopping tss process.")
	r.Communication.UnSubscribe(r.subscriptionID)
	r.storer.UnlockKeyshare()
	r.Cancel()
}

// Ready returns true if all parties from peerstore are ready
func (r *Resharing) Ready(readyPeers []peer.ID, excludedPeers []peer.ID) (bool, error) {
	return len(readyPeers) == len(r.Host.Peerstore().Peers()), nil
}

func (r *Resharing) ValidCoordinators() []peer.ID {
	return r.key.Peers
}

func (r *Resharing) StartParams(readyPeers []peer.ID) []byte {
	startParams := &startParams{
		PublicKey:          r.key.Key.PublicKey,
		VerificationShares: r.key.Key.VerificationShares,
	}
	paramBytes, _ := json.Marshal(startParams)
	return paramBytes
}

func (r *Resharing) unmarshallStartParams(paramBytes []byte) (startParams, error) {
	var startParams startParams
	err := json.Unmarshal(paramBytes, &startParams)
	if err != nil {
		return startParams, err
	}

	return startParams, nil
}

func (r *Resharing) Retryable() bool {
	return false
}

// processEndMessage waits for the final message with generated key share and stores it locally.
func (r *Resharing) processEndMessage(ctx context.Context) error {

	for {
		select {
		case <-r.Done:
			{
				result, err := r.Handler.Result()
				if err != nil {
					return err
				}
				taprootConfig := result.(*frost.TaprootConfig)

				err = r.storer.StoreKeyshare(keyshare.NewFrostKeyshare(taprootConfig, r.newThreshold, r.Peers))
				if err != nil {
					return err
				}

				r.Log.Info().Msgf("Refreshed key")
				r.Cancel()
				return nil
			}
		case <-ctx.Done():
			return nil
		}
	}
}
