// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package chain

import (
	"fmt"

	"github.com/ChainSafe/sygma-relayer/config"
	"github.com/spf13/viper"
)

type GeneralChainConfig struct {
	Name           string `mapstructure:"name"`
	Id             *uint8 `mapstructure:"id"`
	Endpoint       string `mapstructure:"endpoint"`
	Type           string `mapstructure:"type"`
	BlockstorePath string `mapstructure:"blockstorePath"`
	FreshStart     bool   `mapstructure:"fresh"`
	LatestBlock    bool   `mapstructure:"latest"`
	Key            string
	Insecure       bool
}

func (c *GeneralChainConfig) Validate() error {
	// viper defaults to 0 for not specified ints
	if c.Id == nil {
		return fmt.Errorf("required field domain.Id empty for chain %v", c.Id)
	}
	if c.Endpoint == "" {
		return fmt.Errorf("required field chain.Endpoint empty for chain %v", *c.Id)
	}
	if c.Name == "" {
		return fmt.Errorf("required field chain.Name empty for chain %v", *c.Id)
	}
	return nil
}

func (c *GeneralChainConfig) ParseFlags() {
	blockstore := viper.GetString(config.BlockstoreFlagName)
	if blockstore != "" {
		c.BlockstorePath = blockstore
	}
	freshStart := viper.GetBool(config.FreshStartFlagName)
	if freshStart {
		c.FreshStart = freshStart
	}
	latestBlock := viper.GetBool(config.LatestBlockFlagName)
	if latestBlock {
		c.LatestBlock = latestBlock
	}
}
