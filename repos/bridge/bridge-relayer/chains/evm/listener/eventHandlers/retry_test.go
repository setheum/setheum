// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package eventHandlers_test

import (
	"fmt"
	"math/big"
	"testing"

	"github.com/rs/zerolog/log"

	"github.com/ethereum/go-ethereum/common"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/eventHandlers"
	mock_listener "github.com/ChainSafe/sygma-relayer/chains/evm/listener/eventHandlers/mock"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type RetryV2EventHandlerTestSuite struct {
	suite.Suite
	retryEventHandler *eventHandlers.RetryV2EventHandler
	mockEventListener *mock_listener.MockEventListener
	domainID          uint8
	msgChan           chan []*message.Message
}

func TestRunRetryEventHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(RetryV2EventHandlerTestSuite))
}

func (s *RetryV2EventHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.domainID = 1
	s.mockEventListener = mock_listener.NewMockEventListener(ctrl)
	s.msgChan = make(chan []*message.Message, 1)
	s.retryEventHandler = eventHandlers.NewRetryV2EventHandler(
		log.With(),
		s.mockEventListener,
		common.Address{},
		s.domainID,
		s.msgChan)
}

func (s *RetryV2EventHandlerTestSuite) Test_FetchRetryEventsFails() {
	s.mockEventListener.EXPECT().FetchRetryV2Events(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return([]events.RetryV2Event{}, fmt.Errorf("error"))

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))

	s.NotNil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryV2EventHandlerTestSuite) Test_FetchRetryEvents_ValidRetry() {
	s.mockEventListener.EXPECT().FetchRetryV2Events(
		gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(),
	).Return([]events.RetryV2Event{
		{SourceDomainID: 2, DestinationDomainID: 3, ResourceID: [32]byte{1}, BlockHeight: big.NewInt(100)},
		{SourceDomainID: 3, DestinationDomainID: 4, ResourceID: [32]byte{2}, BlockHeight: big.NewInt(101)},
	}, nil)

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs2 := <-s.msgChan
	msgs1 := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs1[0].Data, retry.RetryMessageData{
		SourceDomainID: 2, DestinationDomainID: 3, ResourceID: [32]byte{1}, BlockHeight: big.NewInt(100),
	})
	s.Equal(msgs1[0].Destination, uint8(2))
	s.Equal(msgs1[0].Source, s.domainID)

	s.Equal(msgs2[0].Data, retry.RetryMessageData{
		SourceDomainID: 3, DestinationDomainID: 4, ResourceID: [32]byte{2}, BlockHeight: big.NewInt(101),
	})
	s.Equal(msgs2[0].Destination, uint8(3))
	s.Equal(msgs2[0].Source, s.domainID)
}

type RetryV1EventHandlerTestSuite struct {
	suite.Suite
	retryEventHandler  *eventHandlers.RetryV1EventHandler
	mockDepositHandler *mock_listener.MockDepositHandler
	mockPropStorer     *mock_listener.MockPropStorer
	mockEventListener  *mock_listener.MockEventListener
	domainID           uint8
	msgChan            chan []*message.Message
}

func TestRunRetryV1EventHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(RetryV1EventHandlerTestSuite))
}

func (s *RetryV1EventHandlerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.domainID = 1
	s.mockEventListener = mock_listener.NewMockEventListener(ctrl)
	s.mockDepositHandler = mock_listener.NewMockDepositHandler(ctrl)
	s.mockPropStorer = mock_listener.NewMockPropStorer(ctrl)
	s.msgChan = make(chan []*message.Message, 1)
	s.retryEventHandler = eventHandlers.NewRetryV1EventHandler(
		log.With(),
		s.mockEventListener,
		s.mockDepositHandler,
		s.mockPropStorer,
		common.Address{},
		s.domainID,
		big.NewInt(5),
		s.msgChan)
}

func (s *RetryV1EventHandlerTestSuite) Test_FetchDepositFails() {
	s.mockEventListener.EXPECT().FetchRetryV1Events(gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any()).Return([]events.RetryV1Event{}, fmt.Errorf("error"))

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))

	s.NotNil(err)
	s.Equal(len(s.msgChan), 0)
}

func (s *RetryV1EventHandlerTestSuite) Test_FetchDepositFails_ExecutionContinues() {
	d := events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockEventListener.EXPECT().FetchRetryV1Events(
		gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(),
	).Return([]events.RetryV1Event{{TxHash: "event1"}, {TxHash: "event2"}}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event1"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{}, fmt.Errorf("error"))
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event2"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d}, nil)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d.DestinationDomainID,
		d.DepositNonce,
		d.ResourceID,
		d.Data,
		d.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{
		Data: transfer.TransferMessageData{
			DepositNonce: 2,
		},
	}, nil)
	s.mockPropStorer.EXPECT().PropStatus(gomock.Any(), gomock.Any(), gomock.Any()).Return(store.MissingProp, nil)

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}})
}

