// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package executor_test

import (
	"encoding/hex"
	"errors"
	"math/big"
	"testing"

	mock_executor "github.com/ChainSafe/sygma-relayer/chains/evm/executor/mock"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/golang/mock/gomock"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/chains/evm/executor"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/suite"
)

var errIncorrectERC20PayloadLen = errors.New("wrong payload length 1")
var errIncorrectERC721PayloadLen = errors.New("malformed payload. Len  of payload should be 3")
var errIncorrectGenericPayloadLen = errors.New("malformed payload. Len  of payload should be 1")
var errIncorrectERC1155PayloadLen = errors.New("malformed payload. Len  of payload should be 4")

var errIncorrectAmount = errors.New("wrong payload amount format")
var errIncorrectRecipient = errors.New("wrong payload recipient format")
var errIncorrectRecipientLen = errors.New("malformed payload. Len  of recipient should be 20")
var errIncorrectTransferData = errors.New("wrong payload transferData format")
var errIncorrectTokenID = errors.New("wrong payload tokenID format")
var errIncorrectMetadata = errors.New("wrong payload metadata format")

// ERC20
type ERC20HandlerTestSuite struct {
	suite.Suite
}

func TestRunERC20HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(ERC20HandlerTestSuite))
}

func (s *ERC20HandlerTestSuite) SetupSuite()    {}
func (s *ERC20HandlerTestSuite) TearDownSuite() {}
func (s *ERC20HandlerTestSuite) SetupTest()     {}
func (s *ERC20HandlerTestSuite) TearDownTest()  {}

func (s *ERC20HandlerTestSuite) TestERC20HandleMessage() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // amount
				[]byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}, // recipientAddress
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
}

func (s *ERC20HandlerTestSuite) TestERC20HandleMessage_WithOptionalMessage() {
	amount := []byte{2}
	recipient := []byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}
	optionalMessage := []byte("optionalMessage")

	expectedData := common.LeftPadBytes(amount, 32)
	expectedData = append(expectedData, common.LeftPadBytes(big.NewInt(int64(len(recipient))).Bytes(), 32)...)
	expectedData = append(expectedData, recipient...)
	expectedData = append(expectedData, optionalMessage...)

	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				amount,
				recipient,
				optionalMessage,
			},
			Type: transfer.FungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
	s.Equal(prop.Data.(transfer.TransferProposalData).Data, expectedData)
}

func (s *ERC20HandlerTestSuite) TestERC20HandleMessageIncorrectDataLen() {
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

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectERC20PayloadLen.Error())
}

func (s *ERC20HandlerTestSuite) TestERC20HandleMessageIncorrectAmount() {

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

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectAmount.Error())
}

func (s *ERC20HandlerTestSuite) TestERC20HandleMessageIncorrectRecipient() {
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

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectRecipient.Error())
}

// ERC721
type ERC721HandlerTestSuite struct {
	suite.Suite
}

func TestRunERC721HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(ERC721HandlerTestSuite))
}

func (s *ERC721HandlerTestSuite) SetupSuite()    {}
func (s *ERC721HandlerTestSuite) TearDownSuite() {}
func (s *ERC721HandlerTestSuite) SetupTest()     {}
func (s *ERC721HandlerTestSuite) TearDownTest()  {}

func (s *ERC721HandlerTestSuite) TestERC721MessageHandlerEmptyMetadata() {

	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // tokenID
				[]byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}, // recipientAddress
				[]byte{}, // metadata
			},
			Type: transfer.NonFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
}

func (s *ERC721HandlerTestSuite) TestERC721MessageHandlerIncorrectDataLen() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // tokenID
			},
			Type: transfer.NonFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectERC721PayloadLen.Error())
}

func (s *ERC721HandlerTestSuite) TestERC721MessageHandlerIncorrectAmount() {

	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				"incorrectAmount", // tokenID
				[]byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}, // recipientAddress
				[]byte{}, // metadata
			},
			Type: transfer.NonFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectTokenID.Error())
}

