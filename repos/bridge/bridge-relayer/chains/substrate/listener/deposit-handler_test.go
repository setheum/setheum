// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener_test

import (
	"errors"
	"time"
	"unsafe"

	"github.com/sygmaprotocol/sygma-core/relayer/message"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"

	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/substrate/events"
	"github.com/ChainSafe/sygma-relayer/chains/substrate/listener"
	"github.com/ChainSafe/sygma-relayer/e2e/substrate"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/stretchr/testify/suite"
)

var errNoCorrespondingDepositHandler = errors.New("no corresponding deposit handler for this transfer type exists")
var errIncorrectDataLen = errors.New("invalid calldata length: less than 84 bytes")

type Erc20HandlerTestSuite struct {
	suite.Suite
}

func TestRunErc20HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(Erc20HandlerTestSuite))
}

func (s *Erc20HandlerTestSuite) TestErc20HandleEvent() {
	recipientAddr := *(*[]types.U8)(unsafe.Pointer(&substrate.SubstratePK.PublicKey))
	recipient := substrate.ConstructRecipientData(recipientAddr)

	var calldata []byte
	amount, _ := types.BigIntToIntBytes(big.NewInt(2), 32)
	calldata = append(calldata, amount...)
	recipientLen, _ := (types.BigIntToIntBytes(big.NewInt(int64(len(recipient))), 32))
	calldata = append(calldata, recipientLen...)
	calldata = append(calldata, types.Bytes(recipient)...)

	timestamp := time.Now()
	depositLog := &events.Deposit{
		DestDomainID: types.NewU8(2),
		ResourceID:   types.Bytes32{1},
		DepositNonce: types.NewU64(1),
		TransferType: types.NewU8(0),
		CallData:     calldata,
		Handler:      [1]byte{0},
		Timestamp:    timestamp,
	}

	sourceID := uint8(1)
	amountParsed := calldata[:32]
	recipientAddressParsed := calldata[64:]

	expected := &message.Message{
		Source:      sourceID,
		Destination: uint8(depositLog.DestDomainID),
		Data: transfer.TransferMessageData{
			DepositNonce: uint64(depositLog.DepositNonce),
			ResourceId:   depositLog.ResourceID,
			Payload: []interface{}{
				amountParsed,
				recipientAddressParsed,
			},
			Type: transfer.FungibleTransfer,
		},
		Type:      transfer.TransferMessageType,
		ID:        "messageID",
		Timestamp: timestamp,
	}

	message, err := listener.FungibleTransferHandler(
		sourceID,
		depositLog.DestDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.CallData,
		"messageID",
		timestamp)

	s.Nil(err)
	s.NotNil(message)
	s.Equal(message, expected)
}

func (s *Erc20HandlerTestSuite) TestErc20HandleEventIncorrectdeposit_dataLen() {
	var calldata []byte

	depositLog := &events.Deposit{
		DestDomainID: types.NewU8(2),
		ResourceID:   types.Bytes32{1},
		DepositNonce: types.NewU64(1),
		TransferType: types.NewU8(0),
		CallData:     calldata,
		Handler:      [1]byte{0},
	}

	sourceID := uint8(1)

	message, err := listener.FungibleTransferHandler(
		sourceID,
		depositLog.DestDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.CallData,
		"messageID",
		time.Now())
	s.Nil(message)
	s.EqualError(err, errIncorrectDataLen.Error())
}

func (s *Erc20HandlerTestSuite) TestSuccesfullyRegisterFungibleTransferHandler() {
	recipientAddr := *(*[]types.U8)(unsafe.Pointer(&substrate.SubstratePK.PublicKey))
	recipient := substrate.ConstructRecipientData(recipientAddr)
	// Create calldata
	var calldata []byte
	amount, _ := types.BigIntToIntBytes(big.NewInt(2), 32)
	calldata = append(calldata, amount...)
	recipientLen, _ := (types.BigIntToIntBytes(big.NewInt(int64(len(recipient))), 32))
	calldata = append(calldata, recipientLen...)
	calldata = append(calldata, types.Bytes(recipient)...)

	timestamp := time.Now()
	d1 := &events.Deposit{
		DestDomainID: types.NewU8(2),
		ResourceID:   types.Bytes32{1},
		DepositNonce: types.NewU64(1),
		TransferType: types.NewU8(0),
		CallData:     calldata,
		Timestamp:    timestamp,
		Handler:      [1]byte{0},
	}

	depositHandler := listener.NewSubstrateDepositHandler()
	// Register FungibleTransferHandler function
	depositHandler.RegisterDepositHandler(transfer.FungibleTransfer, listener.FungibleTransferHandler)
	message1, err1 := depositHandler.HandleDeposit(1, d1.DestDomainID, d1.DepositNonce, d1.ResourceID, d1.CallData, d1.TransferType, "messageID", d1.Timestamp)
	s.Nil(err1)
	s.NotNil(message1)

	// Use unregistered transfer type
	message2, err2 := depositHandler.HandleDeposit(1, d1.DestDomainID, d1.DepositNonce, d1.ResourceID, d1.CallData, 1, "messageID", d1.Timestamp)
	s.Nil(message2)
	s.NotNil(err2)
	s.EqualError(err2, errNoCorrespondingDepositHandler.Error())
}
