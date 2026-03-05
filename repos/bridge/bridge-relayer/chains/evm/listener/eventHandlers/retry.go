package eventHandlers

import (
	"context"
	"fmt"
	"math/big"
	"strings"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/consts"
	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type RetryV2EventHandler struct {
	log           zerolog.Logger
	eventListener EventListener
	retryAddress  common.Address
	domainID      uint8
	msgChan       chan []*message.Message
}

func NewRetryV2EventHandler(
	logC zerolog.Context,
	eventListener EventListener,
	retryAddress common.Address,
	domainID uint8,
	msgChan chan []*message.Message,
) *RetryV2EventHandler {
	return &RetryV2EventHandler{
		log:           logC.Logger(),
		eventListener: eventListener,
		retryAddress:  retryAddress,
		domainID:      domainID,
		msgChan:       msgChan,
	}
}

func (eh *RetryV2EventHandler) HandleEvents(
	startBlock *big.Int,
	endBlock *big.Int,
) error {
	retryEvents, err := eh.eventListener.FetchRetryV2Events(context.Background(), eh.retryAddress, startBlock, endBlock)
	if err != nil {
		return fmt.Errorf("unable to fetch retry v2 events because of: %+v", err)
	}

	for _, e := range retryEvents {
		messageID := fmt.Sprintf("retry-%d-%d", e.SourceDomainID, e.DestinationDomainID)
		msg := message.NewMessage(
			eh.domainID,
			e.SourceDomainID,
			retry.RetryMessageData{
				SourceDomainID:      e.SourceDomainID,
				DestinationDomainID: e.DestinationDomainID,
				BlockHeight:         e.BlockHeight,
				ResourceID:          e.ResourceID,
			},
			messageID,
			retry.RetryMessageType,
			time.Now(),
		)

		eh.log.Info().Str("messageID", messageID).Msgf(
			"Resolved retry message %+v in block range: %s-%s", msg, startBlock.String(), endBlock.String(),
		)
		go func() { eh.msgChan <- []*message.Message{msg} }()
	}
	return nil
}

type PropStorer interface {
	StorePropStatus(source, destination uint8, depositNonce uint64, status store.PropStatus) error
	PropStatus(source, destination uint8, depositNonce uint64) (store.PropStatus, error)
}

type RetryV1EventHandler struct {
	log                zerolog.Logger
	eventListener      EventListener
	depositHandler     DepositHandler
	propStorer         PropStorer
	bridgeAddress      common.Address
	bridgeABI          abi.ABI
	domainID           uint8
	blockConfirmations *big.Int
	msgChan            chan []*message.Message
}

func NewRetryV1EventHandler(
	logC zerolog.Context,
	eventListener EventListener,
	depositHandler DepositHandler,
	propStorer PropStorer,
	bridgeAddress common.Address,
	domainID uint8,
	blockConfirmations *big.Int,
	msgChan chan []*message.Message,
) *RetryV1EventHandler {
	bridgeABI, _ := abi.JSON(strings.NewReader(consts.BridgeABI))
	return &RetryV1EventHandler{
		log:                logC.Logger(),
		eventListener:      eventListener,
		depositHandler:     depositHandler,
		propStorer:         propStorer,
		bridgeAddress:      bridgeAddress,
		bridgeABI:          bridgeABI,
		domainID:           domainID,
		blockConfirmations: blockConfirmations,
		msgChan:            msgChan,
	}
}

func (eh *RetryV1EventHandler) HandleEvents(
	startBlock *big.Int,
	endBlock *big.Int,
) error {
	retryEvents, err := eh.eventListener.FetchRetryV1Events(context.Background(), eh.bridgeAddress, startBlock, endBlock)
	if err != nil {
		return fmt.Errorf("unable to fetch retry events because of: %+v", err)
	}

	retriesByDomain := make(map[uint8][]*message.Message)
	for _, event := range retryEvents {
		func(event events.RetryV1Event) {
			defer func() {
				if r := recover(); r != nil {
					eh.log.Error().Err(err).Msgf("panic occured while handling retry event %+v", event)
				}
			}()

			deposits, err := eh.eventListener.FetchRetryDepositEvents(event, eh.bridgeAddress, eh.blockConfirmations)
			if err != nil {
				eh.log.Error().Err(err).Msgf("Unable to fetch deposit events from event %+v", event)
				return
			}

			for _, d := range deposits {
				messageID := fmt.Sprintf("retry-%d-%d-%d-%d", eh.domainID, d.DestinationDomainID, startBlock, endBlock)
				msg, err := eh.depositHandler.HandleDeposit(
					eh.domainID, d.DestinationDomainID, d.DepositNonce,
					d.ResourceID, d.Data, d.HandlerResponse, messageID, d.Timestamp,
				)
				if err != nil {
					eh.log.Err(err).Str("messageID", msg.ID).Msgf("Failed handling deposit %+v", d)
					continue
				}
				isExecuted, err := eh.isExecuted(msg)
				if err != nil {
					eh.log.Err(err).Str("messageID", msg.ID).Msgf("Failed checking if deposit executed %+v", d)
					continue
				}
				if isExecuted {
					eh.log.Debug().Str("messageID", msg.ID).Msgf("Deposit marked as executed %+v", d)
					continue
				}

				eh.log.Info().Str("messageID", msg.ID).Msgf(
					"Resolved retry message %+v in block range: %s-%s", msg, startBlock.String(), endBlock.String(),
				)
				retriesByDomain[msg.Destination] = append(retriesByDomain[msg.Destination], msg)
			}
		}(event)
	}

	for _, retries := range retriesByDomain {
		eh.msgChan <- retries
	}

	return nil
}

func (eh *RetryV1EventHandler) isExecuted(msg *message.Message) (bool, error) {
	var err error
	propStatus, err := eh.propStorer.PropStatus(
		msg.Source,
		msg.Destination,
		msg.Data.(transfer.TransferMessageData).DepositNonce)
	if err != nil {
		return true, err
	}

	if propStatus == store.ExecutedProp {
		return true, nil
	}

	// change the status to failed if proposal is stuck to be able to retry it
	if propStatus == store.PendingProp {
		err = eh.propStorer.StorePropStatus(
			msg.Source,
			msg.Destination,
			msg.Data.(transfer.TransferMessageData).DepositNonce,
			store.FailedProp)
	}
	return false, err
}
