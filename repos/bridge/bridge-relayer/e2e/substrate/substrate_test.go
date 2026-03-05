// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package substrate_test

import (
	"context"
	"encoding/binary"

	substrateTypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/ethereum/go-ethereum/core/types"
	evmClient "github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/gas"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/signAndSend"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/transaction"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/client"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/connection"

	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/contracts/bridge"

	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/e2e/evm/contracts/erc20"
	"github.com/ChainSafe/sygma-relayer/e2e/substrate"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/suite"
)

type TestClient interface {
	evm.EVMClient
	LatestBlock() (*big.Int, error)
	CodeAt(ctx context.Context, contractAddress common.Address, block *big.Int) ([]byte, error)
	FetchEventLogs(ctx context.Context, contractAddress common.Address, event string, startBlock *big.Int, endBlock *big.Int) ([]types.Log, error)
	SubscribeFilterLogs(ctx context.Context, q ethereum.FilterQuery, ch chan<- types.Log) (ethereum.Subscription, error)
	TransactionByHash(ctx context.Context, hash common.Hash) (tx *types.Transaction, isPending bool, err error)
	BalanceAt(ctx context.Context, account common.Address, blockNumber *big.Int) (*big.Int, error)
}

func Test_EVMSubstrate(t *testing.T) {
	ethClient, err := evmClient.NewEVMClient(evm.ETHEndpoint1, evm.AdminAccount)
	if err != nil {
		panic(err)
	}
	gasPricer := gas.NewStaticGasPriceDeterminant(ethClient, nil)

	substrateConnection, err := connection.NewSubstrateConnection(substrate.SubstrateEndpoint)
	if err != nil {
		panic(err)
	}
	substrateClient := client.NewSubstrateClient(substrateConnection, &substrate.SubstratePK, big.NewInt(5), 0)

	var assetId uint32 = 2000
	assetIdSerialized := make([]byte, 4)
	binary.LittleEndian.PutUint32(assetIdSerialized, assetId)

	suite.Run(
		t,
		NewEVMSubstrateTestSuite(
			transaction.NewTransaction,
			ethClient,
			substrateClient,
			substrateConnection,
			gasPricer,
			evm.DEFAULT_CONFIG,
			assetIdSerialized,
		),
	)
}

func NewEVMSubstrateTestSuite(
	fabric transaction.TxFabric,
	evmClient TestClient,
	substrateClient *client.SubstrateClient,
	substrateConnection *connection.Connection,
	gasPricer signAndSend.GasPricer,
	evmConfig evm.BridgeConfig,
	substrateAssetID []byte,
) *IntegrationTestSuite {
	return &IntegrationTestSuite{
		fabric:              fabric,
		evmClient:           evmClient,
		substrateClient:     substrateClient,
		substrateConnection: substrateConnection,
		gasPricer:           gasPricer,
		evmConfig:           evmConfig,
		substrateAssetID:    substrateAssetID,
	}
}

type IntegrationTestSuite struct {
	suite.Suite
	fabric              transaction.TxFabric
	evmClient           TestClient
	substrateClient     *client.SubstrateClient
	substrateConnection *connection.Connection
	gasPricer           signAndSend.GasPricer
	evmConfig           evm.BridgeConfig
	substrateAssetID    []byte
}

func (s *IntegrationTestSuite) SetupSuite() {
	// EVM side preparation
	evmTransactor := signAndSend.NewSignAndSendTransactor(s.fabric, s.gasPricer, s.evmClient)
	mintTo := s.evmClient.From()
	amountToMint := big.NewInt(0).Mul(big.NewInt(5000000000000000), big.NewInt(0).Exp(big.NewInt(10), big.NewInt(18), nil))
	amountToApprove := big.NewInt(0).Mul(big.NewInt(100000), big.NewInt(0).Exp(big.NewInt(10), big.NewInt(18), nil))

	erc20LRContract := erc20.NewERC20Contract(s.evmClient, s.evmConfig.Erc20LockReleaseAddr, evmTransactor)
	_, err := erc20LRContract.MintTokens(mintTo, amountToMint, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}
	_, err = erc20LRContract.MintTokens(s.evmConfig.Erc20LockReleaseHandlerAddr, amountToMint, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}
	_, err = erc20LRContract.ApproveTokens(s.evmConfig.Erc20HandlerAddr, amountToApprove, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}
}

