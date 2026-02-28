// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package eventHandlers

import (
	"context"
	"fmt"
	"math/big"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/p2p"
	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/keygen"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/resharing"
	frostKeygen "github.com/ChainSafe/sygma-relayer/tss/frost/keygen"
	frostResharing "github.com/ChainSafe/sygma-relayer/tss/frost/resharing"
	"github.com/ethereum/go-ethereum/common"
	"github.com/libp2p/go-libp2p/core/host"
)

type KeygenEventHandler struct {
	log           zerolog.Logger
	eventListener EventListener
	coordinator   *tss.Coordinator
	host          host.Host
	communication comm.Communication
	storer        keygen.ECDSAKeyshareStorer
	bridgeAddress common.Address
	threshold     int
}

func NewKeygenEventHandler(
	logC zerolog.Context,
	eventListener EventListener,
	coordinator *tss.Coordinator,
	host host.Host,
	communication comm.Communication,
	storer keygen.ECDSAKeyshareStorer,
	bridgeAddress common.Address,
	threshold int,
) *KeygenEventHandler {
	return &KeygenEventHandler{
		log:           logC.Logger(),
		eventListener: eventListener,
		coordinator:   coordinator,
		host:          host,
		communication: communication,
		storer:        storer,
		bridgeAddress: bridgeAddress,
		threshold:     threshold,
	}
}

func (eh *KeygenEventHandler) HandleEvents(
	startBlock *big.Int,
	endBlock *big.Int,
) error {
	key, err := eh.storer.GetKeyshare()
	if (key.Threshold != 0) && (err == nil) {
		return nil
	}

	keygenEvents, err := eh.eventListener.FetchKeygenEvents(
		context.Background(), eh.bridgeAddress, startBlock, endBlock,
	)
	if err != nil {
		return fmt.Errorf("unable to fetch keygen events because of: %+v", err)
	}
	if len(keygenEvents) == 0 {
		return nil
	}

	eh.log.Info().Msgf(
		"Resolved keygen message in block range: %s-%s", startBlock.String(), endBlock.String(),
	)

	keygenBlockNumber := big.NewInt(0).SetUint64(keygenEvents[0].BlockNumber)
	keygen := keygen.NewKeygen(eh.sessionID(keygenBlockNumber), eh.threshold, eh.host, eh.communication, eh.storer)
	err = eh.coordinator.Execute(context.Background(), []tss.TssProcess{keygen}, make(chan interface{}, 1))
	if err != nil {
		log.Err(err).Msgf("Failed executing keygen")
	}
	return nil
}

func (eh *KeygenEventHandler) sessionID(block *big.Int) string {
	return fmt.Sprintf("keygen-%s", block.String())
}

type FrostKeygenEventHandler struct {
	log             zerolog.Logger
	eventListener   EventListener
	coordinator     *tss.Coordinator
	host            host.Host
	communication   comm.Communication
	storer          frostKeygen.FrostKeyshareStorer
	contractAddress common.Address
	threshold       int
}

func NewFrostKeygenEventHandler(
	logC zerolog.Context,
	eventListener EventListener,
	coordinator *tss.Coordinator,
	host host.Host,
	communication comm.Communication,
	storer frostKeygen.FrostKeyshareStorer,
	contractAddress common.Address,
	threshold int,
) *FrostKeygenEventHandler {
	return &FrostKeygenEventHandler{
		log:             logC.Logger(),
		eventListener:   eventListener,
		coordinator:     coordinator,
		host:            host,
		communication:   communication,
		storer:          storer,
		contractAddress: contractAddress,
		threshold:       threshold,
	}
}

