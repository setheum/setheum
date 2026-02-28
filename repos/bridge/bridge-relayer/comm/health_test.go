// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm_test

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/golang/mock/gomock"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/core/protocol"
	"github.com/stretchr/testify/suite"
)

type CommunicationHealthTestSuite struct {
	suite.Suite
	mockController     *gomock.Controller
	testHosts          []host.Host
	testCommunications []comm.Communication
	testProtocolID     protocol.ID
	testSessionID      string
}

func TestRunCommunicationHealthTestSuite(t *testing.T) {
	suite.Run(t, new(CommunicationHealthTestSuite))
}

func (s *CommunicationHealthTestSuite) SetupSuite() {
	s.testProtocolID = "test/health"
	s.testSessionID = "1"
}
func (s *CommunicationHealthTestSuite) TearDownSuite() {}

func (s *CommunicationHealthTestSuite) SetupTest() {
	s.mockController = gomock.NewController(s.T())

	hosts, communications := InitializeHostsAndCommunications(3, s.testProtocolID)

	s.testHosts = hosts
	s.testCommunications = communications
}

func (s *CommunicationHealthTestSuite) TearDownTest() {
	for _, testHost := range s.testHosts {
		_ = testHost.Close()
	}
}

func (s *CommunicationHealthTestSuite) TestCommHealth_AllPearsAvailable() {
	errors := comm.ExecuteCommHealthCheck(
		s.testCommunications[0], peer.IDSlice{s.testHosts[1].ID(), s.testHosts[2].ID()},
	)
	s.Empty(errors)
}

func (s *CommunicationHealthTestSuite) TestCommHealth_OnePeerOffline() {
	broadcastPeers := peer.IDSlice{s.testHosts[1].ID(), s.testHosts[2].ID()}
	// close one peer
	_ = s.testHosts[2].Close()
	errors := comm.ExecuteCommHealthCheck(
		s.testCommunications[0], broadcastPeers,
	)

	s.NotEmpty(errors)
	s.Equal(1, len(errors))
	s.Equal(broadcastPeers[1], errors[0].Peer)
	s.NotNil(errors[0].Err)
}

func (s *CommunicationHealthTestSuite) TestCommHealth_AllPeersOffline() {
	broadcastPeers := peer.IDSlice{s.testHosts[1].ID(), s.testHosts[2].ID()}
	// close other peers
	_ = s.testHosts[2].Close()
	_ = s.testHosts[1].Close()
	errors := comm.ExecuteCommHealthCheck(
		s.testCommunications[0], broadcastPeers,
	)

	s.NotEmpty(errors)
	s.Equal(2, len(errors))
}