func (s *ERC721HandlerTestSuite) TestERC721MessageHandlerIncorrectRecipient() {

	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // amount
				"incorrectRecipient",
				[]byte{}, // metadata
			},
			Type: transfer.NonFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectRecipient.Error())
}

func (s *ERC721HandlerTestSuite) TestERC721MessageHandlerIncorrectMetadata() {

	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{2}, // amount
				[]byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}, // recipientAddress
				"incorrectMetadata", // metadata
			},
			Type: transfer.NonFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectMetadata.Error())
}

// GENERIC
type GenericHandlerTestSuite struct {
	suite.Suite
}

func TestRunGenericHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(GenericHandlerTestSuite))
}

func (s *GenericHandlerTestSuite) SetupSuite()    {}
func (s *GenericHandlerTestSuite) TearDownSuite() {}
func (s *GenericHandlerTestSuite) SetupTest()     {}
func (s *GenericHandlerTestSuite) TearDownTest()  {}
func (s *GenericHandlerTestSuite) TestGenericHandleEvent() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]byte{}, // metadata
			},
			Type: transfer.PermissionedGenericTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
}

func (s *GenericHandlerTestSuite) TestGenericHandleEventIncorrectDataLen() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload:      []interface{}{},
			Type:         transfer.PermissionedGenericTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectGenericPayloadLen.Error())
}

func (s *GenericHandlerTestSuite) TestGenericHandleEventIncorrectMetadata() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				"incorrectMetadata", // metadata
			},
			Type: transfer.PermissionedGenericTransfer,
		},
		Type: transfer.TransferMessageType,
	}
	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.NotNil(err)
	s.EqualError(err, errIncorrectMetadata.Error())
}

// Permissionless
type PermissionlessGenericHandlerTestSuite struct {
	suite.Suite
}

func TestRunPermissionlessGenericHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(PermissionlessGenericHandlerTestSuite))
}

func (s *PermissionlessGenericHandlerTestSuite) Test_HandleMessage() {
	hash := []byte("0xhash")
	functionSig, _ := hex.DecodeString("0x654cf88c")
	contractAddress := common.HexToAddress("0x02091EefF969b33A5CE8A729DaE325879bf76f90")
	depositor := common.HexToAddress("0x5C1F5961696BaD2e73f73417f07EF55C62a2dC5b")
	maxFee := big.NewInt(200000)
	var metadata []byte
	metadata = append(metadata, hash[:]...)
	calldata := evm.ConstructPermissionlessGenericDepositData(
		metadata,
		functionSig,
		contractAddress.Bytes(),
		depositor.Bytes(),
		maxFee,
	)
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		SenderAddress:       common.HexToAddress("0x5C1F5961696BaD2e73f73417f07EF55C62a2dC5b"),
		Data:                calldata,
		HandlerResponse:     []byte{},
	}
	sourceID := uint8(1)
	message := &message.Message{
		Source:      sourceID,
		Destination: depositLog.DestinationDomainID,
		Data: transfer.TransferMessageData{
			DepositNonce: depositLog.DepositNonce,
			ResourceId:   depositLog.ResourceID,
			Payload: []interface{}{
				functionSig,
				contractAddress.Bytes(),
				common.LeftPadBytes(maxFee.Bytes(), 32),
				depositor.Bytes(),
				hash,
			},
			Type: transfer.PermissionlessGenericTransfer,
		},
		Type: transfer.TransferMessageType,
		ID:   "messageID",
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	expectedData, _ := hex.DecodeString("0000000000000000000000000000000000000000000000000000000000030d4000001402091eeff969b33a5ce8a729dae325879bf76f90145c1f5961696bad2e73f73417f07ef55c62a2dc5b307868617368")
	expected := proposal.NewProposal(
		message.Source,
		message.Destination,
		transfer.TransferProposalData{
			DepositNonce: message.Data.(transfer.TransferMessageData).DepositNonce,
			ResourceId:   message.Data.(transfer.TransferMessageData).ResourceId,
			Metadata:     message.Data.(transfer.TransferMessageData).Metadata,
			Data:         expectedData,
		},
		"messageID",
		transfer.TransferProposalType,
	)
	s.Nil(err)
	s.Equal(expected, prop)
}

