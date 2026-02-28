// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm

// MessageType represents message type identificator
type MessageType uint8

const (
	// TssKeyGenMsg message type used for communicating key generation.
	TssKeyGenMsg MessageType = iota
	// TssKeySignMsg message type used for communicating signature for specific message.
	TssKeySignMsg
	// TssInitiateMsg message type sent by the leader to signify preparation for a tss process.
	TssInitiateMsg
	// TssStartMsg message type sent by a leader to signify the start of a tss process after parties sent the ready message.
	TssStartMsg
	// TssFailMsg message type sent by parties after an communication or tss error happens during process.
	TssFailMsg
	// TssReadyMsg message type sent by coordinator sent if the process fails.
	TssReadyMsg
	// TssReshareMsg message type used for resharing tss messages.
	TssReshareMsg
	// CoordinatorElectionMsg message type used to communicate that new election process needs to start.
	CoordinatorElectionMsg
	// CoordinatorAliveMsg  message type used to respond on CoordinatorElectionMsg message, signaling that peer is alive and ready for new election process.
	CoordinatorAliveMsg
	// CoordinatorLeaveMsg message type used to communicate that peer is going offline and will not participate in the future.
	CoordinatorLeaveMsg
	// CoordinatorSelectMsg message type used to communicate that sender has pronounced itself as a leader.
	CoordinatorSelectMsg
	// CoordinatorPingMsg message type used to check if the current coordinator is alive.
	CoordinatorPingMsg
	// CoordinatorPingResponseMsg message type used to respond on CoordinatorPingMsg message.
	CoordinatorPingResponseMsg
	// Unknown message type
	Unknown
)

// String implements fmt.Stringer
func (msgType MessageType) String() string {
	switch msgType {
	case TssKeyGenMsg:
		return "TssKeyGenMsg"
	case TssKeySignMsg:
		return "TssKeySignMsg"
	case TssInitiateMsg:
		return "TssInitiateMsg"
	case TssStartMsg:
		return "TssStartMsg"
	case TssFailMsg:
		return "TssFailMsg"
	case TssReadyMsg:
		return "TssReadyMsg"
	case TssReshareMsg:
		return "TssReshareMsg"
	case CoordinatorElectionMsg:
		return "CoordinatorElectionMsg"
	case CoordinatorAliveMsg:
		return "CoordinatorAliveMsg"
	case CoordinatorLeaveMsg:
		return "CoordinatorLeaveMsg"
	case CoordinatorSelectMsg:
		return "CoordinatorSelectMsg"
	case CoordinatorPingMsg:
		return "CoordinatorPingMsg"
	case CoordinatorPingResponseMsg:
		return "CoordinatorPingResponseMsg"
	default:
		return "UnknownMsg"
	}
}
