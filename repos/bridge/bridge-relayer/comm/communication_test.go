// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm_test

import (
	"fmt"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/p2p"
	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/golang/mock/gomock"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/core/peerstore"
	"github.com/libp2p/go-libp2p/core/protocol"
	"github.com/stretchr/testify/suite"
)

type CommunicationIntegrationTestSuite struct {
	suite.Suite
	mockController     *gomock.Controller
	testHosts          []host.Host
	testCommunications []comm.Communication
	testProtocolID     protocol.ID
	testSessionID      string
}

const numberOfTestHosts = 3

func TestRunCommunicationIntegrationTestSuite(t *testing.T) {
	suite.Run(t, new(CommunicationIntegrationTestSuite))
}

func (s *CommunicationIntegrationTestSuite) SetupSuite() {
	s.testProtocolID = "test/protocol"
	s.testSessionID = "1"
}
func (s *CommunicationIntegrationTestSuite) TearDownSuite() {}
func (s *CommunicationIntegrationTestSuite) SetupTest() {
	s.mockController = gomock.NewController(s.T())

	hosts, communications := InitializeHostsAndCommunications(numberOfTestHosts, s.testProtocolID)
	s.testHosts = hosts
	s.testCommunications = communications
}
func (s *CommunicationIntegrationTestSuite) TearDownTest() {
	for _, testHost := range s.testHosts {
		_ = testHost.Close()
	}
}

func (s *CommunicationIntegrationTestSuite) TestCommunication_BroadcastMessage_SubscribersGotMessage() {
	firstSubChannel := make(chan *comm.WrappedMessage)
	s.testCommunications[0].Subscribe(s.testSessionID, comm.CoordinatorPingMsg, firstSubChannel)

	go func() {
		msg := <-firstSubChannel
		s.Equal("1", msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()

	secondSubChannel := make(chan *comm.WrappedMessage)
	s.testCommunications[1].Subscribe(s.testSessionID, comm.CoordinatorPingMsg, secondSubChannel)

	go func() {
		msg := <-secondSubChannel
		s.Equal(s.testSessionID, msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()

	err := s.testCommunications[2].Broadcast(
		[]peer.ID{s.testHosts[0].ID(), s.testHosts[1].ID()},
		nil,
		comm.CoordinatorPingMsg,
		"1",
	)
	s.Nil(err)
}

func (s *CommunicationIntegrationTestSuite) TestCommunication_BroadcastMessage_ErrorOnSendingMessageToExternalHost() {
	privKeyForHost, _, _ := crypto.GenerateKeyPair(crypto.ECDSA, 1)
	topology := &topology.NetworkTopology{
		Peers: []*peer.AddrInfo{},
	}
	externalHost, _ := p2p.NewHost(privKeyForHost, topology, p2p.NewConnectionGate(topology), uint16(4005))

	firstSubChannel := make(chan *comm.WrappedMessage)
	s.testCommunications[0].Subscribe(s.testSessionID, comm.CoordinatorPingMsg, firstSubChannel)

	secondSubChannel := make(chan *comm.WrappedMessage)
	s.testCommunications[1].Subscribe(s.testSessionID, comm.CoordinatorPingMsg, secondSubChannel)

	go func() {
		msg := <-firstSubChannel
		s.Equal("1", msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()

	go func() {
		msg := <-secondSubChannel
		s.Equal("1", msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()

	err := s.testCommunications[2].Broadcast(
		[]peer.ID{s.testHosts[0].ID(), externalHost.ID(), s.testHosts[1].ID()},
		nil,
		comm.CoordinatorPingMsg,
		"1",
	)
	s.NotNil(err)
}

func (s *CommunicationIntegrationTestSuite) TestCommunication_BroadcastMessage_StopReceivingMessagesAfterUnsubscribe() {
	/** Both subscribers got a message **/

	firstSubChannel := make(chan *comm.WrappedMessage)
	firstSubID := s.testCommunications[0].Subscribe(s.testSessionID, comm.CoordinatorPingMsg, firstSubChannel)

	go func() {
		msg := <-firstSubChannel
		s.Equal("1", msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()

	secondSubChannel := make(chan *comm.WrappedMessage)
	s.testCommunications[1].Subscribe(s.testSessionID, comm.CoordinatorPingMsg, secondSubChannel)

	go func() {
		msg := <-secondSubChannel
		s.Equal(s.testSessionID, msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()

	err := s.testCommunications[2].Broadcast(
		[]peer.ID{s.testHosts[0].ID(), s.testHosts[1].ID()},
		nil,
		comm.CoordinatorPingMsg,
		"1",
	)
	s.Nil(err)

	/** After unsubscribe only one subscriber got a message **/

	s.testCommunications[0].UnSubscribe(firstSubID)

	err = s.testCommunications[2].Broadcast(
		[]peer.ID{s.testHosts[0].ID(), s.testHosts[1].ID()},
		nil,
		comm.CoordinatorPingMsg,
		"1",
	)
	s.Nil(err)

	time.Sleep(1 * time.Second)

	go func() {
		select {
		case <-firstSubChannel:
			s.Fail("host[0] should be unsubscribed")
			break
		default:
			break
		}
	}()

	go func() {
		msg := <-secondSubChannel
		s.Equal(s.testSessionID, msg.SessionID)
		s.Equal(comm.CoordinatorPingMsg, msg.MessageType)
		s.Equal(s.testHosts[2].ID(), msg.From)
	}()
}

/**
* Util function used for setting tests with multiple communications
 */
func InitializeHostsAndCommunications(numberOfActors int, protocolID protocol.ID) ([]host.Host, []comm.Communication) {
	topology := &topology.NetworkTopology{
		Peers: []*peer.AddrInfo{},
	}
	privateKeys := []crypto.PrivKey{}
	for i := 0; i < numberOfActors; i++ {
		privKeyForHost, _, _ := crypto.GenerateKeyPair(crypto.ECDSA, 1)
		privateKeys = append(privateKeys, privKeyForHost)
		peerID, _ := peer.IDFromPrivateKey(privKeyForHost)
		addrInfoForHost, _ := peer.AddrInfoFromString(fmt.Sprintf(
			"/ip4/127.0.0.1/tcp/%d/p2p/%s", 4000+i, peerID.Pretty(),
		))
		topology.Peers = append(topology.Peers, addrInfoForHost)
	}

	var testHosts []host.Host
	// create test hosts
	for i := 0; i < numberOfActors; i++ {
		newHost, _ := p2p.NewHost(privateKeys[i], topology, p2p.NewConnectionGate(topology), uint16(4000+i))
		testHosts = append(testHosts, newHost)
	}

	// populate peerstores
	peersAdrInfos := map[int][]*peer.AddrInfo{}
	for i := 0; i < numberOfActors; i++ {
		for j := 0; j < numberOfActors; j++ {
			if i != j {
				adrInfoForHost, _ := peer.AddrInfoFromString(fmt.Sprintf(
					"/ip4/127.0.0.1/tcp/%d/p2p/%s", 4000+j, testHosts[j].ID().Pretty(),
				))
				testHosts[i].Peerstore().AddAddr(adrInfoForHost.ID, adrInfoForHost.Addrs[0], peerstore.PermanentAddrTTL)
				peersAdrInfos[i] = append(peersAdrInfos[i], adrInfoForHost)
			}
		}
	}

	// create communications
	var testCommunications []comm.Communication
	for i := 0; i < numberOfActors; i++ {
		com := p2p.NewCommunication(
			testHosts[i],
			protocolID,
		)
		testCommunications = append(testCommunications, com)
	}

	return testHosts, testCommunications
}
