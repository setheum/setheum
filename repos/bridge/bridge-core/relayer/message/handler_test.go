package message_test

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/suite"
	"github.com/sygmaprotocol/sygma-core/mock"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
	"go.uber.org/mock/gomock"
)

type MessageHandlerTestSuite struct {
	suite.Suite

	mockHandler *mock.MockHandler
}

func TestRunMessageHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(MessageHandlerTestSuite))
}

func (s *MessageHandlerTestSuite) SetupTest() {
	gomockController := gomock.NewController(s.T())
	s.mockHandler = mock.NewMockHandler(gomockController)
}

func (s *MessageHandlerTestSuite) TestHandleMessageWithoutRegisteredHandler() {
	mh := message.NewMessageHandler()

	_, err := mh.HandleMessage(&message.Message{Type: "invalid"})

	s.NotNil(err)
}

func (s *MessageHandlerTestSuite) TestHandleMessageWithInvalidType() {
	mh := message.NewMessageHandler()
	mh.RegisterMessageHandler("invalid", s.mockHandler)

	_, err := mh.HandleMessage(&message.Message{Type: "valid"})

	s.NotNil(err)
}

func (s *MessageHandlerTestSuite) TestHandleMessageHandlerReturnsError() {
	s.mockHandler.EXPECT().HandleMessage(gomock.Any()).Return(nil, fmt.Errorf("error"))

	mh := message.NewMessageHandler()
	mh.RegisterMessageHandler("valid", s.mockHandler)

	_, err := mh.HandleMessage(&message.Message{Type: "valid"})

	s.NotNil(err)
}

func (s *MessageHandlerTestSuite) TestHandleMessageWithValidType() {
	expectedProp := &proposal.Proposal{
		Type: "prop",
	}
	s.mockHandler.EXPECT().HandleMessage(gomock.Any()).Return(expectedProp, nil)

	mh := message.NewMessageHandler()
	mh.RegisterMessageHandler("valid", s.mockHandler)

	msg := &message.Message{
		Source:      1,
		Destination: 2,
		Data:        nil,
		Type:        "valid",
	}
	prop, err := mh.HandleMessage(msg)

	s.Nil(err)
	s.Equal(prop, expectedProp)
}
