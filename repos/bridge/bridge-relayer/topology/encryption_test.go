// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology_test

import (
	"encoding/json"
	"testing"

	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/stretchr/testify/suite"
)

type AESEncryptionTestSuite struct {
	suite.Suite
	aesEncryption *topology.AESEncryption
}

func TestRunAESEncryptionTestSuite(t *testing.T) {
	suite.Run(t, new(AESEncryptionTestSuite))
}

func (s *AESEncryptionTestSuite) SetupTest() {
	cipherKey := []byte("v8y/B?E(H+MbQeTh")
	s.aesEncryption, _ = topology.NewAESEncryption(cipherKey)
}

func (s *AESEncryptionTestSuite) Test_EncrDecr() {
	expectedTopology := topology.RawTopology{
		Peers: []topology.RawPeer{
			{PeerAddress: "/dns4/relayer2/tcp/9001/p2p/QmeTuMtdpPB7zKDgmobEwSvxodrf5aFVSmBXX3SQJVjJaT"},
			{PeerAddress: "/dns4/relayer3/tcp/9002/p2p/QmYAYuLUPNwYEBYJaKHcE7NKjUhiUV8txx2xDXHvcYa1xK"},
			{PeerAddress: "/dns4/relayer1/tcp/9000/p2p/QmcvEg7jGvuxdsUFRUiE4VdrL2P1Yeju5L83BsJvvXz7zX"},
		},
		Threshold: "2",
	}

	pt, err := json.Marshal(expectedTopology)

	s.Nil(err)

	ct, err := s.aesEncryption.Encrypt(pt)
	s.Nil(err)

	resultingPt := s.aesEncryption.Decrypt(ct)

	decryptedTopology := topology.RawTopology{}

	err = json.Unmarshal(resultingPt, &decryptedTopology)
	s.Nil(err)

	s.Equal(expectedTopology, decryptedTopology)
}
