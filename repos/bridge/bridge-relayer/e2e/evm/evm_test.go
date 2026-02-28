// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package evm_test

import (
	"context"
	"math/big"
	"math/rand"
	"sync"
	"testing"
	"time"

	substrateTypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/rs/zerolog/log"
	"github.com/stretchr/testify/suite"

	"github.com/ChainSafe/sygma-relayer/e2e/evm/contracts/erc1155"
	"github.com/ChainSafe/sygma-relayer/e2e/evm/contracts/erc20"
	"github.com/ChainSafe/sygma-relayer/e2e/evm/contracts/erc721"
	"github.com/ChainSafe/sygma-relayer/e2e/evm/contracts/retry"
	"github.com/ChainSafe/sygma-relayer/e2e/evm/keystore"

	"github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/gas"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/signAndSend"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/transaction"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/contracts/bridge"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
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

// Alice key is used by the relayer, Charlie key is used as admin and depositer
func Test_EVM2EVM(t *testing.T) {
	rand.Seed(time.Now().Unix())
	ethClient1, err := client.NewEVMClient(evm.ETHEndpoint1, evm.AdminAccount)
	if err != nil {
		panic(err)
	}
	gasPricer1 := gas.NewStaticGasPriceDeterminant(ethClient1, nil)

	ethClient2, err := client.NewEVMClient(evm.ETHEndpoint2, evm.AdminAccount)
	if err != nil {
		panic(err)
	}
	gasPricer2 := gas.NewStaticGasPriceDeterminant(ethClient2, nil)

	suite.Run(
		t,
		NewEVM2EVMTestSuite(
			transaction.NewTransaction,
			transaction.NewTransaction,
			ethClient1,
			ethClient2,
			gasPricer1,
			gasPricer2,
			evm.DEFAULT_CONFIG,
			evm.DEFAULT_CONFIG,
		),
	)
}

func NewEVM2EVMTestSuite(
	fabric1, fabric2 transaction.TxFabric,
	client1, client2 TestClient,
	gasPricer1, gasPricer2 signAndSend.GasPricer,
	config1, config2 evm.BridgeConfig,
) *IntegrationTestSuite {
	return &IntegrationTestSuite{
		fabric1:    fabric1,
		fabric2:    fabric2,
		client1:    client1,
		client2:    client2,
		gasPricer1: gasPricer1,
		gasPricer2: gasPricer2,
		config1:    config1,
		config2:    config2,
	}
}

type IntegrationTestSuite struct {
	suite.Suite
	client1    TestClient
	client2    TestClient
	gasPricer1 signAndSend.GasPricer
	gasPricer2 signAndSend.GasPricer
	fabric1    transaction.TxFabric
	fabric2    transaction.TxFabric
	config1    evm.BridgeConfig
	config2    evm.BridgeConfig
}

func (s *IntegrationTestSuite) SetupSuite() {
	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	transactor2 := signAndSend.NewSignAndSendTransactor(s.fabric2, s.gasPricer2, s.client2)
	erc20Contract := erc20.NewERC20Contract(s.client1, s.config1.Erc20Addr, transactor1)
	mintTo := s.client1.From()
	amountToMint := big.NewInt(0).Mul(big.NewInt(5000000000000000), big.NewInt(0).Exp(big.NewInt(10), big.NewInt(18), nil))

	amountToApprove := big.NewInt(0).Mul(big.NewInt(100000), big.NewInt(0).Exp(big.NewInt(10), big.NewInt(18), nil))
	_, err := erc20Contract.MintTokens(mintTo, amountToMint, transactor.TransactOptions{})

	if err != nil {
		panic(err)
	}

	_, err = erc20Contract.MintTokens(s.config1.Erc20HandlerAddr, amountToMint, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}
	// Approving tokens
	_, err = erc20Contract.ApproveTokens(s.config1.Erc20HandlerAddr, amountToApprove, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}

	erc20LRContract := erc20.NewERC20Contract(s.client1, s.config1.Erc20LockReleaseAddr, transactor1)
	_, err = erc20LRContract.MintTokens(mintTo, amountToMint, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}

	erc20LRContract2 := erc20.NewERC20Contract(s.client2, s.config2.Erc20LockReleaseAddr, transactor2)
	_, err = erc20LRContract2.MintTokens(s.config2.Erc20LockReleaseHandlerAddr, amountToMint, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}

	// Approving tokens
	_, err = erc20LRContract.ApproveTokens(s.config1.Erc20HandlerAddr, amountToApprove, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}
}

var amountToDeposit = big.NewInt(100)

func (s *IntegrationTestSuite) Test_Erc20Deposit() {
	pk, _ := crypto.HexToECDSA("cc2c32b154490f09f70c1c8d4b997238448d649e0777495863db231c4ced3616")
	dstAddr := crypto.PubkeyToAddress(pk.PublicKey)

	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	erc20Contract1 := erc20.NewERC20Contract(s.client1, s.config1.Erc20Addr, transactor1)
	bridgeContract1 := bridge.NewBridgeContract(s.client1, s.config1.BridgeAddr, transactor1)

	transactor2 := signAndSend.NewSignAndSendTransactor(s.fabric2, s.gasPricer2, s.client2)
	erc20Contract2 := erc20.NewERC20Contract(s.client2, s.config2.Erc20Addr, transactor2)

	senderBalBefore, err := erc20Contract1.GetBalance(crypto.PubkeyToAddress(pk.PublicKey))
	s.Nil(err)
	destBalanceBefore, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)

	erc20DepositData := evm.ConstructErc20DepositData(dstAddr.Bytes(), amountToDeposit)
	depositTxHash, err := bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.config1.BasicFee}, uint8(2), s.config1.Erc20ResourceID, erc20DepositData, []byte{})

	s.Nil(err)

	log.Debug().Msgf("deposit hash %s", depositTxHash.Hex())

	_, _, err = s.client1.TransactionByHash(context.Background(), *depositTxHash)
	s.Nil(err)

	err = evm.WaitForProposalExecuted(s.client2, s.config2.BridgeAddr)
	s.Nil(err)

	senderBalAfter, err := erc20Contract1.GetBalance(s.client1.From())
	s.Nil(err)
	s.Equal(-1, senderBalAfter.Cmp(senderBalBefore))

	destBalanceAfter, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)
	//Balance has increased
	s.Equal(1, destBalanceAfter.Cmp(destBalanceBefore))
}

