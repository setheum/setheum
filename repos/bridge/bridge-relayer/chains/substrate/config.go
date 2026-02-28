// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package substrate

import (
	"fmt"
	"math/big"
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/creasty/defaults"
	"github.com/mitchellh/mapstructure"

	"github.com/ChainSafe/sygma-relayer/config/chain"
)

type RawSubstrateConfig struct {
	chain.GeneralChainConfig `mapstructure:",squash"`
	ChainID                  int64  `mapstructure:"chainID"`
	StartBlock               int64  `mapstructure:"startBlock"`
	BlockInterval            int64  `mapstructure:"blockInterval" default:"5"`
	BlockRetryInterval       uint64 `mapstructure:"blockRetryInterval" default:"5"`
	SubstrateNetwork         int64  `mapstructure:"substrateNetwork"`
	Tip                      uint64 `mapstructure:"tip"`
}

type SubstrateConfig struct {
	GeneralChainConfig chain.GeneralChainConfig
	ChainID            *big.Int
	StartBlock         *big.Int
	BlockInterval      *big.Int
	BlockRetryInterval time.Duration
	SubstrateNetwork   uint16
	Tip                uint64
}

func (c *SubstrateConfig) String() string {
	kp, _ := signature.KeyringPairFromSecret(c.GeneralChainConfig.Key, c.SubstrateNetwork)
	return fmt.Sprintf(`Name: '%s', Id: '%d', Type: '%s', BlockstorePath: '%s', FreshStart: '%t', 
							  LatestBlock: '%t', Key address: '%s', StartBlock: '%s', BlockInterval: '%s', 
                              BlockRetryInterval: '%s', ChainID: '%d', Tip: '%d', SubstrateNetworkPrefix: "%d"`,
		c.GeneralChainConfig.Name,
		*c.GeneralChainConfig.Id,
		c.GeneralChainConfig.Type,
		c.GeneralChainConfig.BlockstorePath,
		c.GeneralChainConfig.FreshStart,
		c.GeneralChainConfig.LatestBlock,
		kp.Address,
		c.StartBlock,
		c.BlockInterval,
		c.BlockRetryInterval,
		c.ChainID,
		c.Tip,
		c.SubstrateNetwork,
	)
}

func (c *RawSubstrateConfig) Validate() error {
	if err := c.GeneralChainConfig.Validate(); err != nil {
		return err
	}

	return nil
}

// NewSubstrateConfig decodes and validates an instance of an SubstrateConfig from
// raw chain config
func NewSubstrateConfig(chainConfig map[string]interface{}) (*SubstrateConfig, error) {
	var c RawSubstrateConfig
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
	config := &SubstrateConfig{
		GeneralChainConfig: c.GeneralChainConfig,
		ChainID:            big.NewInt(c.ChainID),
		BlockRetryInterval: time.Duration(c.BlockRetryInterval) * time.Second,
		StartBlock:         big.NewInt(c.StartBlock),
		BlockInterval:      big.NewInt(c.BlockInterval),
		SubstrateNetwork:   uint16(c.SubstrateNetwork),
		Tip:                uint64(c.Tip),
	}

	return config, nil
}
