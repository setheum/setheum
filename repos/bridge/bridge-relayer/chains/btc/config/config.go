// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package config

import (
	"encoding/hex"
	"fmt"
	"math/big"
	"time"

	"github.com/ChainSafe/sygma-relayer/config/chain"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/creasty/defaults"
	"github.com/mitchellh/mapstructure"
)

type RawResource struct {
	Address    string
	ResourceID string
	FeeAmount  string
	Tweak      string
	Script     string
}

type Resource struct {
	Address    btcutil.Address
	FeeAmount  *big.Int
	ResourceID [32]byte
	Tweak      string
	Script     []byte
}

type RawBtcConfig struct {
	chain.GeneralChainConfig `mapstructure:",squash"`
	Resources                []RawResource `mapstrcture:"resources"`
	StartBlock               int64         `mapstructure:"startBlock"`
	FeeAddress               string        `mapstructure:"feeAddress"`
	Username                 string        `mapstructure:"username"`
	Password                 string        `mapstructure:"password"`
	BlockInterval            int64         `mapstructure:"blockInterval" default:"5"`
	BlockRetryInterval       uint64        `mapstructure:"blockRetryInterval" default:"5"`
	BlockConfirmations       int64         `mapstructure:"blockConfirmations" default:"10"`
	Network                  string        `mapstructure:"network" default:"mainnet"`
	MempoolUrl               string        `mapstructure:"mempoolUrl"`
}

func (c *RawBtcConfig) Validate() error {
	if err := c.GeneralChainConfig.Validate(); err != nil {
		return err
	}

	if c.BlockConfirmations < 1 {
		return fmt.Errorf("blockConfirmations has to be >=1")
	}

	if c.Username == "" {
		return fmt.Errorf("required field chain.Username empty for chain %v", *c.Id)
	}

	if c.Password == "" {
		return fmt.Errorf("required field chain.Password empty for chain %v", *c.Id)
	}
	return nil
}

type BtcConfig struct {
	GeneralChainConfig chain.GeneralChainConfig
	Resources          []Resource
	FeeAddress         btcutil.Address
	Username           string
	Password           string
	StartBlock         *big.Int
	BlockInterval      *big.Int
	BlockRetryInterval time.Duration
	BlockConfirmations *big.Int
	Tweak              string
	Script             []byte
	MempoolUrl         string
	Network            chaincfg.Params
}

// NewBtcConfig decodes and validates an instance of an BtcConfig from
// raw chain config
func NewBtcConfig(chainConfig map[string]interface{}) (*BtcConfig, error) {
	var c RawBtcConfig
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

	networkParams, err := networkParams(c.Network)
	if err != nil {
		return nil, err
	}
	feeAddress, err := btcutil.DecodeAddress(c.FeeAddress, &networkParams)
	if err != nil {
		return nil, err
	}
	resources := make([]Resource, len(c.Resources))
	for i, r := range c.Resources {
		scriptBytes, err := hex.DecodeString(r.Script)
		if err != nil {
			return nil, err
		}

		feeAmount, success := new(big.Int).SetString(r.FeeAmount, 10)
		if !success {
			return nil, fmt.Errorf("error: could not convert string to *big.Int")
		}

		address, err := btcutil.DecodeAddress(r.Address, &networkParams)
		if err != nil {
			return nil, err
		}
		resourceBytes, err := hex.DecodeString(r.ResourceID[2:])
		if err != nil {
			panic(err)
		}
		var resource32Bytes [32]byte
		copy(resource32Bytes[:], resourceBytes)
		resources[i] = Resource{
			Address:    address,
			ResourceID: resource32Bytes,
			Script:     scriptBytes,
			Tweak:      r.Tweak,
			FeeAmount:  feeAmount,
		}
	}

	c.GeneralChainConfig.ParseFlags()
	config := &BtcConfig{
		GeneralChainConfig: c.GeneralChainConfig,
		StartBlock:         big.NewInt(c.StartBlock),
		BlockConfirmations: big.NewInt(c.BlockConfirmations),
		BlockInterval:      big.NewInt(c.BlockInterval),
		BlockRetryInterval: time.Duration(c.BlockRetryInterval) * time.Second,
		Username:           c.Username,
		Password:           c.Password,
		Network:            networkParams,
		MempoolUrl:         c.MempoolUrl,
		FeeAddress:         feeAddress,
		Resources:          resources,
	}
	return config, nil
}

func networkParams(network string) (chaincfg.Params, error) {
	switch network {
	case "mainnet":
		return chaincfg.MainNetParams, nil
	case "testnet":
		return chaincfg.TestNet3Params, nil
	case "regtest":
		return chaincfg.RegressionNetParams, nil
	case "signet":
		return chaincfg.SigNetParams, nil
	default:
		return chaincfg.Params{}, fmt.Errorf("unknown network %s", network)
	}
}
