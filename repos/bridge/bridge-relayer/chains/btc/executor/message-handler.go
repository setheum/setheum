// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package executor

import (
	"errors"
	"fmt"
	"math/big"

	"github.com/btcsuite/btcd/btcjson"
	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"

	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
)

type BtcTransferProposalData struct {
	Amount       uint64
	Recipient    string
	DepositNonce uint64
	ResourceId   [32]byte
}

type BtcTransferProposal struct {
	Source      uint8
	Destination uint8
	Data        BtcTransferProposalData
}

type FungibleMessageHandler struct{}

func (h *FungibleMessageHandler) HandleMessage(msg *message.Message) (*proposal.Proposal, error) {
	transferMessage := &transfer.TransferMessage{
		Source:      msg.Source,
		Destination: msg.Destination,
		Data:        msg.Data.(transfer.TransferMessageData),
		Type:        msg.Type,
		ID:          msg.ID,
	}

	switch transferMessage.Data.Type {
	case transfer.FungibleTransfer:
		return ERC20MessageHandler(transferMessage)
	}
	return nil, errors.New("wrong message type passed while handling message")
}

func ERC20MessageHandler(msg *transfer.TransferMessage) (*proposal.Proposal, error) {
	if len(msg.Data.Payload) != 2 {
		return nil, errors.New("malformed payload. Len  of payload should be 2")
	}
	amount, ok := msg.Data.Payload[0].([]byte)
	if !ok {
		return nil, errors.New("wrong payload amount format")
	}
	recipient, ok := msg.Data.Payload[1].([]byte)
	if !ok {
		return nil, errors.New("wrong payload recipient format")
	}
	bigAmount := new(big.Int).SetBytes(amount)

	// remove 10 decimal places to match Bitcoin network
	divisor := new(big.Int)
	divisor.Exp(big.NewInt(10), big.NewInt(10), nil)
	bigAmount.Div(bigAmount, divisor)

	return proposal.NewProposal(msg.Source, msg.Destination, BtcTransferProposalData{
		Amount:       bigAmount.Uint64(),
		Recipient:    string(recipient),
		DepositNonce: msg.Data.DepositNonce,
		ResourceId:   msg.Data.ResourceId,
	}, msg.ID, transfer.TransferProposalType), nil
}

type BlockFetcher interface {
	GetBlockVerboseTx(*chainhash.Hash) (*btcjson.GetBlockVerboseTxResult, error)
	GetBestBlockHash() (*chainhash.Hash, error)
}

type PropStorer interface {
	StorePropStatus(source, destination uint8, depositNonce uint64, status store.PropStatus) error
	PropStatus(source, destination uint8, depositNonce uint64) (store.PropStatus, error)
}

type DepositProcessor interface {
	ProcessDeposits(blockNumber *big.Int) (map[uint8][]*message.Message, error)
}

type RetryMessageHandler struct {
	depositProcessor   DepositProcessor
	blockFetcher       BlockFetcher
	blockConfirmations *big.Int
	propStorer         PropStorer
	msgChan            chan []*message.Message
}

func NewRetryMessageHandler(
	depositProcessor DepositProcessor,
	blockFetcher BlockFetcher,
	blockConfirmations *big.Int,
	propStorer PropStorer,
	msgChan chan []*message.Message) *RetryMessageHandler {
	return &RetryMessageHandler{
		depositProcessor:   depositProcessor,
		blockFetcher:       blockFetcher,
		blockConfirmations: blockConfirmations,
		propStorer:         propStorer,
		msgChan:            msgChan,
	}
}

func (h *RetryMessageHandler) HandleMessage(msg *message.Message) (*proposal.Proposal, error) {
	retryData := msg.Data.(retry.RetryMessageData)
	hash, err := h.blockFetcher.GetBestBlockHash()
	if err != nil {
		return nil, err
	}
	block, err := h.blockFetcher.GetBlockVerboseTx(hash)
	if err != nil {
		return nil, err
	}
	latestBlock := big.NewInt(block.Height)
	if latestBlock.Cmp(new(big.Int).Add(retryData.BlockHeight, h.blockConfirmations)) != 1 {
		return nil, fmt.Errorf(
			"latest block %s higher than receipt block number + block confirmations %s",
			latestBlock,
			new(big.Int).Add(retryData.BlockHeight, h.blockConfirmations),
		)
	}

	domainDeposits, err := h.depositProcessor.ProcessDeposits(retryData.BlockHeight)
	if err != nil {
		return nil, err
	}
	filteredDeposits, err := retry.FilterDeposits(h.propStorer, domainDeposits, retryData.ResourceID, retryData.DestinationDomainID)
	if err != nil {
		return nil, err
	}
	if len(filteredDeposits) == 0 {
		return nil, nil
	}

	h.msgChan <- filteredDeposits
	return nil, nil
}
