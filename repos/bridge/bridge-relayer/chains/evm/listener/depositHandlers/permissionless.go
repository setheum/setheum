// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers

import (
	"errors"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type PermissionlessGenericDepositHandler struct{}

// GenericDepositHandler converts data pulled from generic deposit event logs into message
func (dh *PermissionlessGenericDepositHandler) HandleDeposit(
	sourceID, destID uint8,
	nonce uint64,
	resourceID [32]byte,
	calldata, handlerResponse []byte,
	messageID string,
	timestamp time.Time) (*message.Message, error) {
	if len(calldata) < 76 {
		err := errors.New("invalid calldata length: less than 76 bytes")
		return nil, err
	}

	maxFee := calldata[:32]

	functionSigLen := big.NewInt(0).SetBytes(calldata[32:34])
	functionSigEnd := 34 + functionSigLen.Int64()
	functionSig := calldata[34:functionSigEnd]

	contractAddressLen := big.NewInt(0).SetBytes(calldata[functionSigEnd : functionSigEnd+1])
	contractAddressEnd := functionSigEnd + 1 + contractAddressLen.Int64()
	contractAddress := calldata[functionSigEnd+1 : contractAddressEnd]

	depositorLen := big.NewInt(0).SetBytes(calldata[contractAddressEnd : contractAddressEnd+1])
	depositorEnd := contractAddressEnd + 1 + depositorLen.Int64()
	depositorAddress := calldata[contractAddressEnd+1 : depositorEnd]
	executionData := calldata[depositorEnd:]

	payload := []interface{}{
		functionSig,
		contractAddress,
		maxFee,
		depositorAddress,
		executionData,
	}

	metadata := make(map[string]interface{})

	metadata["gasLimit"] = uint64(big.NewInt(0).SetBytes(maxFee).Int64())

	return message.NewMessage(
		sourceID,
		destID,
		transfer.TransferMessageData{
			DepositNonce: nonce,
			ResourceId:   resourceID,
			Metadata:     metadata,
			Payload:      payload,
			Type:         transfer.PermissionlessGenericTransfer,
		},
		messageID,
		transfer.TransferMessageType,
		timestamp,
	), nil
}
