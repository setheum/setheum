// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener

import (
	"fmt"
	"math/big"
	"strconv"
	"strings"
	"time"

	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ethereum/go-ethereum/common"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type BtcDepositHandler struct{}

// NewBtcDepositHandler creates an instance of BtcDepositHandler that contains
// handler functions for processing deposit events
func NewBtcDepositHandler() *BtcDepositHandler {
	return &BtcDepositHandler{}
}

func (e *BtcDepositHandler) HandleDeposit(
	sourceID uint8,
	depositNonce uint64,
	resourceID [32]byte,
	amount *big.Int,
	data string,
	blockNumber *big.Int,
	timestamp time.Time,
) (*message.Message, error) {
	// data is composed of recieverEVMAddress_destinationDomainID
	parsedData := strings.Split(data, "_")
	evmAdd := common.HexToAddress(parsedData[0]).Bytes()
	destDomainID, err := strconv.ParseUint(parsedData[1], 10, 8)
	if err != nil {
		return nil, err
	}

	// add 10 decimal places (8->18)
	multiplier := new(big.Int)
	multiplier.Exp(big.NewInt(10), big.NewInt(10), nil)
	amount.Mul(amount, multiplier)
	payload := []interface{}{
		amount.Bytes(),
		evmAdd,
	}

	messageID := fmt.Sprintf("%d-%d-%d", sourceID, destDomainID, blockNumber)
	return message.NewMessage(sourceID, uint8(destDomainID), transfer.TransferMessageData{
		DepositNonce: depositNonce,
		ResourceId:   resourceID,
		Metadata:     nil,
		Payload:      payload,
		Type:         transfer.FungibleTransfer,
	},
		messageID,
		transfer.TransferMessageType,
		timestamp), nil
}
