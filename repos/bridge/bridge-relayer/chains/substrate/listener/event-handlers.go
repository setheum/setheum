// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener

import (
	"fmt"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/substrate/events"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/block"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type Connection interface {
	GetFinalizedHead() (types.Hash, error)
	GetBlock(blockHash types.Hash) (*block.SignedBlock, error)
	GetBlockHash(blockNumber uint64) (types.Hash, error)
	GetBlockEvents(hash types.Hash) ([]*parser.Event, error)
	UpdateMetatdata() error
	FetchEvents(startBlock, endBlock *big.Int) ([]*parser.Event, error)
}

type SystemUpdateEventHandler struct {
	conn Connection
}

func NewSystemUpdateEventHandler(conn Connection) *SystemUpdateEventHandler {
	return &SystemUpdateEventHandler{
		conn: conn,
	}
}

func (eh *SystemUpdateEventHandler) HandleEvents(startBlock *big.Int, endBlock *big.Int) error {
	evts, err := eh.conn.FetchEvents(startBlock, endBlock)
	if err != nil {
		log.Error().Err(err).Msg("Error fetching events")
		return err
	}
	for _, e := range evts {
		if e.Name == events.ParachainUpdatedEvent {
			log.Info().Msgf("Updating substrate metadata")

			err := eh.conn.UpdateMetatdata()
			if err != nil {
				log.Error().Err(err).Msg("Unable to update Metadata")
				return err
			}
		}
	}

	return nil
}

type DepositHandler interface {
	HandleDeposit(
		sourceID uint8,
		destID types.U8,
		nonce types.U64,
		resourceID types.Bytes32,
		calldata []byte,
		transferType types.U8,
		messageID string,
		timestamp time.Time,
	) (*message.Message, error)
}

type FungibleTransferEventHandler struct {
	domainID       uint8
	depositHandler DepositHandler
	log            zerolog.Logger
	msgChan        chan []*message.Message
	conn           Connection
}

func NewFungibleTransferEventHandler(logC zerolog.Context, domainID uint8, depositHandler DepositHandler, msgChan chan []*message.Message, conn Connection) *FungibleTransferEventHandler {
	return &FungibleTransferEventHandler{
		depositHandler: depositHandler,
		domainID:       domainID,
		log:            logC.Logger(),
		msgChan:        msgChan,
		conn:           conn,
	}
}

func (eh *FungibleTransferEventHandler) HandleEvents(startBlock *big.Int, endBlock *big.Int) error {
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

func (eh *FungibleTransferEventHandler) ProcessDeposits(startBlock *big.Int, endBlock *big.Int) (map[uint8][]*message.Message, error) {
	evts, err := eh.conn.FetchEvents(startBlock, endBlock)
	if err != nil {
		log.Error().Err(err).Msg("Error fetching events")
		return nil, err
	}

	domainDeposits := make(map[uint8][]*message.Message)

	for _, evt := range evts {
		if evt.Name == events.DepositEvent {
			func(evt parser.Event) {
				defer func() {
					if r := recover(); r != nil {
						log.Error().Msgf("panic occured while handling deposit %+v", evt)
					}
				}()
				d, err := DecodeDepositEvent(evt.Fields)
				if err != nil {
					log.Error().Err(err).Msgf("%v", err)
					return
				}

				messageID := fmt.Sprintf("%d-%d-%d-%d", eh.domainID, d.DestDomainID, startBlock, endBlock)
				m, err := eh.depositHandler.HandleDeposit(
					eh.domainID, d.DestDomainID, d.DepositNonce, d.ResourceID, d.CallData, d.TransferType, messageID, d.Timestamp)
				if err != nil {
					log.Error().Err(err).Msgf("%v", err)
					return
				}

				eh.log.Info().Str("messageID", messageID).Msgf("Resolved deposit message %+v", d)
				domainDeposits[m.Destination] = append(domainDeposits[m.Destination], m)
			}(*evt)
		}
	}
	return domainDeposits, nil
}

type RetryEventHandler struct {
	conn           Connection
	domainID       uint8
	depositHandler DepositHandler
	log            zerolog.Logger
	msgChan        chan []*message.Message
}

func NewRetryEventHandler(logC zerolog.Context, conn Connection, depositHandler DepositHandler, domainID uint8, msgChan chan []*message.Message) *RetryEventHandler {
	return &RetryEventHandler{
		depositHandler: depositHandler,
		domainID:       domainID,
		conn:           conn,
		log:            logC.Logger(),
		msgChan:        msgChan,
	}
}

func (rh *RetryEventHandler) HandleEvents(startBlock *big.Int, endBlock *big.Int) error {
	evts, err := rh.conn.FetchEvents(startBlock, endBlock)
	if err != nil {
		log.Error().Err(err).Msg("Error fetching events")
		return err
	}

	hash, err := rh.conn.GetFinalizedHead()
	if err != nil {
		return err
	}
	finalized, err := rh.conn.GetBlock(hash)
	if err != nil {
		return err
	}
	finalizedBlockNumber := big.NewInt(int64(finalized.Block.Header.Number))

	domainDeposits := make(map[uint8][]*message.Message)
	for _, evt := range evts {
		if evt.Name == events.RetryEvent {
			err := func(evt parser.Event) error {
				defer func() {
					if r := recover(); r != nil {
						log.Error().Msgf("panic occured while handling retry event %+v because %s", evt, r)
					}
				}()
				er, err := DecodeRetryEvent(evt.Fields)
				if err != nil {
					return err
				}
				// (latestBlockNumber - event.DepositOnBlockHeight) == blockConfirmations
				if big.NewInt(finalizedBlockNumber.Int64()).Cmp(er.DepositOnBlockHeight.Int) == -1 {
					log.Warn().Msgf("Retry event for block number %d has not enough confirmations", er.DepositOnBlockHeight)
					return nil
				}

				bh, err := rh.conn.GetBlockHash(er.DepositOnBlockHeight.Uint64())
				if err != nil {
					return err
				}

				bEvts, err := rh.conn.GetBlockEvents(bh)
				if err != nil {
					return err
				}

				for _, event := range bEvts {
					if event.Name == events.DepositEvent {
						d, err := DecodeDepositEvent(event.Fields)
						if err != nil {
							return err
						}

						messageID := fmt.Sprintf("retry-%d-%d-%d-%d", rh.domainID, d.DestDomainID, startBlock, endBlock)
						m, err := rh.depositHandler.HandleDeposit(
							rh.domainID, d.DestDomainID, d.DepositNonce, d.ResourceID, d.CallData, d.TransferType, messageID, d.Timestamp,
						)
						if err != nil {
							return err
						}

						rh.log.Info().Str("messageID", messageID).Msgf("Resolved retry message %+v", d)

						domainDeposits[m.Destination] = append(domainDeposits[m.Destination], m)
					}
				}

				return nil
			}(*evt)
			if err != nil {
				return err
			}
		}
	}

	for _, deposits := range domainDeposits {
		rh.msgChan <- deposits
	}
	return nil
}
