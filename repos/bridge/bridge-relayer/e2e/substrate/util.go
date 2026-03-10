// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package substrate

import (
	"bytes"
	"context"
	"encoding/binary"
	"errors"
	"math/big"
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	substrateTypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/connection"

	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/gas"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	ethereumTypes "github.com/ethereum/go-ethereum/core/types"
)

var TestTimeout = time.Minute * 4
var BasicFee = big.NewInt(100000000000000)
var GasUsed = uint32(2000000000)
var SubstratePK = signature.KeyringPair{
	URI:       "//Alice",
	PublicKey: []byte{0xd4, 0x35, 0x93, 0xc7, 0x15, 0xfd, 0xd3, 0x1c, 0x61, 0x14, 0x1a, 0xbd, 0x4, 0xa9, 0x9f, 0xd6, 0x82, 0x2c, 0x85, 0x58, 0x85, 0x4c, 0xcd, 0xe3, 0x9a, 0x56, 0x84, 0xe7, 0xa5, 0x6d, 0xa2, 0x7d},
	Address:   "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
}

const (
	SubstrateEndpoint = "ws://localhost:9944"
)

type USDCAsset struct{}

func (a USDCAsset) Encode(encoder scale.Encoder) error {
	_ = encoder.Write([]byte{0, 1, 3, 0, 85, 31, 6, 5, 115, 121, 103, 109, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 4, 117, 115, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4})
	return nil
}

func (a USDCAsset) Decode(decoder scale.Decoder) error {
	return nil
}

type Destination struct {
}

func (a Destination) Encode(encoder scale.Encoder) error {
	_ = encoder.Write([]byte{0, 2, 6, 20, 92, 31, 89, 97, 105, 107, 173, 46, 115, 247, 52, 23, 240, 126, 245, 92, 98, 162, 220, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0})
	return nil
}

func (a Destination) Decode(decoder scale.Decoder) error {
	return nil
}

type Client interface {
	LatestBlock() (*big.Int, error)
	SubscribeFilterLogs(ctx context.Context, q ethereum.FilterQuery, ch chan<- ethereumTypes.Log) (ethereum.Subscription, error)
	FetchEventLogs(ctx context.Context, contractAddress common.Address, event string, startBlock *big.Int, endBlock *big.Int) ([]ethereumTypes.Log, error)
}

type EVMClient interface {
	client.Client
	gas.GasPriceClient
	ChainID(ctx context.Context) (*big.Int, error)
}

func WaitForProposalExecuted(connection *connection.Connection, beforeBalance substrateTypes.U128, key []byte) error {
	timeout := time.After(TestTimeout)
	ticker := time.NewTicker(5 * time.Second)

	for {
		select {
		case <-ticker.C:
			done, err := checkBalance(beforeBalance, connection, key)
			if err != nil {
				ticker.Stop()
				return err
			}
			if done {
				ticker.Stop()
				return nil
			}
		case <-timeout:
			ticker.Stop()
			return errors.New("test timed out waiting for proposal execution event")
		}
	}
}

func checkBalance(beforeBalance substrateTypes.U128, connection *connection.Connection, key []byte) (bool, error) {
	meta := connection.GetMetadata()
	var acc Account
	var assetId uint32 = 2000
	assetIdSerialized := make([]byte, 4)
	binary.LittleEndian.PutUint32(assetIdSerialized, assetId)

	key, _ = substrateTypes.CreateStorageKey(&meta, "Assets", "Account", assetIdSerialized, key)
	_, err := connection.RPC.State.GetStorageLatest(key, &acc)
	if err != nil {
		return false, err
	}
	destBalanceAfter := acc.Balance
	if destBalanceAfter.Int.Cmp(beforeBalance.Int) == 1 {
		return true, nil
	} else {
		return false, nil
	}
}

type Account struct {
	Balance substrateTypes.U128
}

func ConstructRecipientData(recipient []substrateTypes.U8) []byte {
	rec := substrateTypes.MultiLocationV1{
		Parents: 0,
		Interior: substrateTypes.JunctionsV1{
			IsX1: true,
			X1: substrateTypes.JunctionV1{
				IsAccountID32: true,
				AccountID32NetworkID: substrateTypes.NetworkID{
					IsAny: true,
				},
				AccountID: recipient,
			},
		},
	}

	encodedRecipient := bytes.NewBuffer([]byte{})
	encoder := scale.NewEncoder(encodedRecipient)
	_ = rec.Encode(*encoder)

	recipientBytes := encodedRecipient.Bytes()
	var finalRecipient []byte

	// remove accountID size data
	// this is a fix because the substrate decoder is not able to parse the data with extra data
	// that represents size of the recipient byte array
	finalRecipient = append(finalRecipient, recipientBytes[:4]...)
	finalRecipient = append(finalRecipient, recipientBytes[5:]...)

	return finalRecipient
}
