// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener_test

import (
	"fmt"
	"math/big"

	"github.com/ChainSafe/sygma-relayer/chains/substrate/listener"
	mock_events "github.com/ChainSafe/sygma-relayer/chains/substrate/listener/mock"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/rs/zerolog"
	"github.com/sygmaprotocol/sygma-core/relayer/message"

	"testing"

	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types/block"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"
)

type SystemUpdateHandlerTestSuite struct {
	suite.Suite
	mockConn            *mock_events.MockConnection
	systemUpdateHandler *listener.SystemUpdateEventHandler
}

func TestRunSystemUpdateHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(SystemUpdateHandlerTestSuite))
}

func (s *SystemUpdateHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.mockConn = mock_events.NewMockConnection(ctrl)
	s.systemUpdateHandler = listener.NewSystemUpdateEventHandler(s.mockConn)
}

func (s *SystemUpdateHandlerTestSuite) Test_UpdateMetadataFails() {
	s.mockConn.EXPECT().UpdateMetatdata().Return(fmt.Errorf("error"))
	evts := []*parser.Event{
		{
			Name: "ParachainSystem.ValidationFunctionApplied",
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.systemUpdateHandler.HandleEvents(big.NewInt(0), big.NewInt(1))

	s.NotNil(err)
}

func (s *SystemUpdateHandlerTestSuite) Test_NoMetadataUpdate() {

	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return([]*parser.Event{}, nil)
	err := s.systemUpdateHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	s.Nil(err)
}

func (s *SystemUpdateHandlerTestSuite) Test_SuccesfullMetadataUpdate() {
	s.mockConn.EXPECT().UpdateMetatdata().Return(nil)
	evts := []*parser.Event{
		{
			Name: "ParachainSystem.ValidationFunctionApplied",
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.systemUpdateHandler.HandleEvents(big.NewInt(0), big.NewInt(1))

	s.Nil(err)
}

type DepositHandlerTestSuite struct {
	suite.Suite
	depositEventHandler *listener.FungibleTransferEventHandler
	mockDepositHandler  *mock_events.MockDepositHandler
	domainID            uint8
	msgChan             chan []*message.Message
	mockConn            *mock_events.MockConnection
}

func TestRunDepositHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(DepositHandlerTestSuite))
}

func (s *DepositHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.domainID = 1
	s.mockDepositHandler = mock_events.NewMockDepositHandler(ctrl)
	s.msgChan = make(chan []*message.Message, 2)
	s.mockConn = mock_events.NewMockConnection(ctrl)
	s.depositEventHandler = listener.NewFungibleTransferEventHandler(zerolog.Context{}, s.domainID, s.mockDepositHandler, s.msgChan, s.mockConn)
}

func (s *DepositHandlerTestSuite) Test_HandleDepositFails_ExecutionContinue() {
	d1 := map[string]any{
		"dest_domain_id":            types.NewU8(2),
		"resource_id":               types.Bytes32{1},
		"deposit_nonce":             types.NewU64(1),
		"sygma_traits_TransferType": types.NewU8(0),
		"deposit_data":              []byte{},
		"handler_response":          [1]byte{0},
	}
	d2 := map[string]any{
		"dest_domain_id":            types.NewU8(2),
		"resource_id":               types.Bytes32{1},
		"deposit_nonce":             types.NewU64(2),
		"sygma_traits_TransferType": types.NewU8(0),
		"deposit_data":              []byte{},
		"handler_response":          [1]byte{0},
	}
	msgID := fmt.Sprintf("%d-%d-%d-%d", 1, 2, 0, 1)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1["dest_domain_id"],
		d1["deposit_nonce"],
		d1["resource_id"],
		d1["deposit_data"],
		d1["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(&message.Message{}, fmt.Errorf("error"))
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2["dest_domain_id"],
		d2["deposit_nonce"],
		d2["resource_id"],
		d2["deposit_data"],
		d2["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d1["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d1["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d1["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d1["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d1["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d1["handler_response"]},
			},
		},
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d2["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d2["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d2["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d2["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d2["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d2["handler_response"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

func (s *DepositHandlerTestSuite) Test_SuccessfulHandleDeposit() {
	d1 := map[string]any{
		"dest_domain_id":            types.NewU8(2),
		"deposit_nonce":             types.NewU64(1),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	d2 := map[string]any{
		"deposit_nonce":             types.NewU64(2),
		"dest_domain_id":            types.NewU8(2),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	msgID := fmt.Sprintf("%d-%d-%d-%d", 1, 2, 0, 1)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1["dest_domain_id"],
		d1["deposit_nonce"],
		d1["resource_id"],
		d1["deposit_data"],
		d1["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 1}},
		nil,
	)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2["dest_domain_id"],
		d2["deposit_nonce"],
		d2["resource_id"],
		d2["deposit_data"],
		d2["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d1["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d1["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d1["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d1["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d1["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d1["handler_response"]},
			},
		},
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d2["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d2["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d2["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d2["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d2["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d2["handler_response"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 1}}, {Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

func (s *DepositHandlerTestSuite) Test_HandleDepositPanics_ExecutionContinues() {
	d1 := map[string]any{
		"dest_domain_id":            types.NewU8(2),
		"deposit_nonce":             types.NewU64(1),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	d2 := map[string]any{
		"deposit_nonce":             types.NewU64(2),
		"dest_domain_id":            types.NewU8(2),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	msgID := fmt.Sprintf("%d-%d-%d-%d", 1, 2, 0, 1)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1["dest_domain_id"],
		d1["deposit_nonce"],
		d1["resource_id"],
		d1["deposit_data"],
		d1["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Do(func(sourceID, destID, nonce, resourceID, calldata, depositType, msgID, timestamp interface{}) {
		panic("error")
	})
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2["dest_domain_id"],
		d2["deposit_nonce"],
		d2["resource_id"],
		d2["deposit_data"],
		d2["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d1["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d1["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d1["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d1["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d1["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d1["handler_response"]},
			},
		},
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d2["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d2["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d2["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d2["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d2["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d2["handler_response"]},
			},
		},
	}

	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

type RetryHandlerTestSuite struct {
	suite.Suite
	retryHandler       *listener.RetryEventHandler
	mockDepositHandler *mock_events.MockDepositHandler
	mockConn           *mock_events.MockConnection
	domainID           uint8
	msgChan            chan []*message.Message
}

func TestRunRetryHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(RetryHandlerTestSuite))
}

func (s *RetryHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.domainID = 1
	s.mockDepositHandler = mock_events.NewMockDepositHandler(ctrl)
	s.mockConn = mock_events.NewMockConnection(ctrl)
	s.msgChan = make(chan []*message.Message, 2)
	s.retryHandler = listener.NewRetryEventHandler(zerolog.Context{}, s.mockConn, s.mockDepositHandler, s.domainID, s.msgChan)

}

func (s *RetryHandlerTestSuite) Test_CannotFetchLatestBlock() {

	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return([]*parser.Event{}, fmt.Errorf("error"))

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))

	s.NotNil(err)
}

func (s *RetryHandlerTestSuite) Test_EventTooNew() {
	s.mockConn.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{Block: block.Block{
		Header: types.Header{
			Number: types.BlockNumber(uint32(100)),
		},
	}}, nil)

	rtry := map[string]any{
		"deposit_on_block_height": types.NewU128(*big.NewInt(101)),
	}
	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	s.Nil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryHandlerTestSuite) Test_FetchingBlockHashFails() {
	s.mockConn.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{Block: block.Block{
		Header: types.Header{
			Number: types.BlockNumber(uint32(100)),
		},
	}}, nil)

	s.mockConn.EXPECT().GetBlockHash(uint64(95)).Return(types.Hash{}, fmt.Errorf("error"))

	rtry := map[string]any{
		"deposit_on_block_height": types.NewU128(*big.NewInt(95)),
	}
	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))

	s.NotNil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryHandlerTestSuite) Test_FetchingBlockEventsFails() {
	s.mockConn.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{Block: block.Block{
		Header: types.Header{
			Number: types.BlockNumber(uint32(100)),
		},
	}}, nil)
	s.mockConn.EXPECT().GetBlockHash(uint64(95)).Return(types.Hash{}, nil)

	rtry := map[string]any{
		"deposit_on_block_height": types.NewU128(*big.NewInt(95)),
	}
	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)
	s.mockConn.EXPECT().GetBlockEvents(gomock.Any()).Return(nil, fmt.Errorf("error"))

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))

	s.NotNil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryHandlerTestSuite) Test_NoEvents() {
	s.mockConn.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{Block: block.Block{
		Header: types.Header{
			Number: types.BlockNumber(uint32(100)),
		},
	}}, nil)
	s.mockConn.EXPECT().GetBlockHash(uint64(95)).Return(types.Hash{}, nil)

	rtry := map[string]any{
		"deposit_on_block_height": types.NewU128(*big.NewInt(95)),
	}
	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)
	s.mockConn.EXPECT().GetBlockEvents(gomock.Any()).Return([]*parser.Event{}, nil)

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))

	s.Nil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryHandlerTestSuite) Test_ValidEvents() {
	s.mockConn.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{Block: block.Block{
		Header: types.Header{
			Number: types.BlockNumber(uint32(100)),
		},
	}}, nil)
	s.mockConn.EXPECT().GetBlockHash(uint64(95)).Return(types.Hash{}, nil)
	d1 := map[string]any{
		"dest_domain_id":            types.NewU8(2),
		"deposit_nonce":             types.NewU64(1),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	d2 := map[string]any{
		"deposit_nonce":             types.NewU64(2),
		"dest_domain_id":            types.NewU8(2),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	blockEvts := []*parser.Event{
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d1["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d1["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d1["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d1["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d1["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d1["handler_response"]},
			},
		},
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d2["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d2["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d2["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d2["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d2["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d2["handler_response"]},
			},
		},
	}
	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 1)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1["dest_domain_id"],
		d1["deposit_nonce"],
		d1["resource_id"],
		d1["deposit_data"],
		d1["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 1}},
		nil,
	)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2["dest_domain_id"],
		d2["deposit_nonce"],
		d2["resource_id"],
		d2["deposit_data"],
		d2["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	rtry := map[string]any{
		"deposit_on_block_height": types.NewU128(*big.NewInt(95)),
	}
	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)
	s.mockConn.EXPECT().GetBlockEvents(gomock.Any()).Return(blockEvts, nil)

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(len(s.msgChan), 0)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 1}}, {Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

func (s *RetryHandlerTestSuite) Test_EventPanics() {
	s.mockConn.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlock(gomock.Any()).Return(&block.SignedBlock{Block: block.Block{
		Header: types.Header{
			Number: types.BlockNumber(uint32(100)),
		},
	}}, nil)
	s.mockConn.EXPECT().GetBlockHash(uint64(95)).Return(types.Hash{}, nil)
	s.mockConn.EXPECT().GetBlockHash(uint64(95)).Return(types.Hash{}, nil)
	d1 := map[string]any{
		"dest_domain_id":            types.NewU8(2),
		"deposit_nonce":             types.NewU64(1),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}
	d2 := map[string]any{
		"deposit_nonce":             types.NewU64(2),
		"dest_domain_id":            types.NewU8(2),
		"resource_id":               types.Bytes32{1},
		"sygma_traits_TransferType": types.NewU8(0),
		"handler_response":          [1]byte{0},
		"deposit_data":              []byte{},
	}

	blockEvts1 := []*parser.Event{
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d1["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d1["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d1["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d1["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d1["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d1["handler_response"]},
			},
		},
	}
	blockEvts2 := []*parser.Event{
		{
			Name: "SygmaBridge.Deposit",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "dest_domain_id", Value: d2["dest_domain_id"]},
				&registry.DecodedField{Name: "resource_id", Value: d2["resource_id"]},
				&registry.DecodedField{Name: "deposit_nonce", Value: d2["deposit_nonce"]},
				&registry.DecodedField{Name: "sygma_traits_TransferType", Value: d2["sygma_traits_TransferType"]},
				&registry.DecodedField{Name: "deposit_data", Value: d2["deposit_data"]},
				&registry.DecodedField{Name: "handler_response", Value: d2["handler_response"]},
			},
		},
	}

	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 1)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1["dest_domain_id"],
		d1["deposit_nonce"],
		d1["resource_id"],
		d1["deposit_data"],
		d1["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Do(func(sourceID, destID, nonce, resourceID, calldata, depositType, msgID, timestamp interface{}) {
		panic("error")
	})
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2["dest_domain_id"],
		d2["deposit_nonce"],
		d2["resource_id"],
		d2["deposit_data"],
		d2["sygma_traits_TransferType"],
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	rtry := map[string]any{
		"deposit_on_block_height": types.NewU128(*big.NewInt(95)),
	}
	evts := []*parser.Event{
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
		{
			Name: "SygmaBridge.Retry",
			Fields: registry.DecodedFields{
				&registry.DecodedField{Name: "deposit_on_block_height", Value: rtry["deposit_on_block_height"]},
			},
		},
	}
	s.mockConn.EXPECT().FetchEvents(gomock.Any(), gomock.Any()).Return(evts, nil)
	s.mockConn.EXPECT().GetBlockEvents(gomock.Any()).Return(blockEvts1, nil)
	s.mockConn.EXPECT().GetBlockEvents(gomock.Any()).Return(blockEvts2, nil)

	err := s.retryHandler.HandleEvents(big.NewInt(0), big.NewInt(1))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(len(s.msgChan), 0)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 2}}})
}
