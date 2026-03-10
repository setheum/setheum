package listener_test

import (
	"context"
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/stretchr/testify/suite"
	"github.com/sygmaprotocol/sygma-core/chains/evm/listener"
	"github.com/sygmaprotocol/sygma-core/mock"
	"go.uber.org/mock/gomock"
)

type ListenerTestSuite struct {
	suite.Suite
	listener            *listener.EVMListener
	mockClient          *mock.MockChainClient
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
	s.mockClient = mock.NewMockChainClient(ctrl)
	s.mockEventHandler = mock.NewMockEventHandler(ctrl)
	s.mockBlockStorer = mock.NewMockBlockStorer(ctrl)
	s.mockBlockDeltaMeter = mock.NewMockBlockDeltaMeter(ctrl)
	s.listener = listener.NewEVMListener(
		s.mockClient,
		[]listener.EventHandler{s.mockEventHandler, s.mockEventHandler},
		s.mockBlockStorer,
		s.mockBlockDeltaMeter,
		s.domainID,
		time.Millisecond*75,
		big.NewInt(5),
		big.NewInt(5))
}

func (s *ListenerTestSuite) Test_ListenToEvents_RetriesIfBlockUnavailable() {
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(0), fmt.Errorf("error"))

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_SleepsIfBlockTooNew() {
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(109), nil)

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
	s.mockClient.EXPECT().LatestBlock().Return(head, nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(fmt.Errorf("error"))
	// Second pass
	s.mockClient.EXPECT().LatestBlock().Return(head, nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(nil)
	// third pass
	s.mockClient.EXPECT().LatestBlock().Return(head, nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_StoresBlockIfEventHandlingSuccessful() {
	startBlock := big.NewInt(100)
	endBlock := big.NewInt(105)
	head := big.NewInt(110)

	s.mockClient.EXPECT().LatestBlock().Return(head, nil)
	// prevent infinite runs
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(95), nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_IgnoresBlockStorerError() {
	startBlock := big.NewInt(100)
	endBlock := big.NewInt(105)
	head := big.NewInt(110)

	s.mockClient.EXPECT().LatestBlock().Return(head, nil)
	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), head, endBlock)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(fmt.Errorf("error"))

	// prevent infinite runs
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(95), nil)

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

	s.mockClient.EXPECT().LatestBlock().Return(oldHead, nil)
	s.mockClient.EXPECT().LatestBlock().Return(newHead, nil)
	s.mockClient.EXPECT().LatestBlock().Return(big.NewInt(65), nil)

	s.mockBlockDeltaMeter.EXPECT().TrackBlockDelta(uint8(1), big.NewInt(120), endBlock)

	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock, new(big.Int).Sub(endBlock, big.NewInt(1))).Return(nil)
	s.mockBlockStorer.EXPECT().StoreBlock(endBlock, s.domainID).Return(nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, nil)

	time.Sleep(time.Millisecond * 100)
	cancel()
}
