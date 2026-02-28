// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

package listener

import (
	"context"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/btc/config"
	"github.com/btcsuite/btcd/btcjson"
	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

type EventHandler interface {
	HandleEvents(startBlock *big.Int) error
}
type BlockStorer interface {
	StoreBlock(block *big.Int, domainID uint8) error
}
type Connection interface {
	GetRawTransactionVerbose(*chainhash.Hash) (*btcjson.TxRawResult, error)
	GetBlockHash(int64) (*chainhash.Hash, error)
	GetBlockVerboseTx(*chainhash.Hash) (*btcjson.GetBlockVerboseTxResult, error)
	GetBestBlockHash() (*chainhash.Hash, error)
}
type BtcListener struct {
	conn Connection

	eventHandlers      []EventHandler
	blockRetryInterval time.Duration
	blockConfirmations *big.Int
	blockstore         BlockStorer

	log      zerolog.Logger
	domainID uint8
}

// NewBtcListener creates an BtcListener that listens to deposit events on chain
// and calls event handler when one occurs
func NewBtcListener(connection Connection, eventHandlers []EventHandler, config *config.BtcConfig, blockstore BlockStorer,
) *BtcListener {
	return &BtcListener{
		log:                log.With().Uint8("domainID", *config.GeneralChainConfig.Id).Logger(),
		conn:               connection,
		eventHandlers:      eventHandlers,
		blockRetryInterval: config.BlockRetryInterval,
		blockConfirmations: config.BlockConfirmations,
		blockstore:         blockstore,
		domainID:           *config.GeneralChainConfig.Id,
	}
}

// ListenToEvents goes block by block of a network and executes event handlers that are
// configured for the listener.
func (l *BtcListener) ListenToEvents(ctx context.Context, startBlock *big.Int) {
loop:
	for {
		select {
		case <-ctx.Done():
			return
		default:
			// Get the hash of the most recent block
			bestBlockHash, err := l.conn.GetBestBlockHash()
			if err != nil {
				l.log.Warn().Err(err).Msg("Unable to get latest block")
				time.Sleep(l.blockRetryInterval)
				continue
			}

			// Fetch the most recent block in verbose mode to get additional information including height
			block, err := l.conn.GetBlockVerboseTx(bestBlockHash)
			if err != nil {
				l.log.Warn().Err(err).Msg("Unable to get latest block")
				time.Sleep(l.blockRetryInterval)
				continue
			}

			head := big.NewInt(block.Height)
			if startBlock == nil {
				startBlock = head
			}

			// Sleep if startBlock is higher then head
			if new(big.Int).Sub(head, startBlock).Cmp(l.blockConfirmations) == -1 {
				time.Sleep(l.blockRetryInterval)
				continue
			}

			log.Debug().Msgf("Fetching btc events for block %d", startBlock)

			for _, handler := range l.eventHandlers {
				err := handler.HandleEvents(startBlock)
				if err != nil {
					l.log.Warn().Err(err).Msgf("Unable to handle events")
					continue loop
				}
			}

			//Write to block store. Not a critical operation, no need to retry
			err = l.blockstore.StoreBlock(startBlock, l.domainID)
			if err != nil {
				l.log.Error().Str("block", startBlock.String()).Err(err).Msg("Failed to write latest block to blockstore")
			}
			startBlock.Add(startBlock, big.NewInt(1))
		}
	}
}
