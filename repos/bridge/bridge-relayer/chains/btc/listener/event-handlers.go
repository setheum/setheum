// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener

import (
	"crypto/sha256"
	"encoding/binary"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/btc/config"
	"github.com/btcsuite/btcd/btcjson"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type Deposit struct {
	// ID of the resource that is transfered
	ResourceID [32]byte
	// Address of sender (msg.sender: user)
	SenderAddress string
	// Additional data to be passed to specified handler
	Amount *big.Int
	Data   string
}

type DepositHandler interface {
	HandleDeposit(
		sourceID uint8,
		depositNonce uint64,
		resourceID [32]byte,
		amount *big.Int,
		data string,
		blockNumber *big.Int,
		timestamp time.Time,
	) (*message.Message, error)
}

type FungibleTransferEventHandler struct {
	depositHandler DepositHandler
	domainID       uint8
	feeAddress     btcutil.Address
	log            zerolog.Logger
	conn           Connection
	msgChan        chan []*message.Message
	resources      map[[32]byte]config.Resource
}

func NewFungibleTransferEventHandler(
	logC zerolog.Context,
	domainID uint8,
	depositHandler DepositHandler,
	msgChan chan []*message.Message,
	conn Connection,
	resources map[[32]byte]config.Resource,
	feeAddress btcutil.Address) *FungibleTransferEventHandler {
	return &FungibleTransferEventHandler{
		depositHandler: depositHandler,
		domainID:       domainID,
		feeAddress:     feeAddress,
		log:            logC.Logger(),
		conn:           conn,
		msgChan:        msgChan,
		resources:      resources,
	}
}

func (eh *FungibleTransferEventHandler) HandleEvents(blockNumber *big.Int) error {
	domainDeposits, err := eh.ProcessDeposits(blockNumber)
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

func (eh *FungibleTransferEventHandler) ProcessDeposits(blockNumber *big.Int) (map[uint8][]*message.Message, error) {
	domainDeposits := make(map[uint8][]*message.Message)
	evts, err := eh.FetchEvents(blockNumber)
	if err != nil {
		return nil, err
	}
	for _, evt := range evts {
		err := func(evt btcjson.TxRawResult) error {
			defer func() {
				if r := recover(); r != nil {
					log.Error().Msgf("panic occured while handling deposit %+v", evt)
				}
			}()

			for _, resource := range eh.resources {
				d, isDeposit, err := DecodeDepositEvent(evt, resource, eh.feeAddress)
				if err != nil {
					return err
				}

				if !isDeposit {
					continue
				}
				nonce, err := eh.CalculateNonce(blockNumber, evt.Hash)
				if err != nil {
					return err
				}

				m, err := eh.depositHandler.HandleDeposit(eh.domainID, nonce, d.ResourceID, d.Amount, d.Data, blockNumber, time.Unix(evt.Blocktime, 0))
				if err != nil {
					return err
				}

				log.Debug().Str("messageID", m.ID).Msgf("Resolved message %+v in block: %s", m, blockNumber.String())
				domainDeposits[m.Destination] = append(domainDeposits[m.Destination], m)
				return nil
			}
			return nil
		}(evt)
		if err != nil {
			log.Error().Err(err).Msgf("Failed processing Bitcoin deposit %v", evt)
		}
	}
	return domainDeposits, nil
}

func (eh *FungibleTransferEventHandler) FetchEvents(startBlock *big.Int) ([]btcjson.TxRawResult, error) {
	blockHash, err := eh.conn.GetBlockHash(startBlock.Int64())
	if err != nil {
		return nil, err
	}

	// Fetch block details in verbose mode
	block, err := eh.conn.GetBlockVerboseTx(blockHash)
	if err != nil {
		return nil, err
	}
	return block.Tx, nil
}

func (eh *FungibleTransferEventHandler) CalculateNonce(blockNumber *big.Int, transactionHash string) (uint64, error) {
	// Convert blockNumber to string
	blockNumberStr := blockNumber.String()

	// Concatenate blockNumberStr and transactionHash with a separator
	concatenatedStr := blockNumberStr + "-" + transactionHash

	// Calculate SHA-256 hash of the concatenated string
	hash := sha256.New()
	hash.Write([]byte(concatenatedStr))
	hashBytes := hash.Sum(nil)

	// XOR fold the hash to get a 64-bit value
	var result uint64
	for i := 0; i < 4; i++ {
		part := binary.BigEndian.Uint64(hashBytes[i*8 : (i+1)*8])
		result ^= part
	}

	return result, nil
}
