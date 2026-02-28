// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package keyshare_test

import (
	"encoding/base64"
	"os"
	"testing"

	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/stretchr/testify/suite"
	"github.com/taurusgroup/multi-party-sig/pkg/math/curve"
	"github.com/taurusgroup/multi-party-sig/pkg/party"
	"github.com/taurusgroup/multi-party-sig/pkg/taproot"
	"github.com/taurusgroup/multi-party-sig/protocols/frost"
)

type FrostKeyshareStoreTestSuite struct {
	suite.Suite
	keyshareStore *keyshare.FrostKeyshareStore
	path          string
}

func TestRunFrostKeyshareStoreTestSuite(t *testing.T) {
	suite.Run(t, new(FrostKeyshareStoreTestSuite))
}

func (s *FrostKeyshareStoreTestSuite) SetupTest() {
	s.path = "share.json"
	s.keyshareStore = keyshare.NewFrostKeyshareStore(s.path)
}
func (s *FrostKeyshareStoreTestSuite) TearDownTest() {
	os.Remove(s.path)
}

func (s *FrostKeyshareStoreTestSuite) Test_RetrieveInvalidFile() {
	_, err := s.keyshareStore.GetKeyshare()
	s.NotNil(err)
}

func (s *FrostKeyshareStoreTestSuite) Test_StoreAndRetrieveShare() {
	privateShare := &curve.Secp256k1Scalar{}
	privateShareBytes, _ := base64.StdEncoding.DecodeString("hpUx9M/dN7lAF20Jum3/4sgmfty5W4VNeGoEEB18870=")
	_ = privateShare.UnmarshalBinary(privateShareBytes)

	verificationShares := make(map[party.ID]*curve.Secp256k1Point)
	point := &curve.Secp256k1Point{}
	pointBytes, _ := base64.StdEncoding.DecodeString("Au1e9fpMRj99yTydAjKGGa9H7m/jEpEvySUmTu1xYR9h")
	_ = point.UnmarshalBinary(pointBytes)
	verificationShares[party.ID("1")] = point

	threshold := 3
	peer1, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peer2, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	peers := []peer.ID{peer1, peer2}

	keyshare := keyshare.NewFrostKeyshare(&frost.TaprootConfig{
		ID:                 party.ID(peer1.Pretty()),
		Threshold:          1,
		PrivateShare:       privateShare,
		VerificationShares: verificationShares,
		PublicKey:          taproot.PublicKey{},
		ChainKey:           []byte{},
	}, threshold, peers)

	err := s.keyshareStore.StoreKeyshare(keyshare)
	s.Nil(err)

	storedKeyshare, err := s.keyshareStore.GetKeyshare()
	s.Nil(err)

	s.Equal(keyshare, storedKeyshare)
}
