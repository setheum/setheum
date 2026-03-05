// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package resharing

import (
	"context"
	"encoding/json"
	"errors"
	"math/big"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/common"
	"github.com/ChainSafe/sygma-relayer/tss/util"
	"github.com/binance-chain/tss-lib/ecdsa/keygen"
	"github.com/binance-chain/tss-lib/ecdsa/resharing"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"golang.org/x/exp/slices"
)

type startParams struct {
	OldThreshold int       `json:"oldThreshold"`
	OldSubset    []peer.ID `json:"oldSubset"`
}

type SaveDataStorer interface {
	GetKeyshare() (keyshare.ECDSAKeyshare, error)
	StoreKeyshare(keyshare keyshare.ECDSAKeyshare) error
	LockKeyshare()
	UnlockKeyshare()
}

type Resharing struct {
	common.BaseTss
	key            keyshare.ECDSAKeyshare
	subscriptionID comm.SubscriptionID
	storer         SaveDataStorer
	newThreshold   int
}

func NewResharing(
	sessionID string,
	threshold int,
	host host.Host,
	comm comm.Communication,
	storer SaveDataStorer,
) *Resharing {
	storer.LockKeyshare()
	var key keyshare.ECDSAKeyshare
	key, err := storer.GetKeyshare()
	if err != nil {
		// empty key for parties that don't have one
		key = keyshare.ECDSAKeyshare{}
	}

	partyStore := make(map[string]*tss.PartyID)
	return &Resharing{
		BaseTss: common.BaseTss{
			PartyStore:    partyStore,
			Host:          host,
			Communication: comm,
			Peers:         host.Peerstore().Peers(),
			SID:           sessionID,
			Log:           log.With().Str("SessionID", sessionID).Str("Process", "resharing").Logger(),
			Cancel:        func() {},
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

	startParams, err := r.unmarshallStartParams(params)
	if err != nil {
		return err
	}

	oldParties := common.PartiesFromPeers(startParams.OldSubset)
	oldCtx := tss.NewPeerContext(oldParties)
	newParties := r.sortParties(common.PartiesFromPeers(r.Host.Peerstore().Peers()), oldParties)
	newCtx := tss.NewPeerContext(newParties)
	r.PopulatePartyStore(newParties)
	tssParams, err := tss.NewReSharingParameters(
		tss.S256(),
		oldCtx,
		newCtx,
		r.PartyStore[r.Host.ID().Pretty()],
		len(oldParties),
		startParams.OldThreshold,
		len(newParties),
		r.newThreshold,
	)
	if err != nil {
		return err
	}

	endChn := make(chan keygen.LocalPartySaveData)
	outChn := make(chan tss.Message)
	msgChn := make(chan *comm.WrappedMessage)
	r.subscriptionID = r.Communication.Subscribe(r.SessionID(), comm.TssReshareMsg, msgChn)

	r.Party, err = resharing.NewLocalParty(tssParams, r.key.Key, outChn, endChn, new(big.Int).SetBytes([]byte(r.SID)))
	if err != nil {
		return err
	}

	p := pool.New().WithContext(ctx).WithCancelOnError()
	p.Go(func(ctx context.Context) error { return r.ProcessOutboundMessages(ctx, outChn, comm.TssReshareMsg) })
	p.Go(func(ctx context.Context) error { return r.ProcessInboundMessages(ctx, msgChn) })
	p.Go(func(ctx context.Context) error { return r.processEndMessage(ctx, endChn) })

	r.Log.Info().Msgf("Started resharing process")

	tssError := r.Party.Start()
	if tssError != nil {
		return tssError
	}

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

// ValidCoordinators returns only peers that have a valid keyshare from the previous resharing
// inside host peerstore
func (r *Resharing) ValidCoordinators() []peer.ID {
	peers := r.BaseTss.Host.Peerstore().Peers()
	validCoordinators := make(peer.IDSlice, 0)
	for _, peer := range peers {
		for _, subsetPeer := range r.key.Peers {
			if subsetPeer.Pretty() == peer.Pretty() {
				validCoordinators = append(validCoordinators, peer)
				break
			}
		}
	}
	return validCoordinators
}

// StartParams returns threshold and peer subset from the old key to share with new parties.
func (r *Resharing) StartParams(readyPeers []peer.ID) []byte {
	oldSubset := common.PeersIntersection(r.key.Peers, r.Host.Peerstore().Peers())
	startParams := &startParams{
		OldThreshold: r.key.Threshold,
		OldSubset:    oldSubset,
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

	err = r.validateStartParams(startParams)
	if err != nil {
		return startParams, err
	}

	return startParams, nil
}

func (r *Resharing) validateStartParams(params startParams) error {
	if params.OldThreshold <= 0 {
		return errors.New("threshold too small")
	}
	if len(params.OldSubset) < params.OldThreshold {
		return errors.New("threshold bigger then subset")
	}

	slices.Sort(params.OldSubset)
	slices.Sort(r.key.Peers)
	// if relayer is already part of the active subset, check if peer subset
	// in starting params is same as one saved in keyshare
	if len(r.key.Peers) != 0 && !slices.Equal(params.OldSubset, common.PeersIntersection(r.key.Peers, r.Host.Peerstore().Peers())) {
		return errors.New("invalid peers subset in start params")
	}

	return nil
}

// processEndMessage routes signature to result channel.
func (r *Resharing) processEndMessage(ctx context.Context, endChn chan keygen.LocalPartySaveData) error {
	defer r.Cancel()
	for {
		select {
		case key := <-endChn:
			{
				r.Log.Info().Msg("Successfully reshared key")

				keyshare := keyshare.NewECDSAKeyshare(key, r.newThreshold, r.Peers)
				err := r.storer.StoreKeyshare(keyshare)
				return err
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}

// sortParties assign new parties indexes that are greater than old party indexes to prevent
// errors when assigning message to a party
func (r *Resharing) sortParties(parties tss.SortedPartyIDs, oldParties tss.SortedPartyIDs) tss.SortedPartyIDs {
	newParties := make(tss.SortedPartyIDs, len(parties))
	copy(newParties, oldParties)
	index := len(oldParties)
	oldPeers, _ := common.PeersFromParties(oldParties)

	for _, party := range parties {
		peerID, _ := peer.Decode(party.Id)
		if !util.IsParticipant(peerID, oldPeers) {
			newParties[index] = party
			newParties[index].Index = index
			index++
		}
	}
	return newParties
}

func (r *Resharing) Retryable() bool {
	return false
}
