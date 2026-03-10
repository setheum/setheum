// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers_test

import (
	"errors"
	"math/big"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/sygmaprotocol/sygma-core/relayer/message"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/depositHandlers"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/math"
	"github.com/stretchr/testify/suite"
)

var errIncorrectDataLen = errors.New("invalid calldata length: less than 84 bytes")

type Erc20HandlerTestSuite struct {
	suite.Suite
}

func TestRunErc20HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(Erc20HandlerTestSuite))
}

func (s *Erc20HandlerTestSuite) TestErc20HandleEvent() {
	// 0xf1e58fb17704c2da8479a533f9fad4ad0993ca6b
	recipientByteSlice := []byte{241, 229, 143, 177, 119, 4, 194, 218, 132, 121, 165, 51, 249, 250, 212, 173, 9, 147, 202, 107}
	maxFee := big.NewInt(200000)
	optionalMessage := common.LeftPadBytes(maxFee.Bytes(), 32)
	optionalMessage = append(optionalMessage, []byte("optionalMessage")...)

	optionalMessageWithRevertGas := common.LeftPadBytes(new(big.Int).Add(maxFee, big.NewInt(depositHandlers.OPTIONAL_REVERT_GAS)).Bytes(), 32)
	optionalMessageWithRevertGas = append(optionalMessageWithRevertGas, []byte("optionalMessage")...)

	metadata := make(map[string]interface{})
	metadata["gasLimit"] = uint64(300000)

	calldata := evm.ConstructErc20DepositData(recipientByteSlice, big.NewInt(2))
	calldata = append(calldata, optionalMessage...)
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		SenderAddress:       common.HexToAddress("0x4CEEf6139f00F9F4535Ad19640Ff7A0137708485"),
		Data:                calldata,
		HandlerResponse:     []byte{},
	}

	sourceID := uint8(1)
	amountParsed := calldata[:32]
	recipientAddressParsed := calldata[64 : 64+len(recipientByteSlice)]

	timestamp := time.Now()
	expected := &message.Message{
		Source:      sourceID,
		Destination: depositLog.DestinationDomainID,
		Data: transfer.TransferMessageData{
			DepositNonce: depositLog.DepositNonce,
			ResourceId:   depositLog.ResourceID,
			Payload: []interface{}{
				amountParsed,
				recipientAddressParsed,
				optionalMessageWithRevertGas,
			},
			Type:     transfer.FungibleTransfer,
			Metadata: metadata,
		},
		Type:      transfer.TransferMessageType,
		ID:        "messageID",
		Timestamp: timestamp,
	}
	erc20DepositHandler := depositHandlers.Erc20DepositHandler{}
	message, err := erc20DepositHandler.HandleDeposit(
		sourceID,
		depositLog.DestinationDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.Data,
		depositLog.HandlerResponse,
		"messageID",
		timestamp,
	)

	s.Nil(err)
	s.NotNil(message)
	s.Equal(message, expected)
}

func (s *Erc20HandlerTestSuite) TestErc20HandleEventIncorrectDataLen() {
	metadata := []byte("0xdeadbeef")

	var calldata []byte
	calldata = append(calldata, math.PaddedBigBytes(big.NewInt(int64(len(metadata))), 32)...)
	calldata = append(calldata, metadata...)

	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		SenderAddress:       common.HexToAddress("0x4CEEf6139f00F9F4535Ad19640Ff7A0137708485"),
		Data:                calldata,
		HandlerResponse:     []byte{},
	}

	sourceID := uint8(1)

	erc20DepositHandler := depositHandlers.Erc20DepositHandler{}
	message, err := erc20DepositHandler.HandleDeposit(
		sourceID,
		depositLog.DestinationDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.Data,
		depositLog.HandlerResponse,
		"messageID",
		time.Now(),
	)

	s.Nil(message)
	s.EqualError(err, errIncorrectDataLen.Error())
}
