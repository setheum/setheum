// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package evm

import (
	"fmt"
	"math/big"
	"time"

	"github.com/creasty/defaults"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/mitchellh/mapstructure"

	"github.com/ChainSafe/sygma-relayer/config/chain"
	"github.com/sygmaprotocol/sygma-core/crypto/secp256k1"
)

type HandlerConfig struct {
	Address string
	Type    string
}

type EVMConfig struct {
	GeneralChainConfig    chain.GeneralChainConfig
	Bridge                string
	Retry                 string
	FrostKeygen           string
	Handlers              []HandlerConfig
	MaxGasPrice           *big.Int
	GasMultiplier         *big.Float
	GasLimit              *big.Int
	TransferGas           uint64
	GasIncreasePercentage *big.Int
	StartBlock            *big.Int
	BlockConfirmations    *big.Int
	BlockInterval         *big.Int
	BlockRetryInterval    time.Duration
}

func (c *EVMConfig) String() string {
	privateKey, _ := crypto.HexToECDSA(c.GeneralChainConfig.Key)
	kp := secp256k1.NewKeypair(*privateKey)
	return fmt.Sprintf(`Name: '%s', Id: '%d', Type: '%s', BlockstorePath: '%s', FreshStart: '%t', LatestBlock: '%t', Key address: '%s', Bridge: '%s', Retry: '%s', Handlers: %+v, MaxGasPrice: '%s', GasMultiplier: '%s', GasLimit: '%s', TransferGas: '%d', StartBlock: '%s', BlockConfirmations: '%s', BlockInterval: '%s', BlockRetryInterval: '%s'`,
		c.GeneralChainConfig.Name,
		*c.GeneralChainConfig.Id,
		c.GeneralChainConfig.Type,
		c.GeneralChainConfig.BlockstorePath,
		c.GeneralChainConfig.FreshStart,
		c.GeneralChainConfig.LatestBlock,
		kp.Address(),
		c.Bridge,
		c.Retry,
		c.Handlers,
		c.MaxGasPrice,
		c.GasMultiplier,
		c.GasLimit,
		c.TransferGas,
		c.StartBlock,
		c.BlockConfirmations,
		c.BlockInterval,
		c.BlockRetryInterval,
	)
}

type RawEVMConfig struct {
	chain.GeneralChainConfig `mapstructure:",squash"`
	Bridge                   string          `mapstructure:"bridge"`
	Retry                    string          `mapstructure:"retry"`
	FrostKeygen              string          `mapstructure:"frostKeygen"`
	Handlers                 []HandlerConfig `mapstrcture:"handlers"`
	MaxGasPrice              int64           `mapstructure:"maxGasPrice" default:"500000000000"`
	GasMultiplier            float64         `mapstructure:"gasMultiplier" default:"1"`
	GasIncreasePercentage    int64           `mapstructure:"gasIncreasePercentage" default:"15"`
	GasLimit                 int64           `mapstructure:"gasLimit" default:"15000000"`
	TransferGas              uint64          `mapstructure:"transferGas" default:"250000"`
	StartBlock               int64           `mapstructure:"startBlock"`
	BlockConfirmations       int64           `mapstructure:"blockConfirmations" default:"10"`
	BlockInterval            int64           `mapstructure:"blockInterval" default:"5"`
	BlockRetryInterval       uint64          `mapstructure:"blockRetryInterval" default:"5"`
}

func (c *RawEVMConfig) Validate() error {
	if err := c.GeneralChainConfig.Validate(); err != nil {
		return err
	}
	if c.Bridge == "" {
		return fmt.Errorf("required field chain.Bridge empty for chain %v", *c.Id)
	}
	if c.BlockConfirmations < 1 {
		return fmt.Errorf("blockConfirmations has to be >=1")
	}
	return nil
}

// NewEVMConfig decodes and validates an instance of an EVMConfig from
// raw chain config
func NewEVMConfig(chainConfig map[string]interface{}) (*EVMConfig, error) {
	var c RawEVMConfig
	err := mapstructure.Decode(chainConfig, &c)
	if err != nil {
		return nil, err
	}

	err = defaults.Set(&c)
	if err != nil {
		return nil, err
	}

	err = c.Validate()
	if err != nil {
		return nil, err
	}

	c.GeneralChainConfig.ParseFlags()
	config := &EVMConfig{
		GeneralChainConfig:    c.GeneralChainConfig,
		Handlers:              c.Handlers,
		Bridge:                c.Bridge,
		Retry:                 c.Retry,
		FrostKeygen:           c.FrostKeygen,
		BlockRetryInterval:    time.Duration(c.BlockRetryInterval) * time.Second,
		GasLimit:              big.NewInt(c.GasLimit),
		TransferGas:           c.TransferGas,
		MaxGasPrice:           big.NewInt(c.MaxGasPrice),
		GasIncreasePercentage: big.NewInt(c.GasIncreasePercentage),
		GasMultiplier:         big.NewFloat(c.GasMultiplier),
		StartBlock:            big.NewInt(c.StartBlock),
		BlockConfirmations:    big.NewInt(c.BlockConfirmations),
		BlockInterval:         big.NewInt(c.BlockInterval),
	}

	return config, nil
}
