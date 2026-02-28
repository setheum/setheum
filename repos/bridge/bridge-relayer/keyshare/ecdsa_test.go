// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package keyshare_test

import (
	"os"
	"testing"

	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/binance-chain/tss-lib/ecdsa/keygen"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/stretchr/testify/suite"
)

type ECDSAKeyshareStoreTestSuite struct {
	suite.Suite
	keyshareStore *keyshare.ECDSAKeyshareStore
	path          string
}

func TestRunECDSAKeyshareStoreTestSuite(t *testing.T) {
	suite.Run(t, new(ECDSAKeyshareStoreTestSuite))
}

func (s *ECDSAKeyshareStoreTestSuite) SetupTest() {
	s.path = "share.json"
	s.keyshareStore = keyshare.NewECDSAKeyshareStore(s.path)
}
func (s *ECDSAKeyshareStoreTestSuite) TearDownTest() {
	os.Remove(s.path)
}

func (s *ECDSAKeyshareStoreTestSuite) Test_RetrieveInvalidFile() {
	_, err := s.keyshareStore.GetKeyshare()
	s.NotNil(err)
}

func (s *ECDSAKeyshareStoreTestSuite) Test_StoreAndRetrieveShare() {
	threshold := 3
	peer1, _ := peer.Decode("QmZHPnN3CKiTAp8VaJqszbf8m7v4mPh15M421KpVdYHF54")
	peer2, _ := peer.Decode("QmcW3oMdSqoEcjbyd51auqC23vhKX6BqfcZcY2HJ3sKAZR")
	peers := []peer.ID{peer1, peer2}
	keyshare := keyshare.NewECDSAKeyshare(keygen.NewLocalPartySaveData(5), threshold, peers)

	err := s.keyshareStore.StoreKeyshare(keyshare)
	s.Nil(err)

	storedKeyshare, err := s.keyshareStore.GetKeyshare()
	s.Nil(err)

	s.Equal(keyshare, storedKeyshare)
}
