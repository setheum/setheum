// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers_test

import (
	"encoding/hex"
	"math/big"
	"testing"
	"time"

	"github.com/sygmaprotocol/sygma-core/relayer/message"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"

	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/depositHandlers"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/math"
	"github.com/stretchr/testify/suite"
)

type PermissionlessGenericHandlerTestSuite struct {
	suite.Suite
}

func TestRunPermissionlessGenericHandlerTestSuite(t *testing.T) {
	suite.Run(t, new(PermissionlessGenericHandlerTestSuite))
}

func (s *PermissionlessGenericHandlerTestSuite) TestHandleEvent() {
	hash := []byte("0xhash")
	functionSig, _ := hex.DecodeString("654cf88c")
	contractAddress := common.HexToAddress("0x02091EefF969b33A5CE8A729DaE325879bf76f90")
	depositor := common.HexToAddress("0x5C1F5961696BaD2e73f73417f07EF55C62a2dC5b")
	maxFee := big.NewInt(200000)
	var executionData []byte
	executionData = append(executionData, hash[:]...)
	metadata := make(map[string]interface{})
	metadata["gasLimit"] = uint64(200000)

	calldata := evm.ConstructPermissionlessGenericDepositData(
		hash[:],
		functionSig,
		contractAddress.Bytes(),
		depositor.Bytes(),
		maxFee,
	)
	timestamp := time.Now()
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		SenderAddress:       common.HexToAddress("0x5C1F5961696BaD2e73f73417f07EF55C62a2dC5b"),
		Data:                calldata,
		HandlerResponse:     []byte{},
		Timestamp:           timestamp,
	}

	sourceID := uint8(1)
	expected := &message.Message{
		Source:      sourceID,
		Destination: depositLog.DestinationDomainID,
		Data: transfer.TransferMessageData{
			DepositNonce: depositLog.DepositNonce,
			ResourceId:   depositLog.ResourceID,
			Payload: []interface{}{
				functionSig,
				contractAddress.Bytes(),
				common.LeftPadBytes(maxFee.Bytes(), 32),
				depositor.Bytes(),
				executionData,
			},
			Metadata: metadata,
			Type:     transfer.PermissionlessGenericTransfer,
		},
		Type:      transfer.TransferMessageType,
		ID:        "messageID",
		Timestamp: timestamp,
	}

	permissionlessGenericHandler := depositHandlers.PermissionlessGenericDepositHandler{}

	message, err := permissionlessGenericHandler.HandleDeposit(
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

func (s *PermissionlessGenericHandlerTestSuite) Test_HandleEvent_IncorrectDataLen() {
	metadata := make(map[string]interface{})
	metadata["gasLimit"] = uint64(200000)
	var calldata []byte
	calldata = append(calldata, math.PaddedBigBytes(big.NewInt(int64(len(metadata))), 32)...)
	timestamp := time.Now()
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		SenderAddress:       common.HexToAddress("0x5C1F5961696BaD2e73f73417f07EF55C62a2dC5b"),
		Data:                calldata,
		HandlerResponse:     []byte{},
		Timestamp:           timestamp,
	}

	sourceID := uint8(1)

	permissionlessGenericHandler := depositHandlers.PermissionlessGenericDepositHandler{}
	_, err := permissionlessGenericHandler.HandleDeposit(
		sourceID,
		depositLog.DestinationDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.Data,
		depositLog.HandlerResponse,
		"messageID",
		timestamp)

	s.NotNil(err)
}
