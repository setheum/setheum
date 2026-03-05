// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package util_test

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/tss/util"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/stretchr/testify/suite"
)

type IsParticipantTestSuite struct {
	suite.Suite
}

func TestRunIsParticipantTestSuite(t *testing.T) {
	suite.Run(t, new(IsParticipantTestSuite))
}

func (s *IsParticipantTestSuite) Test_ValidParticipant() {
	peerID1 := "QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54"
	peerID2 := "QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF56"
	peers := peer.IDSlice{peer.ID(peerID1), peer.ID(peerID2)}

	isParticipant := util.IsParticipant(peer.ID(peerID1), peers)

	s.Equal(true, isParticipant)
}

func (s *IsParticipantTestSuite) Test_InvalidParticipant() {
	peerID1 := "QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54"
	peerID2 := "QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF56"
	peers := peer.IDSlice{peer.ID(peerID2)}

	isParticipant := util.IsParticipant(peer.ID(peerID1), peers)

	s.Equal(false, isParticipant)
}

type SortPeersForSessionTestSuite struct {
	suite.Suite
}

func TestRunSortPeersForSessionTestSuite(t *testing.T) {
	suite.Run(t, new(SortPeersForSessionTestSuite))
}

func (s *SortPeersForSessionTestSuite) Test_NoPeers() {
	sortedPeers := util.SortPeersForSession([]peer.ID{}, "sessioniD")

	s.Equal(sortedPeers, util.SortablePeerSlice{})
}

func (s *SortPeersForSessionTestSuite) Test_ValidPeers() {
	peer1, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	peer2, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peer3, _ := peer.Decode("QmYayosTHxL2xa4jyrQ2PmbhGbrkSxsGM1kzXLTT8SsLVy")
	peers := []peer.ID{peer3, peer2, peer1}

	sortedPeers := util.SortPeersForSession(peers, "sessionID")

	s.Equal(sortedPeers, util.SortablePeerSlice{
		util.PeerMsg{SessionID: "sessionID", ID: peer1},
		util.PeerMsg{SessionID: "sessionID", ID: peer2},
		util.PeerMsg{SessionID: "sessionID", ID: peer3},
	})
}
