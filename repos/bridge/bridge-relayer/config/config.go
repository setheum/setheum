// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package config

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/creasty/defaults"
	"github.com/imdario/mergo"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

type Config struct {
	RelayerConfig relayer.RelayerConfig
	ChainConfigs  []map[string]interface{}
}

type RawConfig struct {
	RelayerConfig relayer.RawRelayerConfig `mapstructure:"relayer" json:"relayer"`
	ChainConfigs  []map[string]interface{} `mapstructure:"domains" json:"domains"`
}

// GetConfigFromENV reads config from Env variables, validates it and parses
// it into config suitable for application
//
// Properties of RelayerConfig are expected to be defined as separate Env variables
// where Env variable name reflects properties position in structure. Each Env variable needs to be prefixed with SYG.
//
// For example, if you want to set Config.RelayerConfig.MpcConfig.Port this would
// translate to Env variable named SYG_RELAYER_MPCCONFIG_PORT.
func GetConfigFromENV(config *Config) (*Config, error) {
	rawConfig, err := loadFromEnv()
	if err != nil {
		return config, err
	}

	return processRawConfig(rawConfig, config)
}

// GetConfigFromFile reads config from file, validates it and parses
// it into config suitable for application
func GetConfigFromFile(path string, config *Config) (*Config, error) {
	rawConfig := RawConfig{}

	viper.SetConfigFile(path)
	viper.SetConfigType("json")

	err := viper.ReadInConfig()
	if err != nil {
		return config, err
	}

	err = viper.Unmarshal(&rawConfig)
	if err != nil {
		return config, err
	}

	return processRawConfig(rawConfig, config)
}

// GetSharedConfigFromNetwork fetches shared configuration from URL and parses it.
func GetSharedConfigFromNetwork(url string) (*Config, error) {
	rawConfig := RawConfig{}
	config := &Config{}

	resp, err := http.Get(url)
	if err != nil {
		return &Config{}, err
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return &Config{}, err
	}

	err = json.Unmarshal(body, &rawConfig)
	if err != nil {
		return &Config{}, err
	}

	config.ChainConfigs = rawConfig.ChainConfigs
	return config, err
}

func processRawConfig(rawConfig RawConfig, config *Config) (*Config, error) {
	if err := defaults.Set(&rawConfig); err != nil {
		return config, err
	}

	relayerConfig, err := relayer.NewRelayerConfig(rawConfig.RelayerConfig)
	if err != nil {
		return config, err
	}
	if config == nil {
		config := &Config{}
		config.RelayerConfig = relayerConfig
		config.ChainConfigs = rawConfig.ChainConfigs
		return config, nil
	}

	chainConfigs := make([]map[string]interface{}, 0)
	for i, chain := range rawConfig.ChainConfigs {
		if chain["id"] == 0 || chain["id"] == nil {
			return config, fmt.Errorf("chain 'id' not configured for chain %d", i)
		}
		if chain["type"] == "" || chain["type"] == nil {
			return config, fmt.Errorf("chain 'type' must be provided for every configured chain")
		}

		chainConfig, err := findChainConfig(chain["id"], config.ChainConfigs)
		if err != nil {
			return config, err
		}

		err = mergo.Merge(&chain, chainConfig)
		if err != nil {
			return config, err
		}

		chainConfigs = append(chainConfigs, chain)
	}

	config.ChainConfigs = chainConfigs
	config.RelayerConfig = relayerConfig
	return config, nil
}

func findChainConfig(id interface{}, configs []map[string]interface{}) (interface{}, error) {
	for _, config := range configs {
		if compareDomainID(id, config["id"]) {
			return config, nil
		}
	}

	return nil, fmt.Errorf("missing chain %v", id)
}

func compareDomainID(a, b interface{}) bool {
	switch a := a.(type) {
	case int:
		switch b := b.(type) {
		case int:
			return a == b
		case float64:
			return float64(a) == b
		}
	case float64:
		switch b := b.(type) {
		case int:
			return a == float64(b)
		case float64:
			return a == b
		}
	}
	return false
}

var (
	// Flags for running the app
	ConfigFlagName      = "config"
	KeystoreFlagName    = "keystore"
	BlockstoreFlagName  = "blockstore"
	FreshStartFlagName  = "fresh"
	LatestBlockFlagName = "latest"
)

func BindFlags(rootCMD *cobra.Command) {
	rootCMD.PersistentFlags().String(ConfigFlagName, ".", "Path to JSON configuration file")
	_ = viper.BindPFlag(ConfigFlagName, rootCMD.PersistentFlags().Lookup(ConfigFlagName))

	rootCMD.PersistentFlags().String(BlockstoreFlagName, "./lvldbdata", "Specify path for blockstore")
	_ = viper.BindPFlag(BlockstoreFlagName, rootCMD.PersistentFlags().Lookup(BlockstoreFlagName))

	rootCMD.PersistentFlags().Bool(FreshStartFlagName, false, "Disables loading from blockstore at start. Opts will still be used if specified. (default: false)")
	_ = viper.BindPFlag(FreshStartFlagName, rootCMD.PersistentFlags().Lookup(FreshStartFlagName))

	rootCMD.PersistentFlags().Bool(LatestBlockFlagName, false, "Overrides blockstore and start block, starts from latest block (default: false)")
	_ = viper.BindPFlag(LatestBlockFlagName, rootCMD.PersistentFlags().Lookup(LatestBlockFlagName))

	rootCMD.PersistentFlags().String(KeystoreFlagName, "./keys", "Path to keystore directory")
	_ = viper.BindPFlag(KeystoreFlagName, rootCMD.PersistentFlags().Lookup(KeystoreFlagName))
}
