// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p_test

import (
	"encoding/json"
	"fmt"
	"testing"

	comm "github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/p2p"
	mock_host "github.com/ChainSafe/sygma-relayer/comm/p2p/mock/host"
	mock_network "github.com/ChainSafe/sygma-relayer/comm/p2p/mock/stream"
	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/ChainSafe/sygma-relayer/tss/message"
	"github.com/golang/mock/gomock"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/core/protocol"
	"github.com/stretchr/testify/suite"
)

type Libp2pCommunicationTestSuite struct {
	suite.Suite
	mockController *gomock.Controller
	mockHost       *mock_host.MockHost
	testProtocolID protocol.ID
	allowedPeers   peer.IDSlice
}

func TestRunLibp2pCommunicationTestSuite(t *testing.T) {
	suite.Run(t, new(Libp2pCommunicationTestSuite))
}

func (s *Libp2pCommunicationTestSuite) SetupSuite() {
	pid, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	s.allowedPeers = []peer.ID{pid}
	s.testProtocolID = "test/protocol"
}
func (s *Libp2pCommunicationTestSuite) SetupTest() {
	s.mockController = gomock.NewController(s.T())
	s.mockHost = mock_host.NewMockHost(s.mockController)
}

func (s *Libp2pCommunicationTestSuite) TestLibp2pCommunication_MessageProcessing_ValidMessage() {
	s.mockHost.EXPECT().ID().Return(s.allowedPeers[0])
	s.mockHost.EXPECT().SetStreamHandler(s.testProtocolID, gomock.Any()).Return()
	c := p2p.NewCommunication(s.mockHost, s.testProtocolID)

	msgChannel := make(chan *comm.WrappedMessage)
	c.Subscribe("1", comm.CoordinatorPingMsg, msgChannel)

	testWrappedMsg := comm.WrappedMessage{
		MessageType: comm.CoordinatorPingMsg,
		SessionID:   "1",
		Payload:     nil,
	}
	bytes, _ := json.Marshal(testWrappedMsg)

	mockStream := mock_network.NewMockStream(s.mockController)
	mockConn := mock_network.NewMockConn(s.mockController)
	mockConn.EXPECT().RemotePeer().Return(s.allowedPeers[0])
	mockStream.EXPECT().Conn().Return(mockConn)

	firstCall := mockStream.EXPECT().Read(gomock.Any()).DoAndReturn(func(p []byte) (n int, err error) {
		copy(p[:], []byte(fmt.Sprintf("%s \n", string(bytes[:]))))
		return len(bytes), nil
	})
	secondCall := mockStream.EXPECT().Read(gomock.Any()).DoAndReturn(func(p []byte) (n int, err error) {
		copy(p[:], []byte("\n"))
		return len(bytes), nil
	})
	gomock.InOrder(firstCall, secondCall)

	c.ProcessMessagesFromStream(mockStream)

	msg := <-msgChannel

	s.Equal(s.allowedPeers[0], msg.From)
	s.Equal(testWrappedMsg.MessageType, msg.MessageType)
	s.Equal(testWrappedMsg.SessionID, msg.SessionID)
	s.Nil(msg.Payload)
}

