// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package executor_test

import (
	"encoding/hex"
	"errors"
	"math/big"
	"testing"
	"unsafe"

	"github.com/ChainSafe/sygma-relayer/chains/substrate/executor"
	mock_executor "github.com/ChainSafe/sygma-relayer/chains/substrate/executor/mock"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/ethereum/go-ethereum/common"
	"github.com/golang/mock/gomock"

	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/e2e/substrate"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/block"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"

	"github.com/stretchr/testify/suite"
)

var errIncorrectFungibleTransferPayloadLen = errors.New("malformed payload. Len  of payload should be 2")
var errIncorrectAmount = errors.New("wrong payload amount format")
var errIncorrectRecipient = errors.New("wrong payload recipient format")

type FungibleTransferHandlerTestSuite struct {
	suite.Suite
}

func TestRunFungibleTransferHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(FungibleTransferHandlerTestSuite))
}

func (s *FungibleTransferHandlerTestSuite) TestFungibleTransferHandleMessage() {
	recipientAddr := *(*[]types.U8)(unsafe.Pointer(&substrate.SubstratePK.PublicKey))
	recipient := substrate.ConstructRecipientData(recipientAddr)

	message := &message.Message{
		Source:      1,
		Destination: 2,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{1},
			Payload: []interface{}{
				[]byte{2}, // amount
				recipient,
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}
	data, _ := hex.DecodeString("0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000002400010100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
	expectedProp := &proposal.Proposal{
		Source:      1,
		Destination: 2,
		Data: transfer.TransferProposalData{
			DepositNonce: 1,
			ResourceId:   [32]byte{1},
			Data:         data,
		},
		Type: transfer.TransferProposalType,
	}

	mh := executor.SubstrateMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.Equal(prop, expectedProp)
}

func (s *FungibleTransferHandlerTestSuite) TestFungibleTransferHandleMessageIncorrectDataLen() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{1},
			Payload: []interface{}{
				[]byte{2}, // amount
			},
			Type: transfer.FungibleTransfer,
		},

		Type: transfer.TransferMessageType,
	}

	mh := executor.SubstrateMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectFungibleTransferPayloadLen.Error())
}

func (s *FungibleTransferHandlerTestSuite) TestFungibleTransferHandleMessageIncorrectAmount() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				"incorrectAmount", // amount
				[]byte{0x8e, 0xaf, 0x4, 0x15, 0x16, 0x87, 0x73, 0x63, 0x26, 0xc9, 0xfe, 0xa1, 0x7e, 0x25, 0xfc, 0x52, 0x87, 0x61, 0x36, 0x93, 0xc9, 0x12, 0x90, 0x9c, 0xb2, 0x26, 0xaa, 0x47, 0x94, 0xf2, 0x6a, 0x48}, // recipientAddress
			},
			Type: transfer.FungibleTransfer,
		},

		Type: transfer.TransferMessageType,
	}

	mh := executor.SubstrateMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectAmount.Error())
}

func (s *FungibleTransferHandlerTestSuite) TestFungibleTransferHandleMessageIncorrectRecipient() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2},            // amount
				"incorrectRecipient", // recipientAddress
			},
			Type: transfer.FungibleTransfer,
		},

		Type: transfer.TransferMessageType,
	}

	mh := executor.SubstrateMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectRecipient.Error())
}

func (s *FungibleTransferHandlerTestSuite) TestSuccesfullyRegisterFungibleTransferMessageHandler() {
	recipientAddr := *(*[]types.U8)(unsafe.Pointer(&substrate.SubstratePK.PublicKey))
	recipient := substrate.ConstructRecipientData(recipientAddr)

	messageData := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // amount
				recipient,
			},
			Type: transfer.FungibleTransfer,
		},

		Type: transfer.TransferMessageType,
	}

	invalidMessageData := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // amount
				recipient,
			},
		},

		Type: "NonFungibleTransfer",
	}

	depositMessageHandler := message.NewMessageHandler()
	// Register FungibleTransferMessageHandler function
	depositMessageHandler.RegisterMessageHandler(transfer.TransferMessageType, &executor.SubstrateMessageHandler{})
	prop1, err1 := depositMessageHandler.HandleMessage(messageData)
	s.Nil(err1)
	s.NotNil(prop1)

	// Use unregistered transfer type
	prop2, err2 := depositMessageHandler.HandleMessage(invalidMessageData)
	s.Nil(prop2)
	s.NotNil(err2)
}

