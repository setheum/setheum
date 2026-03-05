// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package btc_test

import (
	"context"
	"encoding/hex"
	"math"

	"github.com/btcsuite/btcd/btcec/v2"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/btcsuite/btcd/txscript"
	"github.com/btcsuite/btcd/wire"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	evmClient "github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/gas"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/signAndSend"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/transaction"

	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/btc/connection"
	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/contracts/bridge"

	"github.com/ChainSafe/sygma-relayer/e2e/btc"
	"github.com/ChainSafe/sygma-relayer/e2e/evm"
	"github.com/ChainSafe/sygma-relayer/e2e/evm/contracts/erc20"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/suite"
)

var amountToDeposit = big.NewInt(100000000)

type TestClient interface {
	evm.EVMClient
	LatestBlock() (*big.Int, error)
	CodeAt(ctx context.Context, contractAddress common.Address, block *big.Int) ([]byte, error)
	FetchEventLogs(ctx context.Context, contractAddress common.Address, event string, startBlock *big.Int, endBlock *big.Int) ([]types.Log, error)
	SubscribeFilterLogs(ctx context.Context, q ethereum.FilterQuery, ch chan<- types.Log) (ethereum.Subscription, error)
	TransactionByHash(ctx context.Context, hash common.Hash) (tx *types.Transaction, isPending bool, err error)
	BalanceAt(ctx context.Context, account common.Address, blockNumber *big.Int) (*big.Int, error)
}

func Test_EVMBtc(t *testing.T) {
	evmConfig := evm.DEFAULT_CONFIG
	evmConfig.Erc20LockReleaseAddr = common.HexToAddress("0xb61bd8740F60e0Bfc1b5C3fA2Bb9810e4AEf8938")
	evmConfig.Erc20LockReleaseResourceID = evm.SliceTo32Bytes(common.LeftPadBytes([]byte{0x10}, 31))

	ethClient, err := evmClient.NewEVMClient(evm.ETHEndpoint1, evm.AdminAccount)
	if err != nil {
		panic(err)
	}
	gasPricer := gas.NewStaticGasPriceDeterminant(ethClient, nil)

	suite.Run(
		t,
		NewEVMBtcTestSuite(
			transaction.NewTransaction,
			ethClient,
			gasPricer,
			evmConfig,
		),
	)
}

func NewEVMBtcTestSuite(
	fabric transaction.TxFabric,
	evmClient TestClient,
	gasPricer signAndSend.GasPricer,
	evmConfig evm.BridgeConfig,
) *IntegrationTestSuite {
	return &IntegrationTestSuite{
		fabric:    fabric,
		evmClient: evmClient,
		gasPricer: gasPricer,
		evmConfig: evmConfig,
	}
}

type IntegrationTestSuite struct {
	suite.Suite
	fabric    transaction.TxFabric
	evmClient TestClient
	gasPricer signAndSend.GasPricer
	evmConfig evm.BridgeConfig
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
	_, err = erc20LRContract.ApproveTokens(s.evmConfig.Erc20LockReleaseHandlerAddr, amountToApprove, transactor.TransactOptions{})
	if err != nil {
		panic(err)
	}
}

func (s *IntegrationTestSuite) Test_Erc20Deposit_EVM_to_Btc() {
	conn, err := connection.NewBtcConnection(btc.BtcEndpoint, "user", "password", true)
	s.Nil(err)

	addr, err := btcutil.DecodeAddress("bcrt1pja8aknn7te4empmghnyqnrtjqn0lyg5zy3p5jsdp4le930wnpnxsrtd3ht", &chaincfg.RegressionNetParams)
	s.Nil(err)
	balanceBefore, err := conn.ListUnspentMinMaxAddresses(0, 1000, []btcutil.Address{addr})
	s.Nil(err)
	s.Equal(len(balanceBefore), 0)

	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric, s.gasPricer, s.evmClient)
	bridgeContract1 := bridge.NewBridgeContract(s.evmClient, s.evmConfig.BridgeAddr, transactor1)
	erc20DepositData := evm.ConstructErc20DepositData([]byte("bcrt1pja8aknn7te4empmghnyqnrtjqn0lyg5zy3p5jsdp4le930wnpnxsrtd3ht"), amountToDeposit)
	_, err = bridgeContract1.ExecuteTransaction("deposit", transactor.TransactOptions{Value: s.evmConfig.BasicFee}, uint8(4), s.evmConfig.Erc20LockReleaseResourceID, erc20DepositData, []byte{})
	s.Nil(err)

	err = btc.WaitForProposalExecuted(conn, addr)
	s.Nil(err)

	balanceAfter, err := conn.ListUnspentMinMaxAddresses(0, 1000, []btcutil.Address{addr})
	s.Nil(err)
	s.Equal(balanceAfter[0].Amount*100000000, float64(amountToDeposit.Int64()))

}

