package listener_test

import (
	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/btc/config"
	"github.com/ChainSafe/sygma-relayer/chains/btc/listener"
	mock_listener "github.com/ChainSafe/sygma-relayer/chains/btc/listener/mock"
	"github.com/btcsuite/btcd/btcjson"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/suite"
)

type DecodeEventsSuite struct {
	suite.Suite
	mockConn   *mock_listener.MockConnection
	resource   config.Resource
	feeAddress btcutil.Address
}

func TestRunDecodeDepositEventsSuite(t *testing.T) {
	suite.Run(t, new(DecodeEventsSuite))
}

func (s *DecodeEventsSuite) SetupTest() {
	ctrl := gomock.NewController(s.T())
	address, _ := btcutil.DecodeAddress("tb1qln69zuhdunc9stwfh6t7adexxrcr04ppy6thgm", &chaincfg.TestNet3Params)
	s.resource = config.Resource{Address: address, ResourceID: [32]byte{}, FeeAmount: big.NewInt(100000000)}
	s.feeAddress, _ = btcutil.DecodeAddress("tb1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2slafjvv", &chaincfg.TestNet3Params)
	s.mockConn = mock_listener.NewMockConnection(ctrl)
}

func (s *DecodeEventsSuite) Test_DecodeDepositEvent_ErrorDecodingOPRETURNData() {
	d1 := btcjson.TxRawResult{
		Vin: []btcjson.Vin{

			{
				Txid: "00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc",
			},
		},
		Vout: []btcjson.Vout{
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type: "nulldata",
					Hex:  "InvalidCharć",
				},
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1qln69zuhdunc9stwfh6t7adexxrcr04ppy6thgm",
				},
				Value: float64(0.00019),
			},
		},
	}

	deposit, isDeposit, err := listener.DecodeDepositEvent(d1, s.resource, s.feeAddress)
	s.Equal(isDeposit, true)
	s.NotNil(err)
	s.Equal(deposit, listener.Deposit{})
}

func (s *DecodeEventsSuite) Test_DecodeDepositEvent() {
	d1 := btcjson.TxRawResult{
		Vin: []btcjson.Vin{

			{
				Txid: "00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc",
			},
		},
		Vout: []btcjson.Vout{
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type: "nulldata",
					Hex:  "6a2c3078653966323341383238393736343238303639376130336143303637393565413932613137306534325f31",
				},
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1qln69zuhdunc9stwfh6t7adexxrcr04ppy6thgm",
				},
				Value: float64(0.00019),
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2slafjvv",
				},
				Value: float64(1),
			},
		},
	}
	deposit, isDeposit, err := listener.DecodeDepositEvent(d1, s.resource, s.feeAddress)
	s.Equal(isDeposit, true)
	s.Nil(err)
	s.Equal(deposit, listener.Deposit{
		ResourceID: [32]byte{},
		Amount:     big.NewInt(int64(d1.Vout[1].Value * 1e8)),
		Data:       "0xe9f23A8289764280697a03aC06795eA92a170e42_1",
	})
}

func (s *DecodeEventsSuite) Test_DecodeDepositEvent_FeeNotSent() {
	d1 := btcjson.TxRawResult{
		Vin: []btcjson.Vin{
			{
				Txid: "00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc",
			},
		},
		Vout: []btcjson.Vout{
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type: "nulldata",
					Hex:  "6a2c3078653966323341383238393736343238303639376130336143303637393565413932613137306534325f31",
				},
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1qln69zuhdunc9stwfh6t7adexxrcr04ppy6thgm",
				},
				Value: float64(0.00019),
			},
		},
	}
	deposit, isDeposit, err := listener.DecodeDepositEvent(d1, s.resource, s.feeAddress)
	s.Equal(isDeposit, false)
	s.Nil(err)
	s.Equal(deposit, listener.Deposit{})
}

func (s *DecodeEventsSuite) Test_DecodeDepositEvent_NotEnoughFeeSent() {
	d1 := btcjson.TxRawResult{
		Vin: []btcjson.Vin{
			{
				Txid: "00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc",
			},
		},
		Vout: []btcjson.Vout{
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type: "nulldata",
					Hex:  "6a2c3078653966323341383238393736343238303639376130336143303637393565413932613137306534325f31",
				},
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1qln69zuhdunc9stwfh6t7adexxrcr04ppy6thgm",
				},
				Value: float64(0.00019),
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2slafjvv",
				},
				Value: float64(0.9),
			},
		},
	}
	deposit, isDeposit, err := listener.DecodeDepositEvent(d1, s.resource, s.feeAddress)
	s.Equal(isDeposit, false)
	s.Nil(err)
	s.Equal(deposit, listener.Deposit{})
}

func (s *DecodeEventsSuite) Test_DecodeDepositEvent_NotBridgeDepositTx() {
	d1 := btcjson.TxRawResult{
		Vin: []btcjson.Vin{

			{
				Txid: "00000000000000000008bba5a6ff31fdb9bb1d4147905b5b3c47a07a07235bfc",
			},
		},
		Vout: []btcjson.Vout{
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type: "nulldata",
					Hex:  "6a2c3078653966323341383238393736343238303639376130336143303637393565413932613137306534325f31",
				},
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "NotBridgeAddress",
				},
				Value: float64(0.00019),
			},
			{
				ScriptPubKey: btcjson.ScriptPubKeyResult{
					Type:    "witness_v1_taproot",
					Address: "tb1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2slafjvv",
				},
				Value: float64(1),
			},
		},
	}
	deposit, isDeposit, err := listener.DecodeDepositEvent(d1, s.resource, s.feeAddress)
	s.Equal(isDeposit, false)
	s.Nil(err)
	s.Equal(deposit, listener.Deposit{})
}
