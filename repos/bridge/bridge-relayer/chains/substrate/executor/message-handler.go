// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package executor

import (
	"errors"
	"fmt"
	"math/big"

	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/block"
	"github.com/ethereum/go-ethereum/common"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type SubstrateMessageHandler struct{}

func (mh *SubstrateMessageHandler) HandleMessage(m *message.Message) (*proposal.Proposal, error) {
	transferMessage := &transfer.TransferMessage{
		Source:      m.Source,
		Destination: m.Destination,
		Data:        m.Data.(transfer.TransferMessageData),
		Type:        m.Type,
		ID:          m.ID,
	}
	switch transferMessage.Data.Type {
	case transfer.FungibleTransfer:
		return fungibleTransferMessageHandler(transferMessage)
	}
	return nil, errors.New("wrong message type passed while handling message")
}

func fungibleTransferMessageHandler(m *transfer.TransferMessage) (*proposal.Proposal, error) {
	if len(m.Data.Payload) != 2 {
		return nil, errors.New("malformed payload. Len  of payload should be 2")
	}
	amount, ok := m.Data.Payload[0].([]byte)
	if !ok {
		return nil, errors.New("wrong payload amount format")
	}
	recipient, ok := m.Data.Payload[1].([]byte)
	if !ok {
		return nil, errors.New("wrong payload recipient format")
	}
	var data []byte
	data = append(data, common.LeftPadBytes(amount, 32)...) // amount (uint256)

	recipientLen := big.NewInt(int64(len(recipient))).Bytes()
	data = append(data, common.LeftPadBytes(recipientLen, 32)...)
	data = append(data, recipient...)
	return proposal.NewProposal(m.Source, m.Destination, transfer.TransferProposalData{
		DepositNonce: m.Data.DepositNonce,
		ResourceId:   m.Data.ResourceId,
		Metadata:     m.Data.Metadata,
		Data:         data,
	}, m.ID, transfer.TransferProposalType), nil
}

type PropStorer interface {
	StorePropStatus(source, destination uint8, depositNonce uint64, status store.PropStatus) error
	PropStatus(source, destination uint8, depositNonce uint64) (store.PropStatus, error)
}

type BlockFetcher interface {
	GetFinalizedHead() (types.Hash, error)
	GetBlock(blockHash types.Hash) (*block.SignedBlock, error)
}

type DepositProcessor interface {
	ProcessDeposits(startBlock *big.Int, endBlock *big.Int) (map[uint8][]*message.Message, error)
}

type RetryMessageHandler struct {
	depositProcessor DepositProcessor
	blockFetcher     BlockFetcher
	propStorer       PropStorer
	msgChan          chan []*message.Message
}

func NewRetryMessageHandler(
	depositProcessor DepositProcessor,
	blockFetcher BlockFetcher,
	propStorer PropStorer,
	msgChan chan []*message.Message) *RetryMessageHandler {
	return &RetryMessageHandler{
		depositProcessor: depositProcessor,
		blockFetcher:     blockFetcher,
		propStorer:       propStorer,
		msgChan:          msgChan,
	}
}

func (h *RetryMessageHandler) HandleMessage(msg *message.Message) (*proposal.Proposal, error) {
	retryData := msg.Data.(retry.RetryMessageData)
	hash, err := h.blockFetcher.GetFinalizedHead()
	if err != nil {
		return nil, err
	}
	finalized, err := h.blockFetcher.GetBlock(hash)
	if err != nil {
		return nil, err
	}
	latestBlock := big.NewInt(int64(finalized.Block.Header.Number))
	if latestBlock.Cmp(retryData.BlockHeight) != 1 {
		return nil, fmt.Errorf(
			"latest block %s higher than receipt block number %s",
			latestBlock,
			retryData.BlockHeight,
		)
	}

	domainDeposits, err := h.depositProcessor.ProcessDeposits(retryData.BlockHeight, retryData.BlockHeight)
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
