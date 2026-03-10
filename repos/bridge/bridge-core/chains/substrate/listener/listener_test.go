package listener_test

import (
	"context"
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/stretchr/testify/suite"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/listener"
	"github.com/sygmaprotocol/sygma-core/mock"
	"go.uber.org/mock/gomock"
)

type ListenerTestSuite struct {
	suite.Suite
	listener            *listener.SubstrateListener
	mockClient          *mock.MockChainConnection
	mockEventHandler    *mock.MockEventHandler
	mockBlockStorer     *mock.MockBlockStorer
	mockBlockDeltaMeter *mock.MockBlockDeltaMeter
	domainID            uint8
}

func TestRunTestSuite(t *testing.T) {
	suite.Run(t, new(ListenerTestSuite))
}

func (s *ListenerTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.domainID = 1
	s.mockClient = mock.NewMockChainConnection(ctrl)
	s.mockEventHandler = mock.NewMockEventHandler(ctrl)
	s.mockBlockStorer = mock.NewMockBlockStorer(ctrl)
	s.mockBlockDeltaMeter = mock.NewMockBlockDeltaMeter(ctrl)
	s.listener = listener.NewSubstrateListener(
		s.mockClient,
		[]listener.EventHandler{s.mockEventHandler, s.mockEventHandler},
		s.mockBlockStorer,
		s.mockBlockDeltaMeter,
		s.domainID,
		time.Millisecond*75,
		big.NewInt(5),
	)
}

func (s *ListenerTestSuite) Test_ListenToEvents_RetriesIfFinalizedHeadUnavailable() {
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, fmt.Errorf("error"))

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_RetriesIfBlockUnavailable() {
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(nil, fmt.Errorf("error"))

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_SleepsIfBlockTooNew() {
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: 104,
			},
		},
	}, nil)

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_RetriesInCaseOfHandlerFailure() {
	startBlock := big.NewInt(100)
	endBlock := big.NewInt(105)
	head := big.NewInt(110)

	// First pass
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: types.BlockNumber(head.Int64()),
			},
		},
	}, nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(fmt.Errorf("error"))
	// Second pass
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: types.BlockNumber(head.Int64()),
			},
		},
	}, nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(nil)
	// third pass
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: 100,
			},
		},
	}, nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_IgnoresBlockStorerError() {
	startBlock := big.NewInt(100)
	endBlock := big.NewInt(105)
	head := big.NewInt(110)

	// First pass
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: types.BlockNumber(head.Int64()),
			},
		},
	}, nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(fmt.Errorf("error"))
	// second pass
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: 95,
			},
		},
	}, nil)

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_UsesHeadAsStartBlockIfNilPassed() {
	startBlock := big.NewInt(110)
	endBlock := big.NewInt(115)
	oldHead := big.NewInt(110)
	newHead := big.NewInt(120)

	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: types.BlockNumber(oldHead.Int64()),
			},
		},
	}, nil)
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: types.BlockNumber(newHead.Int64()),
			},
		},
	}, nil)
	s.mockClient.EXPECT().GetFinalizedHead().Return(types.Hash{}, nil)
	s.mockClient.EXPECT().GetBlock(gomock.Any()).Return(&types.SignedBlock{
		Block: types.Block{
			Header: types.Header{
				Number: types.BlockNumber(95),
			},
		},
	}, nil)

	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), big.NewInt(120), endBlock)

	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, nil)

	time.Sleep(time.Millisecond * 100)
	cancel()
}
