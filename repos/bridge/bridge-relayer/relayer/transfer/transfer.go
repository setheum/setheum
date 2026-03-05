// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package transfer

import (
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type TransferType string

const (
	FungibleTransfer              TransferType = "fungible"
	SemiFungibleTransfer          TransferType = "semiFungible"
	NonFungibleTransfer           TransferType = "nonFungible"
	PermissionedGenericTransfer   TransferType = "permissionedGeneric"
	PermissionlessGenericTransfer TransferType = "permissionlessGeneric"
)

type TransferMessageData struct {
	DepositNonce uint64
	ResourceId   [32]byte
	Metadata     map[string]interface{}
	Payload      []interface{}
	Type         TransferType
}

const (
	TransferMessageType  message.MessageType   = "TransferMessage"
	TransferProposalType proposal.ProposalType = "TransferProposal"
)

type TransferMessage struct {
	Source      uint8
	Destination uint8
	Data        TransferMessageData
	Type        message.MessageType
	ID          string
}

type TransferProposalData struct {
	DepositNonce uint64
	ResourceId   [32]byte
	Metadata     map[string]interface{}
	Data         []byte
}

type TransferProposal struct {
	Source      uint8
	Destination uint8
	Data        TransferProposalData
	Type        proposal.ProposalType
	MessageID   string
}
