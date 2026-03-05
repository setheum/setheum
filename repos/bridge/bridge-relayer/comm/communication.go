// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm

import (
	"github.com/libp2p/go-libp2p/core/peer"
)

// WrappedMessage is a structure representing a raw message that is sent trough Communication
type WrappedMessage struct {
	MessageType MessageType `json:"message_type"`
	SessionID   string      `json:"message_id"`
	Payload     []byte      `json:"payload"`
	From        peer.ID     `json:"-"`
}

// Communication defines methods for communicating between peers
type Communication interface {
	// CloseSession closes and removes all streams for that session
	CloseSession(sessionID string)
	// Broadcast sends message to provided peers
	// If error has occurred on sending any message, broadcast will be aborted and error will be sent to errChan
	Broadcast(peers peer.IDSlice, msg []byte, msgType MessageType, sessionID string) error
	// Subscribe subscribes provided channel to a specific message type for a provided session
	// Returns SubscriptionID - unique identifier of created subscription that is used to unsubscribe from subscription
	Subscribe(sessionID string, msgType MessageType, channel chan *WrappedMessage) SubscriptionID
	// UnSubscribe unsubscribes from subscription defined by provided SubscriptionID
	UnSubscribe(subID SubscriptionID)
}
