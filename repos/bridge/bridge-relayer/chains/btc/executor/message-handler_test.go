package executor_test

import (
	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/btc/executor"
	mock_executor "github.com/ChainSafe/sygma-relayer/chains/btc/executor/mock"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/btcsuite/btcd/btcjson"
	"github.com/ethereum/go-ethereum/common"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type BtcMessageHandlerTestSuite struct {
	suite.Suite
}

func TestRunBtcMessageHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(BtcMessageHandlerTestSuite))
}

func (s *BtcMessageHandlerTestSuite) Test_ERC20HandleMessage_ValidMessage() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				big.NewInt(100000045678).Bytes(), // amount
				[]byte("tb1pffdrehs8455lgnwquggf4dzf6jduz8v7d2usflyujq4ggh4jaapqpfjj83"),
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.FungibleMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
	s.Equal(prop, &proposal.Proposal{
		Source:      1,
		Destination: 0,
		Data: executor.BtcTransferProposalData{
			Amount:       10,
			Recipient:    "tb1pffdrehs8455lgnwquggf4dzf6jduz8v7d2usflyujq4ggh4jaapqpfjj83",
			DepositNonce: 1,
		},
		Type: transfer.TransferProposalType,
	})
}

func (s *BtcMessageHandlerTestSuite) Test_ERC20HandleMessage_ValidMessage_Dust() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				big.NewInt(1).Bytes(), // amount
				[]byte("tb1pffdrehs8455lgnwquggf4dzf6jduz8v7d2usflyujq4ggh4jaapqpfjj83"),
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.FungibleMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
	s.Equal(prop, &proposal.Proposal{
		Source:      1,
		Destination: 0,
		Data: executor.BtcTransferProposalData{
			Amount:       0,
			Recipient:    "tb1pffdrehs8455lgnwquggf4dzf6jduz8v7d2usflyujq4ggh4jaapqpfjj83",
			DepositNonce: 1,
		},
		Type: transfer.TransferProposalType,
	})
}

func (s *BtcMessageHandlerTestSuite) Test_ERC20HandleMessage_IncorrectDataLen() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // amount
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.FungibleMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
}

func (s *BtcMessageHandlerTestSuite) Test_ERC20HandleMessage_IncorrectAmount() {

	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				"incorrectAmount", // amount
				[]byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}, // recipientAddress
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.FungibleMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
}

func (s *BtcMessageHandlerTestSuite) Test_ERC20HandleMessage_IncorrectRecipient() {
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

	mh := executor.FungibleMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
}

func (s *BtcMessageHandlerTestSuite) Test_HandleMessage_InvalidType() {
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
			Type: transfer.NonFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.FungibleMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
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
		big.NewInt(5),
		s.mockPropStorer,
		s.msgChan)
}

func (s *RetryMessageHandlerTestSuite) Test_HandleMessage_RetryTooNew() {
	s.mockBlockFetcher.EXPECT().GetBestBlockHash().Return(nil, nil)
	s.mockBlockFetcher.EXPECT().GetBlockVerboseTx(gomock.Any()).Return(&btcjson.GetBlockVerboseTxResult{
		Height: 105,
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
	s.mockBlockFetcher.EXPECT().GetBestBlockHash().Return(nil, nil)
	s.mockBlockFetcher.EXPECT().GetBlockVerboseTx(gomock.Any()).Return(&btcjson.GetBlockVerboseTxResult{
		Height: 106,
	}, nil)
	s.mockDepositProcessor.EXPECT().ProcessDeposits(big.NewInt(100)).Return(make(map[uint8][]*message.Message), nil)

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
	s.mockBlockFetcher.EXPECT().GetBestBlockHash().Return(nil, nil)
	s.mockBlockFetcher.EXPECT().GetBlockVerboseTx(gomock.Any()).Return(&btcjson.GetBlockVerboseTxResult{
		Height: 106,
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
	s.mockDepositProcessor.EXPECT().ProcessDeposits(big.NewInt(100)).Return(deposits, nil)
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
