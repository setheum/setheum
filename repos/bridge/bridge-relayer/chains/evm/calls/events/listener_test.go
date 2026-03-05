// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package events_test

import (
	"fmt"
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	mock_listener "github.com/ChainSafe/sygma-relayer/chains/evm/calls/events/mock"
)

type ListenerTestSuite struct {
	suite.Suite
	mockClient *mock_listener.MockChainClient
	listener   *events.Listener
}

func TestRunListenerTestSuite(t *testing.T) {
	suite.Run(t, new(ListenerTestSuite))
}

func (s *ListenerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.mockClient = mock_listener.NewMockChainClient(ctrl)
	s.listener = events.NewListener(s.mockClient)
}

func (s *ListenerTestSuite) Test_FetchRetryDepositEvents_FetchingTxFails() {
	s.mockClient.EXPECT().WaitAndReturnTxReceipt(common.HexToHash("0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c")).Return(nil, fmt.Errorf("error"))

	_, err := s.listener.FetchRetryDepositEvents(events.RetryV1Event{TxHash: "0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c"}, common.Address{}, big.NewInt(5))

	s.NotNil(err)
}

func (s *ListenerTestSuite) Test_FetchRetryDepositEvents_EventTooNew() {
	s.mockClient.EXPECT().WaitAndReturnTxReceipt(common.HexToHash("0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c")).Return(&types.Receipt{
		BlockNumber: big.NewInt(14),
	}, nil)
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(10), nil)

	_, err := s.listener.FetchRetryDepositEvents(
		events.RetryV1Event{TxHash: "0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c"},
		common.HexToAddress("0x5798e01f4b1d8f6a5d91167414f3a915d021bc4a"),
		big.NewInt(5),
	)

	s.NotNil(err)
}

func (s *ListenerTestSuite) Test_FetchRetryDepositEvents_NoDepositEvent() {
	s.mockClient.EXPECT().WaitAndReturnTxReceipt(common.HexToHash("0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c")).Return(&types.Receipt{
		BlockNumber: big.NewInt(14),
	}, nil)
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(20), nil)

	deposits, err := s.listener.FetchRetryDepositEvents(
		events.RetryV1Event{TxHash: "0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c"},
		common.HexToAddress("0x5798e01f4b1d8f6a5d91167414f3a915d021bc4a"),
		big.NewInt(5),
	)

	s.Nil(err)
	s.Equal(deposits, []events.Deposit{})
}

func (s *ListenerTestSuite) Test_FetchRetryDepositEvents_NoMatchingEvent() {
	s.mockClient.EXPECT().WaitAndReturnTxReceipt(common.HexToHash("0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c")).Return(&types.Receipt{
		BlockNumber: big.NewInt(14),
		Logs: []*types.Log{
			{
				Address: common.HexToAddress("0x1ec6b294902d42fee964d29fa962e5976e71e67d"),
				Data:    []byte{},
			},
			{
				Address: common.HexToAddress("0x5798e01f4b1d8f6a5d91167414f3a915d021bc4a"),
				Data:    []byte{},
			},
		},
	}, nil)
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(20), nil)

	deposits, err := s.listener.FetchRetryDepositEvents(
		events.RetryV1Event{TxHash: "0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c"},
		common.HexToAddress("0x5798e01f4b1d8f6a5d91167414f3a915d021bc4a"),
		big.NewInt(5),
	)

	s.Nil(err)
	s.Equal(deposits, []events.Deposit{})
}

func (s *ListenerTestSuite) Test_FetchRetryDepositEvents_ValidEvent() {
	depositEvent := common.Hex2Bytes("00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000001d00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000005600000000000000000000000000000000000000000000000000000000000f424000000000000000000000000000000000000000000000000000000000000000148e0a907331554af72563bd8d43051c2e64be5d350102000000000000000000000000000000000000000000000000000000000000000000000000000000000000")
	s.mockClient.EXPECT().WaitAndReturnTxReceipt(common.HexToHash("0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c")).Return(&types.Receipt{
		BlockNumber: big.NewInt(14),
		Logs: []*types.Log{
			{
				Address: common.HexToAddress("0x1ec6b294902d42fee964d29fa962e5976e71e67d"),
				Data:    depositEvent,
				Topics: []common.Hash{
					{},
					{},
				},
				BlockNumber: 14,
			},
			{
				Address: common.HexToAddress("0x5798e01f4b1d8f6a5d91167414f3a915d021bc4a"),
				Data:    depositEvent,
				Topics: []common.Hash{
					{},
					{},
				},
				BlockNumber: 14,
			},
		},
	}, nil)
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(20), nil)
	s.mockClient.EXPECT().BlockByNumber(gomock.Any(), big.NewInt(14)).Return(nil, fmt.Errorf("error"))

	deposits, err := s.listener.FetchRetryDepositEvents(
		events.RetryV1Event{TxHash: "0xf25ed4a14bf7ad20354b46fe38d7d4525f2ea3042db9a9954ef8d73c558b500c"},
		common.HexToAddress("0x5798e01f4b1d8f6a5d91167414f3a915d021bc4a"),
		big.NewInt(5),
	)

	s.Nil(err)
	s.Equal(deposits[0].DestinationDomainID, uint8(2))
}