func (s *IntegrationTestSuite) Test_Erc721Deposit() {
	tokenId := big.NewInt(int64(rand.Int()))
	metadata := "metadata.url"

	dstAddr := keystore.TestKeyRing.EthereumKeys[keystore.BobKey].CommonAddress()

	// erc721 contract for evm1
	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	erc721Contract1 := erc721.NewErc721Contract(s.client1, s.config1.Erc721Addr, transactor1)
	bridgeContract1 := bridge.NewBridgeContract(s.client1, s.config1.BridgeAddr, transactor1)

	// erc721 contract for evm2
	transactor2 := signAndSend.NewSignAndSendTransactor(s.fabric2, s.gasPricer2, s.client2)
	erc721Contract2 := erc721.NewErc721Contract(s.client2, s.config2.Erc721Addr, transactor2)

	// Mint token and give approval
	// This is done here so token only exists on evm1
	_, err := erc721Contract1.Mint(tokenId, metadata, s.client1.From(), transactor.TransactOptions{})
	s.Nil(err, "Mint failed")
	_, err = erc721Contract1.Approve(tokenId, s.config1.Erc721HandlerAddr, transactor.TransactOptions{})
	s.Nil(err, "Approve failed")

	// Check on evm1 if initial owner is admin
	initialOwner, err := erc721Contract1.Owner(tokenId)
	s.Nil(err)
	s.Equal(initialOwner.String(), s.client1.From().String())

	// Check on evm2 token doesn't exist
	_, err = erc721Contract2.Owner(tokenId)
	s.Error(err)

	erc721DepositData := evm.ConstructErc721DepositData(dstAddr.Bytes(), tokenId, []byte(metadata))
	depositTxHash, err := bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.config1.BasicFee}, uint8(2), s.config1.Erc721ResourceID, erc721DepositData, []byte{})

	s.Nil(err)

	_, _, err = s.client1.TransactionByHash(context.Background(), *depositTxHash)
	s.Nil(err)

	err = evm.WaitForProposalExecuted(s.client2, s.config2.BridgeAddr)
	s.Nil(err)

	// Check on evm1 that token is burned
	_, err = erc721Contract1.Owner(tokenId)
	s.Error(err)

	// Check on evm2 that token is minted to destination address
	owner, err := erc721Contract2.Owner(tokenId)
	s.Nil(err)
	s.Equal(dstAddr.String(), owner.String())
}

