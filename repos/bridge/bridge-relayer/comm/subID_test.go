// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm

import (
	"github.com/stretchr/testify/suite"
	"testing"
)

type SubscriptionIDTestSuite struct {
	suite.Suite
}

func TestRunSubscriptionIDTestSuite(t *testing.T) {
	suite.Run(t, new(SubscriptionIDTestSuite))
}

func (s *SubscriptionIDTestSuite) SetupSuite()    {}
func (s *SubscriptionIDTestSuite) TearDownSuite() {}
func (s *SubscriptionIDTestSuite) SetupTest()     {}
func (s *SubscriptionIDTestSuite) TearDownTest()  {}

func (s *SubscriptionIDTestSuite) TestSubscriptionID_UniqueIDs() {
	subID1 := NewSubscriptionID("1", CoordinatorPingMsg)
	subID2 := NewSubscriptionID("1", CoordinatorPingMsg)
	subID3 := NewSubscriptionID("1", CoordinatorPingMsg)

	s.NotEqual(subID1, subID2)
	s.NotEqual(subID1, subID3)
	s.NotEqual(subID2, subID3)
}

func (s *SubscriptionIDTestSuite) TestSubscriptionID_UnwrapValidID_Success() {
	subID := NewSubscriptionID("1", CoordinatorPingMsg)

	sessionID, msgType, id, err := subID.Unwrap()

	s.Nil(err)
	s.Equal("1", sessionID)
	s.Equal(CoordinatorPingMsg, msgType)
	s.NotNil(id)
}

func (s *SubscriptionIDTestSuite) TestSubscriptionID_UnwrapInvalidID_Success() {
	invalidSubIDs := []SubscriptionID{
		"not-id",
		"almost-sub-id",
		"1-23-1212", // invalid message type
	}

	for _, id := range invalidSubIDs {
		_, _, _, err := id.Unwrap()
		s.NotNil(err)
	}
}
