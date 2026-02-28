package message

import (
	"fmt"

	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type Handler interface {
	HandleMessage(m *Message) (*proposal.Proposal, error)
}

type MessageHandler struct {
	handlers map[MessageType]Handler
}

func NewMessageHandler() *MessageHandler {
	return &MessageHandler{
		handlers: make(map[MessageType]Handler),
	}
}

// HandlerMessage calls associated handler for that message type and returns a proposal to be submitted on-chain
func (h *MessageHandler) HandleMessage(m *Message) (*proposal.Proposal, error) {
	mh, ok := h.handlers[m.Type]
	if !ok {
		return nil, fmt.Errorf("no handler found for type %s", m.Type)
	}
	return mh.HandleMessage(m)
}

// RegisterMessageHandler registers a message handler by associating a handler to a message type
func (mh *MessageHandler) RegisterMessageHandler(t MessageType, h Handler) {
	mh.handlers[t] = h
}