// Erc1155
type Erc1155HandlerTestSuite struct {
	suite.Suite
}

func TestRunErc1155HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(Erc1155HandlerTestSuite))
}

func (s *Erc1155HandlerTestSuite) SetupSuite()    {}
func (s *Erc1155HandlerTestSuite) TearDownSuite() {}
func (s *Erc1155HandlerTestSuite) SetupTest()     {}
func (s *Erc1155HandlerTestSuite) TearDownTest()  {}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]*big.Int{
					big.NewInt(2),
				},
				[]*big.Int{
					big.NewInt(3),
				},
				[]byte{28, 58, 3, 208, 76, 2, 107, 31, 75, 66, 8, 210, 206, 5, 60, 86, 134, 230, 251, 141},
				[]byte{},
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(err)
	s.NotNil(prop)
}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message_InvalidPayloadLen() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]*big.Int{
					big.NewInt(2),
				},
				[]*big.Int{
					big.NewInt(3),
				},
				[]byte{28, 58, 3, 208, 76, 2, 107, 31, 75, 66, 8, 210, 206, 5, 60, 86, 134, 230, 251, 141},
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.EqualError(err, errIncorrectERC1155PayloadLen.Error())
}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message_InvalidTokenIDs() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				2,
				[]*big.Int{
					big.NewInt(3),
				},
				[]byte{28, 58, 3, 208, 76, 2, 107, 31, 75, 66, 8, 210, 206, 5, 60, 86, 134, 230, 251, 141},
				[]byte{},
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.EqualError(err, errIncorrectTokenID.Error())
}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message_InvalidAmounts() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]*big.Int{
					big.NewInt(2),
				},
				3,
				[]byte{28, 58, 3, 208, 76, 2, 107, 31, 75, 66, 8, 210, 206, 5, 60, 86, 134, 230, 251, 141},
				[]byte{},
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.EqualError(err, errIncorrectAmount.Error())
}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message_InvalidRecipient() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]*big.Int{
					big.NewInt(2),
				},
				[]*big.Int{
					big.NewInt(3),
				},
				"invalidRecipient",
				[]byte{},
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.EqualError(err, errIncorrectRecipient.Error())
}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message_InvalidRecipientLen() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]*big.Int{
					big.NewInt(2),
				},
				[]*big.Int{
					big.NewInt(3),
				},
				[]byte{28, 58, 3, 208, 76, 2, 107, 31, 75, 66, 8, 210, 206, 5, 60, 86, 134, 230, 251},
				[]byte{},
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.EqualError(err, errIncorrectRecipientLen.Error())
}

func (s *Erc1155HandlerTestSuite) Test_HandleErc1155Message_InvalidTransferData() {
	message := &message.Message{
		Source:      1,
		Destination: 0,
		Data: transfer.TransferMessageData{
			DepositNonce: 1,
			ResourceId:   [32]byte{0},
			Payload: []interface{}{
				[]*big.Int{
					big.NewInt(2),
				},
				[]*big.Int{
					big.NewInt(3),
				},
				[]byte{28, 58, 3, 208, 76, 2, 107, 31, 75, 66, 8, 210, 206, 5, 60, 86, 134, 230, 251, 141},
				"invalidTransferData",
			},
			Type: transfer.SemiFungibleTransfer,
		},
		Type: transfer.TransferMessageType,
	}

	mh := executor.TransferMessageHandler{}
	prop, err := mh.HandleMessage(message)

	s.Nil(prop)
	s.EqualError(err, errIncorrectTransferData.Error())
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
		big.NewInt(5),
		s.msgChan)
}

func (s *RetryMessageHandlerTestSuite) Test_HandleMessage_RetryTooNew() {
	s.mockBlockFetcher.EXPECT().LatestBlock().Return(big.NewInt(105), nil)

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
	s.mockBlockFetcher.EXPECT().LatestBlock().Return(big.NewInt(106), nil)
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
	s.mockBlockFetcher.EXPECT().LatestBlock().Return(big.NewInt(106), nil)

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
