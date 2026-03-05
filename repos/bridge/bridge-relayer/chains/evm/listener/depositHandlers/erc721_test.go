// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package depositHandlers_test

import (
	"math/big"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/depositHandlers"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/math"
	"github.com/stretchr/testify/suite"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

type Erc721HandlerTestSuite struct {
	suite.Suite
}

func TestRunErc721HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(Erc721HandlerTestSuite))
}

func (s *Erc721HandlerTestSuite) TestErc721DepositHandlerEmptyMetadata() {
	recipient := common.HexToAddress("0xf1e58fb17704c2da8479a533f9fad4ad0993ca6b")

	calldata := evm.ConstructErc721DepositData(recipient.Bytes(), big.NewInt(2), []byte{})
	timestamp := time.Now()
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		Data:                calldata,
		HandlerResponse:     []byte{},
		Timestamp:           timestamp,
	}

	sourceID := uint8(1)
	tokenId := calldata[:32]
	recipientAddressParsed := calldata[64:84]
	var metadata []byte

	expected := &message.Message{
		Source:      sourceID,
		Destination: depositLog.DestinationDomainID,
		Data: transfer.TransferMessageData{
			DepositNonce: depositLog.DepositNonce,
			ResourceId:   depositLog.ResourceID,
			Payload: []interface{}{
				tokenId,
				recipientAddressParsed,
				metadata,
			},
			Type: transfer.NonFungibleTransfer,
		},

		Type:      transfer.TransferMessageType,
		ID:        "messageID",
		Timestamp: timestamp,
	}

	erc721DepositHandler := depositHandlers.Erc721DepositHandler{}
	m, err := erc721DepositHandler.HandleDeposit(
		sourceID,
		depositLog.DestinationDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.Data,
		depositLog.HandlerResponse,
		"messageID",
		timestamp)

	s.Nil(err)
	s.NotNil(m)
	s.Equal(expected, m)
}

func (s *Erc721HandlerTestSuite) TestErc721DepositHandlerIncorrectDataLen() {
	metadata := []byte("0xdeadbeef")

	var calldata []byte
	calldata = append(calldata, math.PaddedBigBytes(big.NewInt(int64(len(metadata))), 16)...)
	calldata = append(calldata, metadata...)

	timestamp := time.Now()
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		SenderAddress:       common.HexToAddress("0x4CEEf6139f00F9F4535Ad19640Ff7A0137708485"),
		Data:                calldata,
		HandlerResponse:     []byte{},
		Timestamp:           timestamp,
	}

	sourceID := uint8(1)

	erc721DepositHandler := depositHandlers.Erc721DepositHandler{}
	m, err := erc721DepositHandler.HandleDeposit(
		sourceID,
		depositLog.DestinationDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.Data,
		depositLog.HandlerResponse,
		"messageID",
		timestamp)
	s.Nil(m)
	s.EqualError(err, "invalid calldata length: less than 84 bytes")
}

func (s *Erc721HandlerTestSuite) TestErc721DepositHandler() {
	recipient := common.HexToAddress("0xf1e58fb17704c2da8479a533f9fad4ad0993ca6b")
	metadata := []byte("metadata.url")

	calldata := evm.ConstructErc721DepositData(recipient.Bytes(), big.NewInt(2), metadata)
	timestamp := time.Now()
	depositLog := &events.Deposit{
		DestinationDomainID: 0,
		ResourceID:          [32]byte{0},
		DepositNonce:        1,
		Data:                calldata,
		HandlerResponse:     []byte{},
		Timestamp:           timestamp,
	}

	sourceID := uint8(1)
	tokenId := calldata[:32]
	recipientAddressParsed := calldata[64:84]
	parsedMetadata := calldata[116:128]

	expected := &message.Message{
		Source:      sourceID,
		Destination: depositLog.DestinationDomainID,
		Data: transfer.TransferMessageData{
			DepositNonce: depositLog.DepositNonce,
			ResourceId:   depositLog.ResourceID,
			Payload: []interface{}{
				tokenId,
				recipientAddressParsed,
				parsedMetadata,
			},
			Type: transfer.NonFungibleTransfer,
		},

		Type:      transfer.TransferMessageType,
		ID:        "messageID",
		Timestamp: timestamp,
	}

	erc721DepositHandler := depositHandlers.Erc721DepositHandler{}
	m, err := erc721DepositHandler.HandleDeposit(
		sourceID,
		depositLog.DestinationDomainID,
		depositLog.DepositNonce,
		depositLog.ResourceID,
		depositLog.Data,
		depositLog.HandlerResponse,
		"messageID",
		timestamp)
	s.Nil(err)
	s.NotNil(m)
	s.Equal(expected, m)
}
