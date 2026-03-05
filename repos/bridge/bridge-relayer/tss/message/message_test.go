// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package message_test

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/tss/message"
	"github.com/stretchr/testify/suite"
)

type TssMessageTestSuite struct {
	suite.Suite
}

func TestRunTssMessageTestSuite(t *testing.T) {
	suite.Run(t, new(TssMessageTestSuite))
}

func (s *TssMessageTestSuite) Test_UnmarshaledMessageShouldBeEqual() {
	originalMsg := &message.TssMessage{
		MsgBytes:    []byte{1},
		IsBroadcast: true,
	}
	msgBytes, err := message.MarshalTssMessage(originalMsg.MsgBytes, originalMsg.IsBroadcast)
	s.Nil(err)

	unmarshaledMsg, err := message.UnmarshalTssMessage(msgBytes)
	s.Nil(err)

	s.Equal(originalMsg, unmarshaledMsg)
}

type StartMessageTestSuite struct {
	suite.Suite
}

func TestRunStartMessageTestSuite(t *testing.T) {
	suite.Run(t, new(StartMessageTestSuite))
}

func (s *StartMessageTestSuite) Test_UnmarshaledMessageShouldBeEqual() {
	originalMsg := &message.StartMessage{
		Params: []byte("test"),
	}
	msgBytes, err := message.MarshalStartMessage(originalMsg.Params)
	s.Nil(err)

	unmarshaledMsg, err := message.UnmarshalStartMessage(msgBytes)
	s.Nil(err)

	s.Equal(originalMsg, unmarshaledMsg)
}