type RetryMessageHandlerTestSuite struct {
	suite.Suite

	messageHandler       *executor.RetryMessageHandler
	mockBlockFetcher     *mock_executor.MockBlockFetcher
	mockDepositProcessor *mock_executor.MockDepositProcessor
	mockPropStorer       *mock_executor.MockPropStorer
	msgChan              chan []*message.Message
}

func TestRunRetryMessageHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(RetryMessageHandlerTestSuite))
}

func (s *RetryMessageHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.mockBlockFetcher = mock_executor.NewMockBlockFetcher(ctrl)
	s.mockDepositProcessor = mock_executor.NewMockDepositProcessor(ctrl)
	s.mockPropStorer = mock_executor.NewMockPropStorer(ctrl)
	s.msgChan = make(chan []*message.Message, 1)
	s.messageHandler = executor.NewRetryMessageHandler(
		s.mockDepositProcessor,
		s.mockBlockFetcher,
		s.mockPropStorer,
		s.msgChan)
}

func (s *RetryMessageHandlerTestSuite) Test_HandleMessage_RetryNotFinalized() {
	s.mockBlockFetcher.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockBlockFetcher.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{
		Block: block.Block{
			Header: types.Header{
				Number: 99,
			},
		},
	}, nil)

	message := &message.Message{
		Source:      1,
		Destination: 3,
		Data: retry.RetryMessageData{
			SourceDomainID:      3,
			DestinationDomainID: 4,
			BlockHeight:         big.NewInt(100),
			ResourceID:          [32]byte{},
		},
		Type: transfer.TransferMessageType,
	}

	prop, err := s.messageHandler.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
}

func (s *RetryMessageHandlerTestSuite) Test_HandleMessage_NoDeposits() {
	s.mockBlockFetcher.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockBlockFetcher.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{
		Block: block.Block{
			Header: types.Header{
				Number: 101,
			},
		},
	}, nil)
	s.mockDepositProcessor.EXPECT().ProcessDeposits(big.NewInt(100), big.NewInt(100)).Return(make(map[uint8][]*message.Message), nil)

	message := &message.Message{
		Source:      1,
		Destination: 3,
		Data: retry.RetryMessageData{
			SourceDomainID:      3,
			DestinationDomainID: 4,
			BlockHeight:         big.NewInt(100),
			ResourceID:          [32]byte{},
		},
		Type: transfer.TransferMessageType,
	}

	prop, err := s.messageHandler.HandleMessage(message)

	s.Nil(prop)
	s.Nil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryMessageHandlerTestSuite) Test_HandleMessage_ValidDeposits() {
	s.mockBlockFetcher.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockBlockFetcher.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{
		Block: block.Block{
			Header: types.Header{
				Number: 101,
			},
		},
	}, nil)

	validResource := evm.SliceTo32Bytes(common.LeftPadBytes([]byte{3}, 31))
	invalidResource := evm.SliceTo32Bytes(common.LeftPadBytes([]byte{4}, 31))
	invalidDomain := uint8(3)
	validDomain := uint8(4)

	executedNonce := uint64(1)
	failedNonce := uint64(3)

	deposits := make(map[uint8][]*message.Message)
	deposits[invalidDomain] = []*message.Message{
		{
			Destination: invalidDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: 1,
				ResourceId:   validResource,
			},
		},
	}
	deposits[validDomain] = []*message.Message{
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: executedNonce,
				ResourceId:   validResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: 2,
				ResourceId:   invalidResource,
			},
		},
		{
			Source:      invalidDomain,
			Destination: validDomain,
			Data: transfer.TransferMessageData{
				DepositNonce: failedNonce,
				ResourceId:   validResource,
			},
		},
	}
	s.mockDepositProcessor.EXPECT().ProcessDeposits(big.NewInt(100), big.NewInt(100)).Return(deposits, nil)
	s.mockPropStorer.EXPECT().PropStatus(invalidDomain, validDomain, executedNonce).Return(store.ExecutedProp, nil)
	s.mockPropStorer.EXPECT().PropStatus(invalidDomain, validDomain, failedNonce).Return(store.FailedProp, nil)

	message := &message.Message{
		Source:      1,
		Destination: 3,
		Data: retry.RetryMessageData{
			SourceDomainID:      invalidDomain,
			DestinationDomainID: validDomain,
			BlockHeight:         big.NewInt(100),
			ResourceID:          validResource,
		},
		Type: transfer.TransferMessageType,
	}

	prop, err := s.messageHandler.HandleMessage(message)

	s.Nil(prop)
	s.Nil(err)
	msgs := <-s.msgChan
	s.Equal(msgs[0].Data.(transfer.TransferMessageData).DepositNonce, failedNonce)
	s.Equal(msgs[0].Destination, validDomain)
}