func (s *RetryV1EventHandlerTestSuite) Test_HandleDepositFails_ExecutionContinues() {
	d1 := events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	d2 := events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockEventListener.EXPECT().FetchRetryV1Events(
		gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(),
	).Return([]events.RetryV1Event{{TxHash: "event1"}, {TxHash: "event2"}}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event1"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d1}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event2"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d2}, nil)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1.DestinationDomainID,
		d1.DepositNonce,
		d1.ResourceID,
		d1.Data,
		d1.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 1,
	}}, fmt.Errorf("error"))
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2.DestinationDomainID,
		d2.DepositNonce,
		d2.ResourceID,
		d2.Data,
		d2.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}, nil)
	s.mockPropStorer.EXPECT().PropStatus(gomock.Any(), gomock.Any(), gomock.Any()).Return(store.MissingProp, nil)

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{DepositNonce: 2}}})
}

func (s *RetryV1EventHandlerTestSuite) Test_HandlingRetryPanics_ExecutionContinue() {
	d1 := events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	d2 := events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	s.mockEventListener.EXPECT().FetchRetryV1Events(
		gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(),
	).Return([]events.RetryV1Event{{TxHash: "event1"}, {TxHash: "event2"}}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event1"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d1}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event2"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d2}, nil)
	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 5)
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
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}, nil)
	s.mockPropStorer.EXPECT().PropStatus(gomock.Any(), gomock.Any(), gomock.Any()).Return(store.MissingProp, nil)

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}})
}

func (s *RetryV1EventHandlerTestSuite) Test_MultipleDeposits() {
	d1 := events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	d2 := events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	s.mockEventListener.EXPECT().FetchRetryV1Events(
		gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(),
	).Return([]events.RetryV1Event{{TxHash: "event1"}}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event1"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d1, d2}, nil)
	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1.DestinationDomainID,
		d1.DepositNonce,
		d1.ResourceID,
		d1.Data,
		d1.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 1,
	}}, nil)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2.DestinationDomainID,
		d2.DepositNonce,
		d2.ResourceID,
		d2.Data,
		d2.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}, nil)
	s.mockPropStorer.EXPECT().PropStatus(gomock.Any(), gomock.Any(), gomock.Any()).Return(store.MissingProp, nil).Times(2)

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{{Data: transfer.TransferMessageData{
		DepositNonce: 1,
	}}, {Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}})
}

func (s *RetryV1EventHandlerTestSuite) Test_MultipleDeposits_ExecutedIgnored() {
	d1 := events.Deposit{
		DepositNonce:        1,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	d2 := events.Deposit{
		DepositNonce:        2,
		DestinationDomainID: 2,
		ResourceID:          [32]byte{},
		HandlerResponse:     []byte{},
		Data:                []byte{},
	}
	s.mockEventListener.EXPECT().FetchRetryV1Events(
		gomock.Any(), gomock.Any(), gomock.Any(), gomock.Any(),
	).Return([]events.RetryV1Event{{TxHash: "event1"}}, nil)
	s.mockEventListener.EXPECT().FetchRetryDepositEvents(events.RetryV1Event{TxHash: "event1"}, gomock.Any(), big.NewInt(5)).Return([]events.Deposit{d1, d2}, nil)
	msgID := fmt.Sprintf("retry-%d-%d-%d-%d", 1, 2, 0, 5)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d1.DestinationDomainID,
		d1.DepositNonce,
		d1.ResourceID,
		d1.Data,
		d1.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 1,
	}}, nil)
	s.mockPropStorer.EXPECT().PropStatus(gomock.Any(), gomock.Any(), gomock.Any()).Return(store.ExecutedProp, nil)
	s.mockDepositHandler.EXPECT().HandleDeposit(
		s.domainID,
		d2.DestinationDomainID,
		d2.DepositNonce,
		d2.ResourceID,
		d2.Data,
		d2.HandlerResponse,
		msgID,
		gomock.Any(),
	).Return(&message.Message{Data: transfer.TransferMessageData{
		DepositNonce: 2,
	}}, nil)
	s.mockPropStorer.EXPECT().PropStatus(gomock.Any(), gomock.Any(), gomock.Any()).Return(store.PendingProp, nil)
	s.mockPropStorer.EXPECT().StorePropStatus(gomock.Any(), gomock.Any(), gomock.Any(), store.FailedProp).Return(nil)

	err := s.retryEventHandler.HandleEvents(big.NewInt(0), big.NewInt(5))
	msgs := <-s.msgChan

	s.Nil(err)
	s.Equal(msgs, []*message.Message{
		{
			Data: transfer.TransferMessageData{
				DepositNonce: 2,
			},
		},
	})
}
