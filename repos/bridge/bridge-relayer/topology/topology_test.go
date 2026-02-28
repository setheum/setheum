// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology_test

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"testing"

	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/golang/mock/gomock"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/stretchr/testify/suite"

	mock_topology "github.com/ChainSafe/sygma-relayer/topology/mock"
)

type TopologyTestSuite struct {
	suite.Suite
}

func TestRunTopologyTestSuite(t *testing.T) {
	suite.Run(t, new(TopologyTestSuite))
}

func (s *TopologyTestSuite) SetupTest() {
	os.Clearenv()
}

func (s *TopologyTestSuite) Test_ProcessRawTopology_ValidTopology() {
	topology, err := topology.ProcessRawTopology(&topology.RawTopology{
		Peers: []topology.RawPeer{
			{PeerAddress: "/dns4/relayer2/tcp/9001/p2p/QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT"},
			{PeerAddress: "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
			{PeerAddress: "/dns4/relayer1/tcp/9000/p2p/QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX"},
		},
		Threshold: "2",
	})
	s.Nil(err)
	s.Equal(2, topology.Threshold)
	s.Equal("QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT", topology.Peers[0].ID.Pretty())
	s.Equal("QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK", topology.Peers[1].ID.Pretty())
	s.Equal("QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX", topology.Peers[2].ID.Pretty())
}

func (s *TopologyTestSuite) Test_ProcessRawTopology_InvalidPeerAddress() {
	_, err := topology.ProcessRawTopology(&topology.RawTopology{
		Peers: []topology.RawPeer{
			{PeerAddress: "/dns4/relayer2/tcp/9001/p2p/"},
			{PeerAddress: "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
			{PeerAddress: "/dns4/relayer1/tcp/9000/p2p/QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX"},
		},
		Threshold: "2",
	})
	s.NotNil(err)
}

func (s *TopologyTestSuite) Test_ProcessRawTopology_InvalidThreshold() {
	rt := &topology.RawTopology{
		Peers: []topology.RawPeer{
			{PeerAddress: "/dns4/relayer2/tcp/9001/p2p/QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT"},
			{PeerAddress: "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
			{PeerAddress: "/dns4/relayer1/tcp/9000/p2p/QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX"},
		},
		Threshold: "-1",
	}
	_, err := topology.ProcessRawTopology(rt)
	s.NotNil(err)

	rt.Threshold = "0"
	_, err = topology.ProcessRawTopology(rt)
	s.NotNil(err)

	rt.Threshold = "1"
	_, err = topology.ProcessRawTopology(rt)
	s.Nil(err)
}

type NetworkTopologyTestSuite struct {
	suite.Suite
}

func TestRunNetworkTopologyTestSuite(t *testing.T) {
	suite.Run(t, new(NetworkTopologyTestSuite))
}

func (s *NetworkTopologyTestSuite) Test_IsAllowedPeer_ValidPeer() {
	p1RawAddress := "/ip4/127.0.0.1/tcp/4000/p2p/QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR"
	p2RawAddress := "/ip4/127.0.0.1/tcp/4002/p2p/QmeWhpY8tknHS29gzf9TAsNEwfejTCNJ7vFpmkV6rNUgyq"
	p1, _ := peer.AddrInfoFromString(p1RawAddress)
	p2, _ := peer.AddrInfoFromString(p2RawAddress)
	topology := topology.NetworkTopology{
		Peers: []*peer.AddrInfo{
			p1, p2,
		},
		Threshold: 2,
	}

	isAllowed := topology.IsAllowedPeer(p1.ID)

	s.Equal(isAllowed, true)
}

func (s *NetworkTopologyTestSuite) Test_IsAllowedPeer_InvalidPeer() {
	p1RawAddress := "/ip4/127.0.0.1/tcp/4000/p2p/QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR"
	p2RawAddress := "/ip4/127.0.0.1/tcp/4002/p2p/QmeWhpY8tknHS29gzf9TAsNEwfejTCNJ7vFpmkV6rNUgyq"
	p3RawAddress := "/ip4/127.0.0.1/tcp/4002/p2p/QmeWhpY8tknHS29gzf9TAsNEwfejTCNJ7vFpmkV6rNUgyg"
	p1, _ := peer.AddrInfoFromString(p1RawAddress)
	p2, _ := peer.AddrInfoFromString(p2RawAddress)
	p3, _ := peer.AddrInfoFromString(p3RawAddress)
	topology := topology.NetworkTopology{
		Peers: []*peer.AddrInfo{
			p1, p2,
		},
		Threshold: 2,
	}

	isAllowed := topology.IsAllowedPeer(p3.ID)

	s.Equal(isAllowed, false)
}

type TopologyProviderTestSuite struct {
	suite.Suite
	fetcher *mock_topology.MockFetcher
}

func TestRunTopologyProviderTestSuite(t *testing.T) {
	suite.Run(t, new(TopologyProviderTestSuite))
}

func (s *TopologyProviderTestSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	s.fetcher = mock_topology.NewMockFetcher(ctrl)

}

func (s *TopologyProviderTestSuite) Test_FetchingTopologyFails() {
	s.fetcher.EXPECT().Get("test.url").Return(&http.Response{}, fmt.Errorf("error"))
	topologyConfiguration := relayer.TopologyConfiguration{
		Url:           "test.url",
		EncryptionKey: "qwertyuiopasdfgh",
	}
	topologyProvider, _ := topology.NewNetworkTopologyProvider(topologyConfiguration, s.fetcher)

	_, err := topologyProvider.NetworkTopology("")

	s.NotNil(err)
}

func (s *TopologyProviderTestSuite) Test_ValidTopology() {
	resp := &http.Response{}
	resp.Body = io.NopCloser(strings.NewReader("f533758136cd1f62c3c7fd96b41d439ce3c899b0e705ecebd567275e4447683f80c21d9cf6287d3ac504f116c18308d34fd1f79cda675983dc01231cdb13db39f271f37bbc4ed9f89b87b04ed74cb4de382e43809a2e690c7a0872c1c2eec631455628621291803d34c73965917b52b44e713d927db805bbc145a2fe51c7352ab8b34f216a57c19e2e3dca27a1cf2013a9e6ece2989fd90bff45ad614520419bc132bd07d4aa89f1afb4016ba16b8de0b8921071ab99d86f4c15672c08ad98a55c0b179cff340dc128c3f8a56876d9a75aec735924fcba5f21ae6e64cf875f23cc1fdef4ae5c3d0f43e421d75161fd44d3a7a4cbab3c6ff84e7ff3b83582944c93627c75ad93262d057889e53d48263749dab0355adc8f949b946f3da3e9a4a104728a4f56214bb177bd5d59a257cf55befb53b6bff1b293f883bd60b7c1aa13c75e8ffd394b130ab6d867e60bfef67c78432663775093023c66bbad812bdda890de43b5491dd27a75ae27b79d85afc0ff390b531743642066c200ea5a405ef746041fa5fbf75c23c4dd35a1cc9854b01f1aaeec4265b4c46145a99e6b02eba82408903117fa34917368d5012420a2f985d2eac929c758d487e93f7779ae8ba6ff0f7f1eca1997abbc3ff0efdf"))
	s.fetcher.EXPECT().Get("test.url").Return(resp, nil)
	topologyConfiguration := relayer.TopologyConfiguration{
		Url:           "test.url",
		EncryptionKey: "qwertyuiopasdfgh",
	}
	topologyProvider, _ := topology.NewNetworkTopologyProvider(topologyConfiguration, s.fetcher)

	tp, err := topologyProvider.NetworkTopology("")

	rawTp, _ := topology.ProcessRawTopology(&topology.RawTopology{
		Peers: []topology.RawPeer{
			{PeerAddress: "/dns4/relayer1/tcp/9000/p2p/QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX"},
			{PeerAddress: "/dns4/relayer2/tcp/9001/p2p/QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT"},
			{PeerAddress: "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
			{PeerAddress: "/dns4/relayer-0.test.com/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
		},
		Threshold: "2",
	})
	s.Nil(err)
	s.Equal(rawTp, tp)
}

func (s *TopologyProviderTestSuite) Test_InvalidHash() {
	resp := &http.Response{}
	resp.Body = io.NopCloser(strings.NewReader("f533758136cd1f62c3c7fd96b41d439ce3c899b0e705ecebd567275e4447683f80c21d9cf6287d3ac504f116c18308d34fd1f79cda675983dc01231cdb13db39f271f37bbc4ed9f89b87b04ed74cb4de382e43809a2e690c7a0872c1c2eec631455628621291803d34c73965917b52b44e713d927db805bbc145a2fe51c7352ab8b34f216a57c19e2e3dca27a1cf2013a9e6ece2989fd90bff45ad614520419bc132bd07d4aa89f1afb4016ba16b8de0b8921071ab99d86f4c15672c08ad98a55c0b179cff340dc128c3f8a56876d9a75aec735924fcba5f21ae6e64cf875f23cc1fdef4ae5c3d0f43e421d75161fd44d3a7a4cbab3c6ff84e7ff3b83582944c93627c75ad93262d057889e53d48263749dab0355adc8f949b946f3da3e9a4a104728a4f56214bb177bd5d59a257cf55befb53b6bff1b293f883bd60b7c1aa13c75e8ffd394b130ab6d867e60bfef67c78432663775093023c66bbad812bdda890de43b5491dd27a75ae27b79d85afc0ff390b531743642066c200ea5a405ef746041fa5fbf75c23c4dd35a1cc9854b01f1aaeec4265b4c46145a99e6b02eba82408903117fa34917368d5012420a2f985d2eac929c758d487e93f7779ae8ba6ff0f7f1eca1997abbc3ff0efdf"))
	s.fetcher.EXPECT().Get("test.url").Return(resp, nil)
	topologyConfiguration := relayer.TopologyConfiguration{
		Url:           "test.url",
		EncryptionKey: "qwertyuiopasdfgh",
	}
	topologyProvider, _ := topology.NewNetworkTopologyProvider(topologyConfiguration, s.fetcher)

	_, err := topologyProvider.NetworkTopology("invalid")

	s.NotNil(err)
}

func (s *TopologyProviderTestSuite) Test_ValidHash() {
	resp := &http.Response{}
	resp.Body = io.NopCloser(strings.NewReader("f533758136cd1f62c3c7fd96b41d439ce3c899b0e705ecebd567275e4447683f80c21d9cf6287d3ac504f116c18308d34fd1f79cda675983dc01231cdb13db39f271f37bbc4ed9f89b87b04ed74cb4de382e43809a2e690c7a0872c1c2eec631455628621291803d34c73965917b52b44e713d927db805bbc145a2fe51c7352ab8b34f216a57c19e2e3dca27a1cf2013a9e6ece2989fd90bff45ad614520419bc132bd07d4aa89f1afb4016ba16b8de0b8921071ab99d86f4c15672c08ad98a55c0b179cff340dc128c3f8a56876d9a75aec735924fcba5f21ae6e64cf875f23cc1fdef4ae5c3d0f43e421d75161fd44d3a7a4cbab3c6ff84e7ff3b83582944c93627c75ad93262d057889e53d48263749dab0355adc8f949b946f3da3e9a4a104728a4f56214bb177bd5d59a257cf55befb53b6bff1b293f883bd60b7c1aa13c75e8ffd394b130ab6d867e60bfef67c78432663775093023c66bbad812bdda890de43b5491dd27a75ae27b79d85afc0ff390b531743642066c200ea5a405ef746041fa5fbf75c23c4dd35a1cc9854b01f1aaeec4265b4c46145a99e6b02eba82408903117fa34917368d5012420a2f985d2eac929c758d487e93f7779ae8ba6ff0f7f1eca1997abbc3ff0efdf"))
	s.fetcher.EXPECT().Get("test.url").Return(resp, nil)
	topologyConfiguration := relayer.TopologyConfiguration{
		Url:           "test.url",
		EncryptionKey: "qwertyuiopasdfgh",
	}
	topologyProvider, _ := topology.NewNetworkTopologyProvider(topologyConfiguration, s.fetcher)

	expectedHash := "49cd57ba3b3296a994b2f7ef004164c55d16650fbb0306f31963ceb800ca5bc9"
	tp, err := topologyProvider.NetworkTopology(expectedHash)

	rawTp, _ := topology.ProcessRawTopology(&topology.RawTopology{
		Peers: []topology.RawPeer{
			{PeerAddress: "/dns4/relayer1/tcp/9000/p2p/QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX"},
			{PeerAddress: "/dns4/relayer2/tcp/9001/p2p/QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT"},
			{PeerAddress: "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
			{PeerAddress: "/dns4/relayer-0.test.com/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
		},
		Threshold: "2",
	})
	s.Nil(err)
	s.Equal(rawTp, tp)
}
