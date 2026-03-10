package eventHandlers

import (
	"context"
	"fmt"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type EventListener interface {
	FetchKeygenEvents(ctx context.Context, address common.Address, startBlock *big.Int, endBlock *big.Int) ([]types.Log, error)
	FetchFrostKeygenEvents(ctx context.Context, address common.Address, startBlock *big.Int, endBlock *big.Int) ([]types.Log, error)
	FetchRefreshEvents(ctx context.Context, address common.Address, startBlock *big.Int, endBlock *big.Int) ([]*events.Refresh, error)
	FetchDeposits(ctx context.Context, address common.Address, startBlock *big.Int, endBlock *big.Int) ([]*events.Deposit, error)
	FetchRetryV1Events(ctx context.Context, contractAddress common.Address, startBlock *big.Int, endBlock *big.Int) ([]events.RetryV1Event, error)
	FetchRetryV2Events(ctx context.Context, contractAddress common.Address, startBlock *big.Int, endBlock *big.Int) ([]events.RetryV2Event, error)
	FetchRetryDepositEvents(event events.RetryV1Event, bridgeAddress common.Address, blockConfirmations *big.Int) ([]events.Deposit, error)
}

type DepositHandler interface {
	HandleDeposit(sourceID, destID uint8, nonce uint64, resourceID [32]byte, calldata, handlerResponse []byte, messageID string, timestamp time.Time) (*message.Message, error)
}

type DepositEventHandler struct {
	eventListener  EventListener
	depositHandler DepositHandler
	bridgeAddress  common.Address
	domainID       uint8
	msgChan        chan []*message.Message
}

func NewDepositEventHandler(eventListener EventListener, depositHandler DepositHandler, bridgeAddress common.Address, domainID uint8, msgChan chan []*message.Message) *DepositEventHandler {
	return &DepositEventHandler{
		eventListener:  eventListener,
		depositHandler: depositHandler,
		bridgeAddress:  bridgeAddress,
		domainID:       domainID,
		msgChan:        msgChan,
	}
}

func (eh *DepositEventHandler) HandleEvents(startBlock *big.Int, endBlock *big.Int) error {
	domainDeposits, err := eh.ProcessDeposits(startBlock, endBlock)
	if err != nil {
		return err
	}

	for _, deposits := range domainDeposits {
		go func(d []*message.Message) {
			eh.msgChan <- d
		}(deposits)
	}

	return nil
}

func (eh *DepositEventHandler) ProcessDeposits(startBlock *big.Int, endBlock *big.Int) (map[uint8][]*message.Message, error) {
	deposits, err := eh.eventListener.FetchDeposits(context.Background(), eh.bridgeAddress, startBlock, endBlock)
	if err != nil {
		return nil, fmt.Errorf("unable to fetch deposit events because of: %+v", err)
	}

	domainDeposits := make(map[uint8][]*message.Message)
	for _, d := range deposits {
		func(d *events.Deposit) {
			defer func() {
				if r := recover(); r != nil {
					log.Error().Err(err).Msgf("panic occured while handling deposit %+v", d)
				}
			}()

			messageID := fmt.Sprintf("%d-%d-%d-%d", eh.domainID, d.DestinationDomainID, startBlock, endBlock)
			m, err := eh.depositHandler.HandleDeposit(eh.domainID, d.DestinationDomainID, d.DepositNonce, d.ResourceID, d.Data, d.HandlerResponse, messageID, d.Timestamp)
			if err != nil {
				log.Error().Err(err).Str("start block", startBlock.String()).Str("end block", endBlock.String()).Uint8("domainID", eh.domainID).Msgf("%v", err)
				return
			}

			log.Info().Str("messageID", m.ID).Msgf("Resolved message %+v in block range: %s-%s", m, startBlock.String(), endBlock.String())
			domainDeposits[m.Destination] = append(domainDeposits[m.Destination], m)
		}(d)
	}

	return domainDeposits, nil
}
