// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers

import (
	"errors"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ethereum/go-ethereum/common"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

const OPTIONAL_REVERT_GAS = 100000

type Erc20DepositHandler struct{}

// Erc20DepositHandler converts data pulled from event logs into message
// handlerResponse can be an empty slice
func (dh *Erc20DepositHandler) HandleDeposit(
	sourceID,
	destID uint8,
	nonce uint64,
	resourceID [32]byte,
	calldata,
	handlerResponse []byte,
	messageID string,
	timestamp time.Time) (*message.Message, error) {
	if len(calldata) < 84 {
		err := errors.New("invalid calldata length: less than 84 bytes")
		return nil, err
	}

	// amount: first 32 bytes of calldata
	// handlerResponse: 32 bytes amount if handler converted amounts
	var amount []byte
	if len(handlerResponse) > 0 {
		amount = handlerResponse[:32]
	} else {
		amount = calldata[:32]
	}

	recipientAddressLength := big.NewInt(0).SetBytes(calldata[32:64])
	recipientAddress := calldata[64:(64 + recipientAddressLength.Int64())]
	payload := []interface{}{
		amount,
		recipientAddress,
	}

	metadata := make(map[string]interface{})
	// append optional message if it exists
	if len(calldata) > int(96+recipientAddressLength.Int64()) {
		maxFee := new(big.Int).Add(new(big.Int).SetBytes(calldata[64+recipientAddressLength.Int64():96+recipientAddressLength.Int64()]), big.NewInt(OPTIONAL_REVERT_GAS))
		metadata["gasLimit"] = maxFee.Uint64()

		copy(calldata[64+recipientAddressLength.Int64():96+recipientAddressLength.Int64()], common.LeftPadBytes(maxFee.Bytes(), 32))
		payload = append(payload, calldata[64+recipientAddressLength.Int64():])
	}

	return message.NewMessage(
		sourceID,
		destID,
		transfer.TransferMessageData{
			DepositNonce: nonce,
			ResourceId:   resourceID,
			Metadata:     metadata,
			Payload:      payload,
			Type:         transfer.FungibleTransfer,
		},
		messageID,
		transfer.TransferMessageType,
		timestamp,
	), nil
}
