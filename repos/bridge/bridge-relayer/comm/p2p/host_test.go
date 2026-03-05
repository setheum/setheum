// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p_test

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/comm/p2p"
	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/golang/mock/gomock"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/stretchr/testify/suite"
)

type HostTestSuite struct {
	suite.Suite
	mockController *gomock.Controller
}

func TestRunHostTestSuite(t *testing.T) {
	suite.Run(t, new(HostTestSuite))
}

func (s *HostTestSuite) SetupTest() {
	s.mockController = gomock.NewController(s.T())
}

func (s *HostTestSuite) TestHost_NewHost_Success() {
	p1RawAddress := "/ip4/127.0.0.1/tcp/4000/p2p/QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR"
	p2RawAddress := "/ip4/127.0.0.1/tcp/4002/p2p/QmeWhpY8tknHS29gzf9TAsNEwfejTCNJ7vFpmkV6rNUgyq"

	privKey, _, err := crypto.GenerateKeyPair(2, 0)
	if err != nil {
		return
	}
	s.Nil(err)

	p1, _ := peer.AddrInfoFromString(p1RawAddress)
	p2, _ := peer.AddrInfoFromString(p2RawAddress)

	topology := &topology.NetworkTopology{
		Peers: []*peer.AddrInfo{
			p1, p2,
		},
	}
	host, err := p2p.NewHost(
		privKey,
		topology,
		p2p.NewConnectionGate(topology),
		2020,
	)
	s.Nil(err)
	s.NotNil(host)
	s.Len(host.Peerstore().Peers(), 2)
}

func (s *HostTestSuite) TestHost_NewHost_InvalidPrivKey() {
	host, err := p2p.NewHost(
		nil,
		&topology.NetworkTopology{
			Peers: []*peer.AddrInfo{},
		},
		p2p.NewConnectionGate(&topology.NetworkTopology{}),
		2020,
	)
	s.Nil(host)
	s.NotNil(err)
}

type LoadPeersTestSuite struct {
	suite.Suite
	host host.Host
}

func TestRunLoadPeersTestSuite(t *testing.T) {
	suite.Run(t, new(LoadPeersTestSuite))
}

func (s *LoadPeersTestSuite) SetupTest() {
	p1RawAddress := "/ip4/127.0.0.1/tcp/4000/p2p/QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR"
	p2RawAddress := "/ip4/127.0.0.1/tcp/4002/p2p/QmeWhpY8tknHS29gzf9TAsNEwfejTCNJ7vFpmkV6rNUgyq"
	privKey, _, err := crypto.GenerateKeyPair(2, 0)
	if err != nil {
		panic(err)
	}
	p1, _ := peer.AddrInfoFromString(p1RawAddress)
	p2, _ := peer.AddrInfoFromString(p2RawAddress)
	topology := &topology.NetworkTopology{Peers: []*peer.AddrInfo{p1, p2}}
	host, err := p2p.NewHost(privKey, topology, p2p.NewConnectionGate(topology), 2020)
	if err != nil {
		panic(err)
	}
	s.host = host
}

func peerInSlice(peer peer.ID, peers peer.IDSlice) bool {
	for _, p := range peers {
		if peer == p {
			return true
		}
	}

	return false
}

func (s *LoadPeersTestSuite) Test_LoadPeers_RemovesOldAndSetsNewPeers() {
	newP1RawAddress := "/dns4/relayer2/tcp/9001/p2p/QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT"
	newP2RawAddress := "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"
	newP1, _ := peer.AddrInfoFromString(newP1RawAddress)
	newP2, _ := peer.AddrInfoFromString(newP2RawAddress)

	p2p.LoadPeers(s.host, []*peer.AddrInfo{newP1, newP2})

	s.Equal(peerInSlice(newP1.ID, s.host.Peerstore().Peers()), true)
	s.Equal(peerInSlice(newP2.ID, s.host.Peerstore().Peers()), true)
	s.Equal(len(s.host.Peerstore().Peers()), 2)
}