func (s *IntegrationTestSuite) Test_Erc20Deposit_Btc_to_EVM() {
	pk, _ := crypto.HexToECDSA("8df766a778b57d58c2ad239ed98ec0e611a9bfbdc328b61755ebc40cf20c0f3f")
	dstAddr := crypto.PubkeyToAddress(pk.PublicKey)

	conn, err := connection.NewBtcConnection(btc.BtcEndpoint, "user", "password", true)
	s.Nil(err)
	transactor1 := signAndSend.NewSignAndSendTransactor(s.fabric, s.gasPricer, s.evmClient)
	erc20Contract2 := erc20.NewERC20Contract(s.evmClient, s.evmConfig.Erc20LockReleaseAddr, transactor1)
	destBalanceBefore, err := erc20Contract2.GetBalance(dstAddr)

	// Your Bitcoin address
	add, _ := btcutil.DecodeAddress("mrheH3ouZNyUbpp9LtWP28xqv1yhNQAsfC", &chaincfg.RegressionNetParams)
	s.Nil(err)

	recipientAddress, err := btcutil.DecodeAddress("bcrt1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2sjyr5ek", &chaincfg.RegressionNetParams)
	s.Nil(err)

	feeAddress, err := btcutil.DecodeAddress("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt", &chaincfg.RegressionNetParams)
	s.Nil(err)

	// Define the private key as a hexadecimal string
	privateKeyHex := "ccfa495d2ae193eeec53db12971bdedfe500603ec53f98a6138f0abe932be84f"

	// Decode the hexadecimal string into a byte slice
	privateKeyBytes, err := hex.DecodeString(privateKeyHex)
	s.Nil(err)

	// Create a new private key instance
	privateKey, _ := btcec.PrivKeyFromBytes(privateKeyBytes)

	// Fetch unspent transaction outputs (UTXOs) associated with your Bitcoin address
	unspent, err := conn.Client.ListUnspentMinMaxAddresses(1, 9999999, []btcutil.Address{add})
	s.Nil(err)

	// Create the PkScript for the recipient address
	recipientPkScript, err := txscript.PayToAddrScript(recipientAddress)
	s.Nil(err)

	// Create the PkScript for the recipient address
	feeAddressPkScript, err := txscript.PayToAddrScript(feeAddress)
	s.Nil(err)

	// Define data for the OP_RETURN output
	opReturnData := []byte("0x1c3A03D04c026b1f4B4208D2ce053c5686E6FB8d_01")
	opReturnScript, err := txscript.NullDataScript(opReturnData)
	s.Nil(err)

	// Create transaction inputs
	var txInputs []*wire.TxIn
	hash, _ := chainhash.NewHashFromStr(unspent[0].TxID)
	txInput := wire.NewTxIn(&wire.OutPoint{
		Hash:  *hash,
		Index: unspent[0].Vout,
	}, nil, nil)

	hash2, _ := chainhash.NewHashFromStr(unspent[1].TxID)
	txInput2 := wire.NewTxIn(&wire.OutPoint{
		Hash:  *hash2,
		Index: unspent[1].Vout,
	}, nil, nil)

	txInputs = append(txInputs, txInput)
	txInputs = append(txInputs, txInput2)

	// Create transaction outputs
	txOutputs := []*wire.TxOut{
		{
			Value:    int64(unspent[0].Amount*math.Pow(10, 8)) - 10000000,
			PkScript: recipientPkScript,
		},
		{
			Value:    int64(unspent[1].Amount*math.Pow(10, 8)) - 10000000,
			PkScript: feeAddressPkScript,
		},
		{
			Value:    0,
			PkScript: opReturnScript,
		},
	}
	// Create transaction
	tx := wire.NewMsgTx(wire.TxVersion)

	// Add inputs to the transaction
	tx.TxIn = txInputs
	// Add outputs to the transaction
	for _, txOut := range txOutputs {
		tx.AddTxOut(txOut)
	}

	for i, txIn := range tx.TxIn {
		subscript, err := hex.DecodeString(unspent[i].ScriptPubKey)
		s.Nil(err)
		sigScript, err := txscript.SignatureScript(tx, i, subscript, txscript.SigHashAll, privateKey, true)
		s.Nil(err)
		txIn.SignatureScript = sigScript
	}

	_, err = conn.Client.SendRawTransaction(tx, true)
	s.Nil(err)

	// Generate blocks to confirm the transaction
	_, err = conn.Client.GenerateToAddress(2, add, nil)
	s.Nil(err)

	err = evm.WaitForProposalExecuted(s.evmClient, s.evmConfig.BridgeAddr)
	s.Nil(err)

	destBalanceAfter, err := erc20Contract2.GetBalance(dstAddr)
	s.Nil(err)

	//Balance has increased
	s.Equal(1, destBalanceAfter.Cmp(destBalanceBefore))
}