func (s *Libp2pCommunicationTestSuite) TestLibp2pCommunication_StreamHandlerFunction_ValidMessageWithSubscribers() {
	s.mockHost.EXPECT().ID().Return(s.allowedPeers[0])
	s.mockHost.EXPECT().SetStreamHandler(s.testProtocolID, gomock.Any()).Return()
	c := p2p.NewCommunication(s.mockHost, s.testProtocolID)

	testWrappedMsg := comm.WrappedMessage{
		MessageType: comm.CoordinatorPingMsg,
		SessionID:   "1",
		Payload:     nil,
	}

	bytes, _ := json.Marshal(testWrappedMsg)

	mockStream := mock_network.NewMockStream(s.mockController)
	mockConn := mock_network.NewMockConn(s.mockController)
	mockConn.EXPECT().RemotePeer().AnyTimes().Return(s.allowedPeers[0])
	mockStream.EXPECT().Conn().AnyTimes().Return(mockConn)
	mockStream.EXPECT().Close()

	firstCall := mockStream.EXPECT().Read(gomock.Any()).DoAndReturn(func(p []byte) (n int, err error) {
		copy(p[:], []byte(fmt.Sprintf("%s \n", string(bytes[:]))))
		return len(bytes), nil
	})
	secondCall := mockStream.EXPECT().Read(gomock.Any()).DoAndReturn(func(p []byte) (n int, err error) {
		copy(p[:], []byte("\n"))
		return len(bytes), nil
	})
	gomock.InOrder(firstCall, secondCall)

	testSubChannelFirst := make(chan *comm.WrappedMessage)
	subID1 := c.Subscribe("1", comm.CoordinatorPingMsg, testSubChannelFirst)

	testSubChannelSecond := make(chan *comm.WrappedMessage)
	subID2 := c.Subscribe("1", comm.CoordinatorPingMsg, testSubChannelSecond)

	go c.StreamHandlerFunc(mockStream)

	subMsgFirst := <-testSubChannelFirst
	s.NotNil(subMsgFirst)
	s.Equal(s.allowedPeers[0], subMsgFirst.From)
	s.Equal(testWrappedMsg.MessageType, subMsgFirst.MessageType)
	s.Equal(testWrappedMsg.SessionID, subMsgFirst.SessionID)
	s.Nil(subMsgFirst.Payload)

	subMsgSecond := <-testSubChannelSecond
	s.NotNil(subMsgSecond)
	s.Equal(s.allowedPeers[0], subMsgSecond.From)
	s.Equal(testWrappedMsg.MessageType, subMsgSecond.MessageType)
	s.Equal(testWrappedMsg.SessionID, subMsgSecond.SessionID)
	s.Nil(subMsgSecond.Payload)

	c.UnSubscribe(subID1)
	c.UnSubscribe(subID2)
}

func (s *Libp2pCommunicationTestSuite) TestLibp2pCommunication_SendReceiveMessage() {
	var testHosts []host.Host
	var communications []p2p.Libp2pCommunication
	numberOfTestHosts := 2
	portOffset := 0
	protocolID := "/p2p/test"

	topology := &topology.NetworkTopology{
		Peers: []*peer.AddrInfo{},
	}

	privateKeys := []crypto.PrivKey{}
	for i := 0; i < numberOfTestHosts; i++ {
		privKeyForHost, _, _ := crypto.GenerateKeyPair(crypto.ECDSA, 1)
		privateKeys = append(privateKeys, privKeyForHost)
		peerID, _ := peer.IDFromPrivateKey(privKeyForHost)
		addrInfoForHost, _ := peer.AddrInfoFromString(fmt.Sprintf(
			"/ip4/127.0.0.1/tcp/%d/p2p/%s", 4000+portOffset+i, peerID.Pretty(),
		))
		topology.Peers = append(topology.Peers, addrInfoForHost)
	}

	for i := 0; i < numberOfTestHosts; i++ {
		connectionGate := p2p.NewConnectionGate(topology)
		newHost, _ := p2p.NewHost(privateKeys[i], topology, connectionGate, uint16(4000+portOffset+i))
		testHosts = append(testHosts, newHost)
		communications = append(communications, p2p.NewCommunication(newHost, protocol.ID(protocolID)))
	}

	msgChn := make(chan *comm.WrappedMessage)
	communications[1].SubscribeTo("1", comm.CoordinatorPingMsg, msgChn)
	communications[1].SubscribeTo("2", comm.TssKeySignMsg, msgChn)

	err := communications[0].Broadcast([]peer.ID{testHosts[1].ID()}, []byte{}, comm.CoordinatorPingMsg, "1")
	s.Nil(err)
	pingMsg := <-msgChn

	msgBytes, _ := message.MarshalTssMessage([]byte("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"), true)
	err = communications[0].Broadcast([]peer.ID{testHosts[1].ID()}, msgBytes, comm.TssKeySignMsg, "2")
	s.Nil(err)
	largeMsg := <-msgChn

	s.Equal(pingMsg, &comm.WrappedMessage{
		MessageType: comm.CoordinatorPingMsg,
		SessionID:   "1",
		Payload:     []byte{},
		From:        testHosts[0].ID(),
	})
	s.Equal(largeMsg, &comm.WrappedMessage{
		MessageType: comm.TssKeySignMsg,
		SessionID:   "2",
		Payload:     msgBytes,
		From:        testHosts[0].ID(),
	})
}