func (eh *FrostKeygenEventHandler) HandleEvents(
	startBlock *big.Int,
	endBlock *big.Int,
) error {
	keygenEvents, err := eh.eventListener.FetchFrostKeygenEvents(
		context.Background(), eh.contractAddress, startBlock, endBlock,
	)
	if err != nil {
		return fmt.Errorf("unable to fetch keygen events because of: %+v", err)
	}

	if len(keygenEvents) == 0 {
		return nil
	}

	eh.log.Info().Msgf(
		"Resolved FROST keygen message in block range: %s-%s", startBlock.String(), endBlock.String(),
	)

	keygenBlockNumber := big.NewInt(0).SetUint64(keygenEvents[0].BlockNumber)
	keygen := frostKeygen.NewKeygen(eh.sessionID(keygenBlockNumber), eh.threshold, eh.host, eh.communication, eh.storer)
	err = eh.coordinator.Execute(context.Background(), []tss.TssProcess{keygen}, make(chan interface{}, 1))
	if err != nil {
		log.Err(err).Msgf("Failed executing keygen")
	}
	return nil
}

func (eh *FrostKeygenEventHandler) sessionID(block *big.Int) string {
	return fmt.Sprintf("frost-keygen-%s", block.String())
}

type RefreshEventHandler struct {
	log              zerolog.Logger
	topologyProvider topology.NetworkTopologyProvider
	topologyStore    *topology.TopologyStore
	eventListener    EventListener
	bridgeAddress    common.Address
	coordinator      *tss.Coordinator
	host             host.Host
	communication    comm.Communication
	connectionGate   *p2p.ConnectionGate
	ecdsaStorer      resharing.SaveDataStorer
	frostStorer      frostResharing.FrostKeyshareStorer
}

func NewRefreshEventHandler(
	logC zerolog.Context,
	topologyProvider topology.NetworkTopologyProvider,
	topologyStore *topology.TopologyStore,
	eventListener EventListener,
	coordinator *tss.Coordinator,
	host host.Host,
	communication comm.Communication,
	connectionGate *p2p.ConnectionGate,
	ecdsaStorer resharing.SaveDataStorer,
	frostStorer frostResharing.FrostKeyshareStorer,
	bridgeAddress common.Address,
) *RefreshEventHandler {
	return &RefreshEventHandler{
		log:              logC.Logger(),
		topologyProvider: topologyProvider,
		topologyStore:    topologyStore,
		eventListener:    eventListener,
		coordinator:      coordinator,
		host:             host,
		communication:    communication,
		ecdsaStorer:      ecdsaStorer,
		frostStorer:      frostStorer,
		connectionGate:   connectionGate,
		bridgeAddress:    bridgeAddress,
	}
}

// HandleEvent fetches refresh events and in case of an event retrieves and stores the latest topology
// and starts a resharing tss process
func (eh *RefreshEventHandler) HandleEvents(
	startBlock *big.Int,
	endBlock *big.Int,
) error {
	refreshEvents, err := eh.eventListener.FetchRefreshEvents(
		context.Background(), eh.bridgeAddress, startBlock, endBlock,
	)
	if err != nil {
		return fmt.Errorf("unable to fetch keygen events because of: %+v", err)
	}
	if len(refreshEvents) == 0 {
		return nil
	}

	hash := refreshEvents[len(refreshEvents)-1].Hash
	if hash == "" {
		log.Error().Msgf("Hash cannot be empty string")
		return nil
	}
	topology, err := eh.topologyProvider.NetworkTopology(hash)
	if err != nil {
		log.Error().Err(err).Msgf("Failed fetching network topology")
		return nil
	}
	err = eh.topologyStore.StoreTopology(topology)
	if err != nil {
		log.Error().Err(err).Msgf("Failed storing network topology")
		return nil
	}

	eh.connectionGate.SetTopology(topology)
	p2p.LoadPeers(eh.host, topology.Peers)

	eh.log.Info().Msgf(
		"Resolved refresh message in block range: %s-%s", startBlock.String(), endBlock.String(),
	)

	resharing := resharing.NewResharing(
		eh.sessionID(startBlock), topology.Threshold, eh.host, eh.communication, eh.ecdsaStorer,
	)
	err = eh.coordinator.Execute(context.Background(), []tss.TssProcess{resharing}, make(chan interface{}, 1))
	if err != nil {
		log.Err(err).Msgf("Failed executing ecdsa key refresh")
		return nil
	}
	return nil
}

func (eh *RefreshEventHandler) sessionID(block *big.Int) string {
	return fmt.Sprintf("resharing-%s", block.String())
}