func (s *IntegrationTestSuite) Test_PermissionlessGenericDeposit() {
	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	bridgeContract1 := bridge.NewBridgeContract(s.client1, s.config1.BridgeAddr, transactor1)

	byteArrayToHash, _ := substrateTypes.NewI64(int64(rand.Int())).MarshalJSON()
	hash := substrateTypes.NewHash(byteArrayToHash)
	functionSig := string(crypto.Keccak256([]byte("storeWithDepositor(address,bytes32,address)"))[:4])
	contractAddress := s.client2.From()
	maxFee := big.NewInt(600000)
	depositor := s.client1.From()
	var metadata []byte
	metadata = append(metadata, common.LeftPadBytes(hash[:], 32)...)
	metadata = append(metadata, common.LeftPadBytes(depositor.Bytes(), 32)...)

	permissionlessGenericDepositData := evm.ConstructPermissionlessGenericDepositData(metadata, []byte(functionSig), contractAddress.Bytes(), depositor.Bytes(), maxFee)
	_, err := bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.config1.BasicFee}, uint8(2), s.config1.PermissionlessGenericResourceID, permissionlessGenericDepositData, []byte{})
	s.Nil(err)

	err = evm.WaitForProposalExecuted(s.client2, s.config2.BridgeAddr)
	s.Nil(err)
}

func (s *IntegrationTestSuite) Test_RetryDeposit() {
	dstAddr := keystore.TestKeyRing.EthereumKeys[keystore.BobKey].CommonAddress()
	amountToMint := big.NewInt(0).Mul(big.NewInt(350), big.NewInt(0).Exp(big.NewInt(10), big.NewInt(18), nil))

	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	bridgeContract1 := bridge.NewBridgeContract(s.client1, s.config1.BridgeAddr, transactor1)

	transactor2 := signAndSend.NewSignAndSendTransactor(s.fabric2, s.gasPricer2, s.client2)
	erc20Contract2 := erc20.NewERC20Contract(s.client2, s.config2.Erc20LockReleaseAddr, transactor2)

	retryContract := retry.NewRetryContract(s.client2, s.config2.RetryAddr, transactor2)

	destBalanceBefore, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)

	erc20DepositData := evm.ConstructErc20DepositData(dstAddr.Bytes(), amountToDeposit)
	depositTxHash, err := bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.config1.BasicFee}, uint8(2), s.config1.Erc20LockReleaseResourceID, erc20DepositData, []byte{})

	s.Nil(err)

	log.Debug().Msgf("deposit hash %s", depositTxHash.Hex())

	receipt, err := s.client1.TransactionReceipt(context.Background(), *depositTxHash)
	s.Nil(err)
	time.Sleep(time.Second * 15)

	_, err = erc20Contract2.MintTokens(s.config2.Erc20LockReleaseHandlerAddr, amountToMint, transactor.TransactOptions{})
	if err != nil {
		return
	}

	retryTxHash, err := retryContract.Retry(1, 2, receipt.BlockNumber, s.config1.Erc20LockReleaseResourceID, transactor.TransactOptions{})
	if err != nil {
		return
	}
	s.Nil(err)
	s.NotNil(retryTxHash)

	time.Sleep(time.Second * 15)

	destBalanceAfter, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)
	//Balance has increased
	s.Equal(1, destBalanceAfter.Cmp(destBalanceBefore))
}

