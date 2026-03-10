// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers

import (
	"time"

	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type Erc1155DepositHandler struct{}

func (dh *Erc1155DepositHandler) HandleDeposit(
	sourceID,
	destID uint8,
	nonce uint64,
	resourceID [32]byte,
	calldata, handlerResponse []byte,
	messageID string,
	timestamp time.Time) (*message.Message, error) {
	erc1155Type, err := GetErc1155Type()
	if err != nil {
		return nil, err
	}

	decodedCallData, err := erc1155Type.UnpackValues(calldata)
	if err != nil {
		return nil, err
	}

	payload := []interface{}{
		decodedCallData[0],
		decodedCallData[1],
		decodedCallData[2],
		decodedCallData[3],
	}

	return message.NewMessage(
		sourceID,
		destID,
		transfer.TransferMessageData{
			DepositNonce: nonce,
			ResourceId:   resourceID,
			Metadata:     nil,
			Payload:      payload,
			Type:         transfer.SemiFungibleTransfer,
		},
		messageID,
		transfer.TransferMessageType,
		timestamp,
	), nil
}

func GetErc1155Type() (abi.Arguments, error) {
	tokenIDsType, err := abi.NewType("uint256[]", "", nil)
	if err != nil {
		return nil, err
	}

	amountsType, err := abi.NewType("uint256[]", "", nil)
	if err != nil {
		return nil, err
	}

	recipientType, err := abi.NewType("bytes", "", nil)
	if err != nil {
		return nil, err
	}

	transferDataType, err := abi.NewType("bytes", "", nil)
	if err != nil {
		return nil, err
	}

	// Define the arguments using the created types
	return abi.Arguments{
		abi.Argument{Name: "tokenIDs", Type: tokenIDsType, Indexed: false},
		abi.Argument{Name: "amounts", Type: amountsType, Indexed: false},
		abi.Argument{Name: "recipient", Type: recipientType, Indexed: false},
		abi.Argument{Name: "transferData", Type: transferDataType, Indexed: false},
	}, nil
}