func (s *IntegrationTestSuite) Test_Erc20Deposit_Substrate_to_EVM() {
	var accountInfoBefore substrate.Account
	meta := s.substrateConnection.GetMetadata()
	key, _ := substrateTypes.CreateStorageKey(&meta, "Assets", "Account", s.substrateAssetID, substrate.SubstratePK.PublicKey)
	_, err := s.substrateConnection.RPC.State.GetStorageLatest(key, &accountInfoBefore)
	s.Nil(err)

	transactor := signAndSend.NewSignAndSendTransactor(s.fabric, s.gasPricer, s.evmClient)

	erc20Contract := erc20.NewERC20Contract(s.evmClient, s.evmConfig.Erc20LockReleaseAddr, transactor)

	destBalanceBefore, err := erc20Contract.GetBalance(s.evmClient.From())
	s.Nil(err)

	extHash, sub, err := s.substrateClient.Transact("SygmaBridge.deposit", substrate.USDCAsset{}, substrate.Destination{})
	s.Nil(err)

	err = s.substrateClient.TrackExtrinsic(extHash, sub)
	s.Nil(err)

	err = evm.WaitForProposalExecuted(s.evmClient, s.evmConfig.BridgeAddr)
	s.Nil(err)

	meta = s.substrateConnection.GetMetadata()
	var senderBalanceAfter substrate.Account
	key, _ = substrateTypes.CreateStorageKey(&meta, "Assets", "Account", s.substrateAssetID, substrate.SubstratePK.PublicKey)
	_, err = s.substrateConnection.RPC.State.GetStorageLatest(key, &senderBalanceAfter)
	s.Nil(err)

	// balance of sender has decreased
	s.Equal(1, accountInfoBefore.Balance.Int.Cmp(senderBalanceAfter.Balance.Int))
	destBalanceAfter, err := erc20Contract.GetBalance(s.evmClient.From())

	s.Nil(err)
	//Balance has increased
	s.Equal(1, destBalanceAfter.Cmp(destBalanceBefore))
}

var amountToDeposit = big.NewInt(100000000000000)

func (s *IntegrationTestSuite) Test_Erc20Deposit_EVM_to_Substrate() {
	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric, s.gasPricer, s.evmClient)
	erc20Contract1 := erc20.NewERC20Contract(s.evmClient, s.evmConfig.Erc20LockReleaseAddr, transactor1)
	bridgeContract1 := bridge.NewBridgeContract(s.evmClient, s.evmConfig.BridgeAddr, transactor1)

	senderBalBefore, err := erc20Contract1.GetBalance(s.evmClient.From())
	s.Nil(err)

	meta := s.substrateConnection.GetMetadata()
	var destBalanceBefore substrate.Account
	key, _ := substrateTypes.CreateStorageKey(&meta, "Assets", "Account", s.substrateAssetID, substrate.SubstratePK.PublicKey)
	_, err = s.substrateConnection.RPC.State.GetStorageLatest(key, &destBalanceBefore)
	s.Nil(err)
	pk := []substrateTypes.U8{0xd4, 0x35, 0x93, 0xc7, 0x15, 0xfd, 0xd3, 0x1c, 0x61, 0x14, 0x1a, 0xbd, 0x4, 0xa9, 0x9f, 0xd6, 0x82, 0x2c, 0x85, 0x58, 0x85, 0x4c, 0xcd, 0xe3, 0x9a, 0x56, 0x84, 0xe7, 0xa5, 0x6d, 0xa2, 0x7d}

	recipientMultilocation := substrate.ConstructRecipientData(pk)

	erc20DepositData := evm.ConstructErc20DepositData(recipientMultilocation, amountToDeposit)
	_, err = bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.evmConfig.BasicFee}, uint8(3), s.evmConfig.Erc20LockReleaseResourceID, erc20DepositData, []byte{})

	s.Nil(err)

	err = substrate.WaitForProposalExecuted(s.substrateConnection, destBalanceBefore.Balance, substrate.SubstratePK.PublicKey)
	s.Nil(err)
	senderBalAfter, err := erc20Contract1.GetBalance(s.evmClient.From())
	s.Nil(err)
	s.Equal(-1, senderBalAfter.Cmp(senderBalBefore))

	var destBalanceAfter substrate.Account
	key, _ = substrateTypes.CreateStorageKey(&meta, "Assets", "Account", s.substrateAssetID, substrate.SubstratePK.PublicKey)
	_, err = s.substrateConnection.RPC.State.GetStorageLatest(key, &destBalanceAfter)
	s.Nil(err)

	//Balance has increased
	s.Equal(1, destBalanceAfter.Balance.Int.Cmp(destBalanceBefore.Balance.Int))
}
