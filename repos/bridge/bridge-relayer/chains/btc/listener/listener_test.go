package listener_test

import (
	"context"
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/btc/config"
	"github.com/ChainSafe/sygma-relayer/chains/btc/listener"
	mock_listener "github.com/ChainSafe/sygma-relayer/chains/btc/listener/mock"
	"github.com/ChainSafe/sygma-relayer/config/chain"
	"github.com/btcsuite/btcd/btcjson"
	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"
)

type ListenerTestSuite struct {
	suite.Suite
	listener         *listener.BtcListener
	mockConn         *mock_listener.MockConnection
	mockEventHandler *mock_listener.MockEventHandler
	mockBlockStorer  *mock_listener.MockBlockStorer
	domainID         uint8
}

func TestRunTestSuite(t *testing.T) {
	suite.Run(t, new(ListenerTestSuite))
}

func (s *ListenerTestSuite) SetupTest() {
	s.domainID = 1
	btcConfig := config.BtcConfig{
		GeneralChainConfig: chain.GeneralChainConfig{
			Id: &s.domainID,
		},
		BlockRetryInterval: time.Millisecond * 75,
		BlockConfirmations: big.NewInt(5),
	}

	ctrl := gomock.NewController(s.T())
	s.mockBlockStorer = mock_listener.NewMockBlockStorer(ctrl)

	s.mockConn = mock_listener.NewMockConnection(ctrl)
	s.mockEventHandler = mock_listener.NewMockEventHandler(ctrl)

	s.listener = listener.NewBtcListener(
		s.mockConn,
		[]listener.EventHandler{s.mockEventHandler, s.mockEventHandler},
		&btcConfig,
		s.mockBlockStorer,
	)
}

func (s *ListenerTestSuite) Test_ListenToEvents_RetriesIfFinalizedHeadUnavailable() {
	s.mockConn.EXPECT().GetBestBlockHash().Return(nil, fmt.Errorf("error"))

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()

}

func (s *ListenerTestSuite) Test_ListenToEvents_GetVerboseTxError() {
	hash, _ := chainhash.NewHashFromStr("00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc")
	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(nil, fmt.Errorf("error"))

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_SleepsIfBlockTooNew() {
	hash, _ := chainhash.NewHashFromStr("00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc")
	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: int64(102)}, nil)

	ctx, cancel := context.WithCancel(context.Background())
	go s.listener.ListenToEvents(ctx, big.NewInt(100))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_RetriesInCaseOfHandlerFailure() {
	startBlock := big.NewInt(105)
	head := int64(110)

	// First pass
	hash, _ := chainhash.NewHashFromStr("00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc")
	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: head}, nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock).Return(fmt.Errorf("error"))
	// Second pass
	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: head}, nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock).Return(nil)

	s.mockBlockStorer.EXPECT().StoreBlock(startBlock, s.domainID).Return(nil)
	// third pass
	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: head}, nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, big.NewInt(105))

	time.Sleep(time.Millisecond * 50)
	cancel()
}

func (s *ListenerTestSuite) Test_ListenToEvents_UsesHeadAsStartBlockIfNilPassed() {
	startBlock := big.NewInt(100)
	oldHead := int64(100)
	newHead := int64(106)
	hash, _ := chainhash.NewHashFromStr("00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc")
	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: oldHead}, nil)

	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: newHead}, nil)

	s.mockConn.EXPECT().GetBestBlockHash().Return(hash, nil)
	s.mockConn.EXPECT().GetBlockVerboseTx(hash).Return(&btcjson.GetBlockVerboseTxResult{Height: int64(50)}, nil)

	s.mockEventHandler.EXPECT().HandleEvents(startBlock).Return(nil)
	s.mockEventHandler.EXPECT().HandleEvents(startBlock).Return(nil)

	s.mockBlockStorer.EXPECT().StoreBlock(startBlock, s.domainID).Return(nil)

	ctx, cancel := context.WithCancel(context.Background())

	go s.listener.ListenToEvents(ctx, nil)

	time.Sleep(time.Millisecond * 100)
	cancel()
}
