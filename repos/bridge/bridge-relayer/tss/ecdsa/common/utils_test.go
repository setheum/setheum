// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package common_test

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/common"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/stretchr/testify/suite"
)

type PeersFromPartiesTestSuite struct {
	suite.Suite
}

func TestRunPeersFromPartiesTestSuite(t *testing.T) {
	suite.Run(t, new(PeersFromPartiesTestSuite))
}

func (s *PeersFromPartiesTestSuite) Test_NoParties() {
	peers, err := common.PeersFromParties([]*tss.PartyID{})

	s.Nil(err)
	s.Equal(peers, []peer.ID{})
}

func (s *PeersFromPartiesTestSuite) Test_InvalidParty() {
	party1 := common.CreatePartyID("invalid")

	_, err := common.PeersFromParties([]*tss.PartyID{party1})

	s.NotNil(err)
}

func (s *PeersFromPartiesTestSuite) Test_ValidParties() {
	party1 := common.CreatePartyID("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	party2 := common.CreatePartyID("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peerID1, _ := peer.Decode(party1.Id)
	peerID2, _ := peer.Decode(party2.Id)

	peers, err := common.PeersFromParties([]*tss.PartyID{party1, party2})

	s.Nil(err)
	s.Equal(peers, []peer.ID{peerID1, peerID2})
}

type PeersFromIDSTestSuite struct {
	suite.Suite
}

func TestRunPeersFromIDSTestSuite(t *testing.T) {
	suite.Run(t, new(PeersFromIDSTestSuite))
}

func (s *PeersFromIDSTestSuite) Test_NoIDS() {
	peers, err := common.PeersFromIDS([]string{})

	s.Nil(err)
	s.Equal(peers, []peer.ID{})
}

func (s *PeersFromIDSTestSuite) Test_InvalidParty() {
	_, err := common.PeersFromIDS([]string{"invalid"})

	s.NotNil(err)
}

func (s *PeersFromIDSTestSuite) Test_ValidIDS() {
	party1 := common.CreatePartyID("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	party2 := common.CreatePartyID("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peerID1, _ := peer.Decode(party1.Id)
	peerID2, _ := peer.Decode(party2.Id)

	peers, err := common.PeersFromIDS([]string{party1.Id, party2.Id})

	s.Nil(err)
	s.Equal(peers, []peer.ID{peerID1, peerID2})
}

type PartiesFromPeersTestSuite struct {
	suite.Suite
}

func TestRunPartiesFromPeersTestSuite(t *testing.T) {
	suite.Run(t, new(PartiesFromPeersTestSuite))
}

func (s *PartiesFromPeersTestSuite) Test_ValidPeers() {
	party1 := common.CreatePartyID("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	party1.Index = 1
	party2 := common.CreatePartyID("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	party2.Index = 0
	peerID1, _ := peer.Decode(party1.Id)
	peerID2, _ := peer.Decode(party2.Id)

	sortedParties := common.PartiesFromPeers([]peer.ID{peerID1, peerID2})

	s.Equal(sortedParties, tss.SortedPartyIDs{party2, party1})
}

type ExcludedPeersTestSuite struct {
	suite.Suite
}

func TestRunExcludedPeersTestSuite(t *testing.T) {
	suite.Run(t, new(ExcludedPeersTestSuite))
}

func (s *ExcludedPeersTestSuite) Test_ExcludePeers_Excluded() {
	peerID1, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	peerID2, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peerID3, _ := peer.Decode("QmYayosTHxL2xa4jyrQ2PmbhGbrkSxsGM1kzXLTT8SsLVy")
	peers := []peer.ID{peerID1, peerID2, peerID3}
	excludedPeers := []peer.ID{peerID3, peerID2}

	includedPeers := common.ExcludePeers(peers, excludedPeers)

	s.Equal(includedPeers, peer.IDSlice{peerID1})
}

type PeersIntersectionTestSuite struct {
	suite.Suite
}

func TestRunPeersIntersectionTestSuite(t *testing.T) {
	suite.Run(t, new(PeersIntersectionTestSuite))
}

func (s *PeersIntersectionTestSuite) Test_PeersIntersection() {
	peerID1, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	peerID2, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peerID3, _ := peer.Decode("QmYayosTHxL2xa4jyrQ2PmbhGbrkSxsGM1kzXLTT8SsLVy")
	oldPeers := []peer.ID{peerID1, peerID2, peerID3}
	newPeers := []peer.ID{peerID3, peerID2}

	includedPeers := common.PeersIntersection(oldPeers, newPeers)

	s.Equal(includedPeers, peer.IDSlice{peerID2, peerID3})
}
