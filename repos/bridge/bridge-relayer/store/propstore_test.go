package store_test

import (
	"errors"
	"testing"

	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/stretchr/testify/suite"
	mock_store "github.com/sygmaprotocol/sygma-core/mock"
	"github.com/syndtr/goleveldb/leveldb"
	"go.uber.org/mock/gomock"
)

type PropStoreTestSuite struct {
	suite.Suite
	nonceStore           *store.PropStore
	keyValueReaderWriter *mock_store.MockKeyValueReaderWriter
}

func TestRunPropStoreTestSuite(t *testing.T) {
	suite.Run(t, new(PropStoreTestSuite))
}

func (s *PropStoreTestSuite) SetupTest() {
	gomockController := gomock.NewController(s.T())
	s.keyValueReaderWriter = mock_store.NewMockKeyValueReaderWriter(gomockController)
	s.nonceStore = store.NewPropStore(s.keyValueReaderWriter)
}

func (s *PropStoreTestSuite) Test_StorePropStatus_FailedStore() {
	key := "source:1:destination:2:depositNonce:3"
	s.keyValueReaderWriter.EXPECT().SetByKey([]byte(key), []byte(store.ExecutedProp)).Return(errors.New("error"))

	err := s.nonceStore.StorePropStatus(1, 2, 3, store.ExecutedProp)

	s.NotNil(err)
}

func (s *PropStoreTestSuite) TestStoreBlock_SuccessfulStore() {
	key := "source:1:destination:2:depositNonce:3"
	s.keyValueReaderWriter.EXPECT().SetByKey([]byte(key), []byte(store.ExecutedProp)).Return(nil)

	err := s.nonceStore.StorePropStatus(1, 2, 3, store.ExecutedProp)

	s.Nil(err)
}

func (s *PropStoreTestSuite) Test_GetPropStatus_FailedFetch() {
	key := "source:1:destination:2:depositNonce:3"
	s.keyValueReaderWriter.EXPECT().GetByKey([]byte(key)).Return(nil, errors.New("error"))

	_, err := s.nonceStore.PropStatus(1, 2, 3)

	s.NotNil(err)
}

func (s *PropStoreTestSuite) TestGetNonce_NonceNotFound() {
	key := "source:1:destination:2:depositNonce:3"
	s.keyValueReaderWriter.EXPECT().GetByKey([]byte(key)).Return(nil, leveldb.ErrNotFound)

	status, err := s.nonceStore.PropStatus(1, 2, 3)

	s.Nil(err)
	s.Equal(status, store.MissingProp)
}

func (s *PropStoreTestSuite) TestGetNonce_SuccessfulFetch() {
	key := "source:1:destination:2:depositNonce:3"
	s.keyValueReaderWriter.EXPECT().GetByKey([]byte(key)).Return([]byte(store.ExecutedProp), nil)

	status, err := s.nonceStore.PropStatus(1, 2, 3)

	s.Nil(err)
	s.Equal(status, store.ExecutedProp)
}