func (s *IntegrationTestSuite) Test_MultipleDeposits() {
	dstAddr := keystore.TestKeyRing.EthereumKeys[keystore.BobKey].CommonAddress()

	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	bridgeContract1 := bridge.NewBridgeContract(s.client1, s.config1.BridgeAddr, transactor1)

	transactor2 := signAndSend.NewSignAndSendTransactor(s.fabric2, s.gasPricer2, s.client2)
	erc20Contract2 := erc20.NewERC20Contract(s.client2, s.config2.Erc20Addr, transactor2)

	destBalanceBefore, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)

	numOfDeposits := 25

	var wg sync.WaitGroup
	for i := 0; i < numOfDeposits; i++ {
		go func() {

			erc20DepositData := evm.ConstructErc20DepositData(dstAddr.Bytes(), amountToDeposit)
			_, err := bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.config1.BasicFee}, uint8(2), s.config1.Erc20ResourceID, erc20DepositData, []byte{})

			wg.Add(1)
			defer wg.Done()
			s.Nil(err)
		}()
	}
	wg.Wait()
	time.Sleep(10 * time.Second)

	err = evm.WaitForProposalExecuted(s.client2, s.config2.BridgeAddr)
	s.Nil(err)

	destBalanceAfter, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)
	//Balance has increased
	s.Equal(1, destBalanceAfter.Cmp(destBalanceBefore))
}

func (s *IntegrationTestSuite) Test_Erc1155Deposit() {
	tokenId := big.NewInt(int64(rand.Int()))
	amount := big.NewInt(int64(rand.Int()))

	metadata := "metadata.url"

	txOptions := transactor.TransactOptions{}

	dstAddr := keystore.TestKeyRing.EthereumKeys[keystore.BobKey].CommonAddress()

	// 1155 contract for evm1
	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric1, s.gasPricer1, s.client1)
	erc1155Contract1 := erc1155.NewErc1155Contract(s.client1, s.config1.Erc1155Addr, transactor1)
	bridgeContract1 := bridge.NewBridgeContract(s.client1, s.config1.BridgeAddr, transactor1)

	// 1155 contract for evm2
	transactor2 := signAndSend.NewSignAndSendTransactor(s.fabric2, s.gasPricer2, s.client2)
	erc1155Contract2 := erc1155.NewErc1155Contract(s.client2, s.config2.Erc1155Addr, transactor2)

	// Mint token and give approval
	// This is done here so token only exists on evm1
	_, err := erc1155Contract1.Mint(tokenId, amount, []byte{0}, s.client1.From(), txOptions)
	s.Nil(err, "Mint failed")
	_, err = erc1155Contract1.Approve(tokenId, s.config1.Erc1155HandlerAddr, txOptions)
	s.Nil(err, "Approve failed")

	initialAmount, err := erc1155Contract1.BalanceOf(s.client1.From(), tokenId)
	s.Nil(err)
	s.Equal(0, initialAmount.Cmp(amount))

	erc1155DepositData, err := evm.ConstructErc1155DepositData(dstAddr.Bytes(), tokenId, amount, []byte(metadata))
	s.Nil(err)
	depositTxHash, err := bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.config1.BasicFee}, uint8(2), s.config1.Erc1155ResourceID, erc1155DepositData, []byte{})
	s.Nil(err)

	_, _, err = s.client1.TransactionByHash(context.Background(), *depositTxHash)
	s.Nil(err)

	err = evm.WaitForProposalExecuted(s.client2, s.config2.BridgeAddr)
	s.Nil(err)

	// Check on evm1 that token is burned
	sourceAmount, err := erc1155Contract1.BalanceOf(s.client1.From(), tokenId)
	s.Nil(err)
	s.Equal(0, sourceAmount.Cmp(big.NewInt(0)))

	// Check on evm2 that token is minted to destination address
	dstAmount, err := erc1155Contract2.BalanceOf(dstAddr, tokenId)
	s.Nil(err)
	s.Equal(0, dstAmount.Cmp(initialAmount))
}
