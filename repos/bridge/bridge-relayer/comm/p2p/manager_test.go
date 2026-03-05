// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p_test

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/comm/p2p"
	"github.com/libp2p/go-libp2p/core/peer"

	mock_network "github.com/ChainSafe/sygma-relayer/comm/p2p/mock/stream"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"
)

type StreamManagerTestSuite struct {
	suite.Suite
	mockController *gomock.Controller
}

func TestRunStreamManagerTestSuite(t *testing.T) {
	suite.Run(t, new(StreamManagerTestSuite))
}

func (s *StreamManagerTestSuite) SetupTest() {
	s.mockController = gomock.NewController(s.T())
}

func (s *StreamManagerTestSuite) Test_ManagingSubscriptions_Success() {
	streamManager := p2p.NewStreamManager()

	stream1 := mock_network.NewMockStream(s.mockController)
	stream2 := mock_network.NewMockStream(s.mockController)
	stream3 := mock_network.NewMockStream(s.mockController)

	peerID1, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	peerID2, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")

	streamManager.AddStream("1", peerID1, stream1)
	streamManager.AddStream("1", peerID1, stream1)
	streamManager.AddStream("1", peerID2, stream2)
	streamManager.AddStream("2", peerID1, stream3)

	stream1.EXPECT().Close().Times(1).Return(nil)
	stream2.EXPECT().Close().Times(1).Return(nil)

	streamManager.ReleaseStreams("1")
}

func (s *StreamManagerTestSuite) Test_FetchStream_NoStream() {
	streamManager := p2p.NewStreamManager()

	_, err := streamManager.Stream("1", peer.ID(""))

	s.NotNil(err)
}

func (s *StreamManagerTestSuite) Test_FetchStream_ValidStream() {
	streamManager := p2p.NewStreamManager()

	stream := mock_network.NewMockStream(s.mockController)
	peerID1, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	streamManager.AddStream("1", peerID1, stream)

	expectedStream, err := streamManager.Stream("1", peerID1)

	s.Nil(err)
	s.Equal(stream, expectedStream)
}

func (s *StreamManagerTestSuite) Test_AddStream_IgnoresExistingPeer() {
	streamManager := p2p.NewStreamManager()

	stream1 := mock_network.NewMockStream(s.mockController)
	stream2 := mock_network.NewMockStream(s.mockController)
	peerID1, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	streamManager.AddStream("1", peerID1, stream1)
	streamManager.AddStream("1", peerID1, stream2)

	expectedStream, err := streamManager.Stream("1", peerID1)

	s.Nil(err)
	s.Equal(stream1, expectedStream)
}
