// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener_test

import (
	"fmt"
	"strings"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/sygmaprotocol/sygma-core/relayer/message"

	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/btc/listener"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/stretchr/testify/suite"
)

type Erc20HandlerTestSuite struct {
	suite.Suite
}

func TestRunErc20HandlerTestSuite(t *testing.T) {
	suite.Run(t, new(Erc20HandlerTestSuite))
}

func (s *Erc20HandlerTestSuite) Test_Erc20HandleEvent() {
	deposit := &listener.Deposit{
		SenderAddress: "senderAddress",
		ResourceID:    [32]byte{0},
		Amount:        big.NewInt(100),
		Data:          "0x1c3A03D04c026b1f4B4208D2ce053c5686E6FB8d_1",
	}

	sourceID := uint8(1)

	expectedAmount := big.NewInt(1000000000000)
	blockNumber := big.NewInt(100)
	depositNonce := uint64(1)
	dat := strings.Split(deposit.Data, "_")
	evmAdd := common.HexToAddress(dat[0]).Bytes()
	messageID := fmt.Sprintf("%d-%d-%d", sourceID, 1, blockNumber)

	timestamp := time.Now()
	expected := &message.Message{
		Source:      sourceID,
		Destination: uint8(1),
		Data: transfer.TransferMessageData{
			DepositNonce: depositNonce,
			ResourceId:   deposit.ResourceID,
			Payload: []interface{}{
				expectedAmount.Bytes(),
				evmAdd,
			},
			Type: transfer.FungibleTransfer,
		},
		Type:      transfer.TransferMessageType,
		ID:        messageID,
		Timestamp: timestamp,
	}

	btcDepositHandler := listener.NewBtcDepositHandler()
	message, err := btcDepositHandler.HandleDeposit(sourceID, depositNonce, deposit.ResourceID, deposit.Amount, deposit.Data, blockNumber, timestamp)

	s.Nil(err)
	s.NotNil(message)
	s.Equal(message, expected)
}

func (s *Erc20HandlerTestSuite) Test_Erc20HandleEvent_InvalidDestinationDomainID() {

	deposit := &listener.Deposit{
		SenderAddress: "senderAddress",
		ResourceID:    [32]byte{0},
		Amount:        big.NewInt(100),
		Data:          "0x1c3A03D04c026b1f4B4208D2ce053c5686E6FB8d_InvalidDestinationDomainID",
	}

	sourceID := uint8(1)

	blockNumber := big.NewInt(100)
	depositNonce := uint64(1)

	btcDepositHandler := listener.NewBtcDepositHandler()
	message, err := btcDepositHandler.HandleDeposit(sourceID, depositNonce, deposit.ResourceID, deposit.Amount, deposit.Data, blockNumber, time.Now())

	s.Nil(message)
	s.NotNil(err)
}
