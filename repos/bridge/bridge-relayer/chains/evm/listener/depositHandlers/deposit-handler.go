// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers

import (
	"errors"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/eventHandlers"
	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type DepositHandlers map[common.Address]eventHandlers.DepositHandler
type HandlerMatcher interface {
	GetHandlerAddressForResourceID(resourceID [32]byte) (common.Address, error)
}

type ETHDepositHandler struct {
	handlerMatcher  HandlerMatcher
	depositHandlers DepositHandlers
}

// NewETHDepositHandler creates an instance of ETHDepositHandler that contains
// handler functions for processing deposit events
func NewETHDepositHandler(handlerMatcher HandlerMatcher) *ETHDepositHandler {
	return &ETHDepositHandler{
		handlerMatcher:  handlerMatcher,
		depositHandlers: make(map[common.Address]eventHandlers.DepositHandler),
	}
}

func (e *ETHDepositHandler) HandleDeposit(
	sourceID,
	destID uint8,
	depositNonce uint64,
	resourceID [32]byte,
	calldata,
	handlerResponse []byte,
	messageID string,
	timestamp time.Time) (*message.Message, error) {
	handlerAddr, err := e.handlerMatcher.GetHandlerAddressForResourceID(resourceID)
	if err != nil {
		return nil, err
	}

	depositHandler, err := e.matchAddressWithHandlerFunc(handlerAddr)
	if err != nil {
		return nil, err
	}

	return depositHandler.HandleDeposit(sourceID, destID, depositNonce, resourceID, calldata, handlerResponse, messageID, timestamp)
}

// matchAddressWithHandlerFunc matches a handler address with an associated handler function
func (e *ETHDepositHandler) matchAddressWithHandlerFunc(handlerAddress common.Address) (eventHandlers.DepositHandler, error) {
	hf, ok := e.depositHandlers[handlerAddress]
	if !ok {
		return nil, errors.New("no corresponding deposit handler for this address exists")
	}
	return hf, nil
}

// RegisterDepositHandler registers an event handler by associating a handler function to a specified address
func (e *ETHDepositHandler) RegisterDepositHandler(handlerAddress string, handler eventHandlers.DepositHandler) {
	if handlerAddress == "" {
		return
	}

	log.Debug().Msgf("Registered deposit handler for address %s", handlerAddress)
	e.depositHandlers[common.HexToAddress(handlerAddress)] = handler
}
