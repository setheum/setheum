package transaction_test

import (
	"math/big"
	"testing"

	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/gas"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/transaction"
	"github.com/sygmaprotocol/sygma-core/crypto/secp256k1"
	"github.com/sygmaprotocol/sygma-core/mock"

	"github.com/ethereum/go-ethereum/core/types"

	"github.com/ethereum/go-ethereum/common"

	"github.com/stretchr/testify/suite"
	"go.uber.org/mock/gomock"
)

var aliceKp, _ = secp256k1.NewKeypairFromString("b1370fac45517e19e27e16a2a31da07b9775d3a3bceb77a78f790f99655aa668")

type EVMTxTestSuite struct {
	suite.Suite
	client *mock.MockLondonGasClient
}

func TestRunTestSuite(t *testing.T) {
	suite.Run(t, new(EVMTxTestSuite))
}

func (s *EVMTxTestSuite) SetupSuite()    {}
func (s *EVMTxTestSuite) TearDownSuite() {}
func (s *EVMTxTestSuite) SetupTest() {
	gomockController := gomock.NewController(s.T())
	s.client = mock.NewMockLondonGasClient(gomockController)
}
func (s *EVMTxTestSuite) TearDownTest() {}

func (s *EVMTxTestSuite) TestNewTransactionWithStaticGasPricer() {
	s.client.EXPECT().SuggestGasPrice(gomock.Any()).Return(big.NewInt(1000), nil)
	txFabric := transaction.NewTransaction
	gasPriceClient := gas.NewStaticGasPriceDeterminant(s.client, nil)
	gp, err := gasPriceClient.GasPrice(nil)
	s.Nil(err)
	tx, err := txFabric(1, &common.Address{}, big.NewInt(0), 10000, gp, []byte{})
	s.Nil(err)
	rawTx, err := tx.RawWithSignature(aliceKp, big.NewInt(420))
	s.Nil(err)
	txt := types.Transaction{}
	err = txt.UnmarshalBinary(rawTx)
	s.Nil(err)
	s.Equal(types.LegacyTxType, int(txt.Type()))
}

func (s *EVMTxTestSuite) TestNewTransactionWithLondonGasPricer() {
	s.client.EXPECT().BaseFee().Return(big.NewInt(1000), nil)
	s.client.EXPECT().SuggestGasTipCap(gomock.Any()).Return(big.NewInt(1000), nil)
	txFabric := transaction.NewTransaction
	gasPriceClient := gas.NewLondonGasPriceClient(s.client, nil)
	gp, err := gasPriceClient.GasPrice(nil)
	s.Nil(err)
	tx, err := txFabric(1, &common.Address{}, big.NewInt(0), 10000, gp, []byte{})
	s.Nil(err)
	rawTx, err := tx.RawWithSignature(aliceKp, big.NewInt(420))
	s.Nil(err)
	txt := types.Transaction{}
	err = txt.UnmarshalBinary(rawTx)
	s.Nil(err)
	s.Equal(types.DynamicFeeTxType, int(txt.Type()))
}
