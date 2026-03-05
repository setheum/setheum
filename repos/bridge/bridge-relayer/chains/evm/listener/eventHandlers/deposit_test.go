// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package eventHandlers_test

import (
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/eventHandlers"
	mock_listener "github.com/ChainSafe/sygma-relayer/chains/evm/listener/eventHandlers/mock"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type DepositHandlerTestSuite struct {
	suite.Suite
	depositEventHandler *eventHandlers.DepositEventHandler
	mockDepositHandler  *mock_listener.MockDepositHandler
	mockEventListener   *mock_listener.MockEventListener
	domainID            uint8
	msgChan             chan []*message.Message
}

func TestRunDepositHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(DepositHandlerTestSuite))
}

func (s *DepositHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.domainID = 1
	s.mockEventListener = mock_listener.NewMockEventListener(ctrl)
	s.mockDepositHandler = mock_listener.NewMockDepositHandler(ctrl)
	s.msgChan = make(chan []*message.Message, 2)
	s.depositEventHandler = eventHandlers.NewDepositEventHandler(s.mockEventListener, s.mockDepositHandler, common.Address{}, s.domainID, s.msgChan)
}

func (s *DepositHandlerTestSuite) Test_FetchDepositFails() {
	s.mockEventListener.EXPECT().FetchDeposits(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return([]*events.Deposit{}, fmt.Errorf("error"))

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))

	s.NotNil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *DepositHandlerTestSuite) Test_HandleDepositFails_ExecutionContinue() {
	timestamp := time.Now()
	d1 := &events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
		Timestamp:           timestamp,
	}
	d2 := &events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
		Timestamp:           timestamp,
	}
	deposits := []*events.Deposit{d1, d2}
	s.mockEventListener.EXPECT().FetchDeposits(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(deposits, nil)
	msgID := fmt.Sprintf("%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1.DestinationDomainID,
		d1.DepositNonce,
		d1.ResourceID,
		d1.Data,
		d1.HandlerResponse,
		msgID,
		d1.Timestamp,
	).Return(&message.Message{}, fmt.Errorf("error"))
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2.DestinationDomainID,
		d2.DepositNonce,
		d2.ResourceID,
		d2.Data,
		d2.HandlerResponse,
		msgID,
		d2.Timestamp,
	).Return(
		&message.Message{
			Data: transfer.TransferMessageData{
				DepositNonce: 2,
			},
		},
		nil,
	)

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

func (s *DepositHandlerTestSuite) Test_HandleDepositPanis_ExecutionContinues() {
	d1 := &events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	d2 := &events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	deposits := []*events.Deposit{d1, d2}
	s.mockEventListener.EXPECT().FetchDeposits(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(deposits, nil)
	msgID := fmt.Sprintf("%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1.DestinationDomainID,
		d1.DepositNonce,
		d1.ResourceID,
		d1.Data,
		d1.HandlerResponse,
		msgID,
		gomock.Any(),
	).Do(func(sourceID, destID, nonce, resourceID, calldata, handlerResponse, msgID, timestamp interface{}) {
		panic("error")
	})
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2.DestinationDomainID,
		d2.DepositNonce,
		d2.ResourceID,
		d2.Data,
		d2.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

func (s *DepositHandlerTestSuite) Test_SuccessfulHandleDeposit() {
	d1 := &events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	d2 := &events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	deposits := []*events.Deposit{d1, d2}
	s.mockEventListener.EXPECT().FetchDeposits(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return(deposits, nil)
	msgID := fmt.Sprintf("%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1.DestinationDomainID,
		d1.DepositNonce,
		d1.ResourceID,
		d1.Data,
		d1.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 1}},
		nil,
	)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2.DestinationDomainID,
		d2.DepositNonce,
		d2.ResourceID,
		d2.Data,
		d2.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(
		&message.Message{Data: transfer.TransferMessageData{DepositNonce: 2}},
		nil,
	)

	err := s.depositEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 1}}, {Data: transfer.TransferMessageData{DepositNonce: 2}}})
}
