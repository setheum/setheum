// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package btc

import (
	"context"
	"math/big"

	"github.com/ChainSafe/sygma-relayer/chains/btc/executor"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type BatchProposalExecutor interface {
	Execute(msgs []*message.Message) error
}
type EventListener interface {
	ListenToEvents(ctx context.Context, startBlock *big.Int)
}
type BtcChain struct {
	id uint8

	listener EventListener
	executor *executor.Executor
	mh       *message.MessageHandler

	startBlock *big.Int
	logger     zerolog.Logger
}

func NewBtcChain(
	listener EventListener,
	executor *executor.Executor,
	mh *message.MessageHandler,
	id uint8,
) *BtcChain {
	return &BtcChain{
		listener: listener,
		executor: executor,
		mh:       mh,
		id:       id,

		logger: log.With().Uint8("domainID", id).Logger()}
}

func (c *BtcChain) Write(props []*proposal.Proposal) error {
	err := c.executor.Execute(props)
	if err != nil {
		c.logger.Err(err).Str("messageID", props[0].MessageID).Msgf("error writing proposals %+v on network %d", props, c.DomainID())
		return err
	}

	return nil
}

func (c *BtcChain) ReceiveMessage(m *message.Message) (*proposal.Proposal, error) {
	return c.mh.HandleMessage(m)
}

// PollEvents is the goroutine that polls blocks and searches Deposit events in them.
// Events are then sent to eventsChan.
func (c *BtcChain) PollEvents(ctx context.Context) {
	c.logger.Info().Str("startBlock", c.startBlock.String()).Msg("Polling Blocks...")
	go c.listener.ListenToEvents(ctx, c.startBlock)
}

func (c *BtcChain) DomainID() uint8 {
	return c.id
}
