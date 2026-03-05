// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package config_test

import (
	"encoding/json"
	"os"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/config"
	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/stretchr/testify/suite"
)

type GetConfigTestSuite struct {
	suite.Suite
}

func TestRunGetConfigTestSuite(t *testing.T) {
	suite.Run(t, new(GetConfigTestSuite))
}

func (s *GetConfigTestSuite) TearDownTest() {
	os.Clearenv()
}

func (s *GetConfigTestSuite) Test_GetConfigFromFile_InvalidPath() {
	_, err := config.GetConfigFromFile("invalid", &config.Config{})

	s.NotNil(err)
}

func (s *GetConfigTestSuite) Test_GetConfigFromENV() {
	_ = os.Setenv("SYG_CHAINS", `[
   {
      "id":1,
      "from":"0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
      "name":"evm1",
      "type":"evm",
      "endpoint":"ws://evm1-1:8546",
      "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
      "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
      "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
      "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
      "maxGasPrice":20000000000,
      "blockConfirmations":2
   },
	 {
   "id":2,
   "from":"0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
   "name":"evm2",
   "type":"evm",
   "endpoint":"ws://evm2-1:8546",
   "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
   "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
   "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
   "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
   "maxGasPrice":20000000000,
   "blockConfirmations":2
}
]`)

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEY", "test-pk")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEYSHAREPATH", "/cfg/keyshares/0.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_FROSTKEYSHAREPATH", "/cfg/keyshares/0-frost.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_PORT", "9000")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_ENCRYPTIONKEY", "test-enc-key")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_URL", "http://test.com")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_PATH", "path")
	_ = os.Setenv("SYG_RELAYER_ENV", "TEST")
	_ = os.Setenv("SYG_RELAYER_ID", "123")

	// load from Env
	cnf, err := config.GetConfigFromENV(&config.Config{ChainConfigs: []map[string]interface{}{{
		"id":                 1,
		"blockConfirmations": 5,
		"gasLimit":           500,
	}, {
		"id":                 2,
		"blockConfirmations": 3,
		"gasLimit":           300,
	}}})

	s.Nil(err)

	s.Equal(config.Config{
		RelayerConfig: relayer.RelayerConfig{
			LogLevel:   1,
			LogFile:    "out.log",
			Env:        "TEST",
			Id:         "123",
			HealthPort: 9001,
			MpcConfig: relayer.MpcRelayerConfig{
				TopologyConfiguration: relayer.TopologyConfiguration{
					EncryptionKey: "test-enc-key",
					Url:           "http://test.com",
					Path:          "path",
				},
				Port:                    9000,
				KeysharePath:            "/cfg/keyshares/0.keyshare",
				FrostKeysharePath:       "/cfg/keyshares/0-frost.keyshare",
				Key:                     "test-pk",
				CommHealthCheckInterval: 5 * time.Minute,
			},
			BullyConfig: relayer.BullyConfig{
				PingWaitTime:     1 * time.Second,
				PingBackOff:      1 * time.Second,
				PingInterval:     1 * time.Second,
				ElectionWaitTime: 2 * time.Second,
				BullyWaitTime:    3 * time.Minute,
			},
			UploaderConfig: relayer.UploaderConfig{
				MaxRetries:     5,
				MaxElapsedTime: 300000,
			},
		},
		ChainConfigs: []map[string]interface{}{
			{
				"id":                 float64(1),
				"type":               "evm",
				"bridge":             "0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
				"erc721Handler":      "0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
				"gasLimit":           500,
				"maxGasPrice":        2e+10,
				"from":               "0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
				"name":               "evm1",
				"endpoint":           "ws://evm1-1:8546",
				"erc20Handler":       "0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
				"genericHandler":     "0xe1588E2c6a002AE93AeD325A910Ed30961874109",
				"blockConfirmations": float64(2),
			},
			{
				"id":                 float64(2),
				"type":               "evm",
				"bridge":             "0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
				"erc721Handler":      "0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
				"gasLimit":           300,
				"maxGasPrice":        2e+10,
				"from":               "0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
				"name":               "evm2",
				"endpoint":           "ws://evm2-1:8546",
				"erc20Handler":       "0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
				"genericHandler":     "0xe1588E2c6a002AE93AeD325A910Ed30961874109",
				"blockConfirmations": float64(2),
			},
		},
	}, *cnf)
}

func (s *GetConfigTestSuite) Test_GetConfigFromENV_RandomOrder() {
	_ = os.Setenv("SYG_CHAINS", `[
   {
      "id":1,
      "from":"0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
      "name":"evm1",
      "type":"evm",
      "endpoint":"ws://evm1-1:8546",
      "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
      "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
      "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
      "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
      "maxGasPrice":20000000000,
      "blockConfirmations":2
   },
	 {
   "id":2,
   "from":"0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
   "name":"evm2",
   "type":"evm",
   "endpoint":"ws://evm2-1:8546",
   "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
   "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
   "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
   "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
   "maxGasPrice":20000000000,
   "blockConfirmations":2
}
]`)

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEY", "test-pk")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEYSHAREPATH", "/cfg/keyshares/0.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_FROSTKEYSHAREPATH", "/cfg/keyshares/0-frost.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_PORT", "9000")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_ENCRYPTIONKEY", "test-enc-key")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_URL", "http://test.com")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_PATH", "path")
	_ = os.Setenv("SYG_RELAYER_ENV", "TEST")
	_ = os.Setenv("SYG_RELAYER_ID", "123")

	// load from Env
	cnf, err := config.GetConfigFromENV(&config.Config{ChainConfigs: []map[string]interface{}{{
		"id":                 2,
		"blockConfirmations": 3,
		"gasLimit":           300,
	}, {
		"id":                 1,
		"blockConfirmations": 5,
		"gasLimit":           500,
	}}})

	s.Nil(err)

	s.Equal(config.Config{
		RelayerConfig: relayer.RelayerConfig{
			LogLevel:   1,
			LogFile:    "out.log",
			Env:        "TEST",
			Id:         "123",
			HealthPort: 9001,
			MpcConfig: relayer.MpcRelayerConfig{
				TopologyConfiguration: relayer.TopologyConfiguration{
					EncryptionKey: "test-enc-key",
					Url:           "http://test.com",
					Path:          "path",
				},
				Port:                    9000,
				KeysharePath:            "/cfg/keyshares/0.keyshare",
				FrostKeysharePath:       "/cfg/keyshares/0-frost.keyshare",
				Key:                     "test-pk",
				CommHealthCheckInterval: 5 * time.Minute,
			},
			BullyConfig: relayer.BullyConfig{
				PingWaitTime:     1 * time.Second,
				PingBackOff:      1 * time.Second,
				PingInterval:     1 * time.Second,
				ElectionWaitTime: 2 * time.Second,
				BullyWaitTime:    3 * time.Minute,
			},
			UploaderConfig: relayer.UploaderConfig{
				MaxRetries:     5,
				MaxElapsedTime: 300000,
			},
		},
		ChainConfigs: []map[string]interface{}{
			{
				"id":                 float64(1),
				"type":               "evm",
				"bridge":             "0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
				"erc721Handler":      "0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
				"gasLimit":           500,
				"maxGasPrice":        2e+10,
				"from":               "0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
				"name":               "evm1",
				"endpoint":           "ws://evm1-1:8546",
				"erc20Handler":       "0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
				"genericHandler":     "0xe1588E2c6a002AE93AeD325A910Ed30961874109",
				"blockConfirmations": float64(2),
			},
			{
				"id":                 float64(2),
				"type":               "evm",
				"bridge":             "0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
				"erc721Handler":      "0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
				"gasLimit":           300,
				"maxGasPrice":        2e+10,
				"from":               "0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
				"name":               "evm2",
				"endpoint":           "ws://evm2-1:8546",
				"erc20Handler":       "0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
				"genericHandler":     "0xe1588E2c6a002AE93AeD325A910Ed30961874109",
				"blockConfirmations": float64(2),
			},
		},
	}, *cnf)
}

func (s *GetConfigTestSuite) Test_SharedConfigLengthMismatch() {
	_ = os.Setenv("SYG_CHAINS", `[
   {
      "id":1,
      "from":"0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
      "name":"evm1",
      "type":"evm",
      "endpoint":"ws://evm1-1:8546",
      "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
      "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
      "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
      "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
      "maxGasPrice":20000000000,
      "blockConfirmations":2
   },
	 {
   "id":2,
   "from":"0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
   "name":"evm2",
   "type":"evm",
   "endpoint":"ws://evm2-1:8546",
   "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
   "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
   "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
   "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
   "maxGasPrice":20000000000,
   "blockConfirmations":2
}
]`)

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEY", "test-pk")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEYSHAREPATH", "/cfg/keyshares/0.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_PORT", "9000")

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_ENCRYPTIONKEY", "test-enc-key")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_URL", "http://test.com")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_PATH", "path")

	// load from Env
	_, err := config.GetConfigFromENV(&config.Config{ChainConfigs: []map[string]interface{}{{
		"id":                 1,
		"blockConfirmations": 5,
		"gasLimit":           500,
	}}})

	s.NotNil(err)
}

type ConfigTestCase struct {
	name       string
	inConfig   config.RawConfig
	shouldFail bool
	errorMsg   string
	outConfig  config.Config
}

func (s *GetConfigTestSuite) Test_GetConfigFromFile() {
	testCases := []ConfigTestCase{
		{
			name: "missing chain type",
			inConfig: config.RawConfig{
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"name": "chain1",
				}},
				RelayerConfig: relayer.RawRelayerConfig{
					OpenTelemetryCollectorURL: "",
					LogLevel:                  "",
					LogFile:                   "",
					MpcConfig: relayer.RawMpcRelayerConfig{
						Port: "2020",
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
					},
					BullyConfig: relayer.RawBullyConfig{
						PingWaitTime:     "1s",
						PingBackOff:      "1s",
						PingInterval:     "1s",
						ElectionWaitTime: "1s",
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
			},
			shouldFail: true,
			errorMsg:   "chain 'type' must be provided for every configured chain",
			outConfig:  config.Config{},
		},
		{
			name: "invalid relayer type",
			inConfig: config.RawConfig{
				RelayerConfig: relayer.RawRelayerConfig{
					LogLevel: "invalid",
					MpcConfig: relayer.RawMpcRelayerConfig{
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "chain1",
				}},
			},
			shouldFail: true,
			errorMsg:   "unknown log level: invalid",
			outConfig:  config.Config{},
		},
		{
			name: "invalid bully config",
			inConfig: config.RawConfig{
				RelayerConfig: relayer.RawRelayerConfig{
					LogLevel: "info",
					MpcConfig: relayer.RawMpcRelayerConfig{
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
						Port: "2020",
					},
					BullyConfig: relayer.RawBullyConfig{
						PingWaitTime:     "2z",
						PingBackOff:      "",
						PingInterval:     "",
						ElectionWaitTime: "",
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "chain1",
				}},
			},
			shouldFail: true,
			errorMsg:   "unable to parse bully ping wait time: time: unknown unit \"z\" in duration \"2z\"",
			outConfig:  config.Config{},
		},
		{
			name: "missing encryption key",
			inConfig: config.RawConfig{
				RelayerConfig: relayer.RawRelayerConfig{
					LogLevel: "info",
					MpcConfig: relayer.RawMpcRelayerConfig{
						TopologyConfiguration: relayer.TopologyConfiguration{},
						Port:                  "2020",
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},

				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "chain1",
				}},
			},
			shouldFail: true,
			errorMsg:   "topology configuration encryption key not provided",
			outConfig:  config.Config{},
		},
		{
			name: "set default values in config",
			inConfig: config.RawConfig{
				RelayerConfig: relayer.RawRelayerConfig{
					// LogLevel: use default value,
					// LogFile: use default value
					MpcConfig: relayer.RawMpcRelayerConfig{
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
						// Port: use default value,
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "evm1",
				}},
			},
			shouldFail: false,
			errorMsg:   "unable to parse bully ping wait time: time: unknown unit \"z\" in duration \"2z\"",
			outConfig: config.Config{
				RelayerConfig: relayer.RelayerConfig{
					LogLevel:                  1,
					LogFile:                   "out.log",
					OpenTelemetryCollectorURL: "",
					HealthPort:                9001,
					MpcConfig: relayer.MpcRelayerConfig{
						Port: 9000,
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
						CommHealthCheckInterval: 5 * time.Minute,
					},
					BullyConfig: relayer.BullyConfig{
						PingWaitTime:     1 * time.Second,
						PingBackOff:      1 * time.Second,
						PingInterval:     1 * time.Second,
						ElectionWaitTime: 2 * time.Second,
						BullyWaitTime:    3 * time.Minute,
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "evm1",
				}},
			},
		},
		{
			name: "valid config",
			inConfig: config.RawConfig{
				RelayerConfig: relayer.RawRelayerConfig{
					LogLevel:   "debug",
					LogFile:    "custom.log",
					HealthPort: "9002",
					MpcConfig: relayer.RawMpcRelayerConfig{
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
						Port:                    "2020",
						KeysharePath:            "./share.key",
						Key:                     "./key.pk",
						CommHealthCheckInterval: "10m",
					},
					BullyConfig: relayer.RawBullyConfig{
						PingWaitTime:     "1s",
						PingBackOff:      "1s",
						PingInterval:     "1s",
						ElectionWaitTime: "1s",
						BullyWaitTime:    "1s",
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "evm1",
				}},
			},
			shouldFail: false,
			errorMsg:   "unable to parse bully ping wait time: time: unknown unit \"z\" in duration \"2z\"",
			outConfig: config.Config{
				RelayerConfig: relayer.RelayerConfig{
					LogLevel:                  0,
					LogFile:                   "custom.log",
					OpenTelemetryCollectorURL: "",
					HealthPort:                9002,
					MpcConfig: relayer.MpcRelayerConfig{
						Port:         2020,
						KeysharePath: "./share.key",
						Key:          "./key.pk",
						TopologyConfiguration: relayer.TopologyConfiguration{
							EncryptionKey: "enc-key",
							Url:           "url",
							Path:          "path",
						},
						CommHealthCheckInterval: 10 * time.Minute,
					},
					BullyConfig: relayer.BullyConfig{
						PingWaitTime:     time.Second,
						PingBackOff:      time.Second,
						PingInterval:     time.Second,
						ElectionWaitTime: time.Second,
						BullyWaitTime:    time.Second,
					},
					UploaderConfig: relayer.UploaderConfig{
						URL:            "https://testIPFSProvider.com",
						AuthToken:      "testToken",
						MaxRetries:     5,
						MaxElapsedTime: 5 * time.Minute,
					},
				},
				ChainConfigs: []map[string]interface{}{{
					"id":   float64(1),
					"type": "evm",
					"name": "evm1",
				}},
			},
		},
	}

	for _, t := range testCases {
		s.Run(t.name, func() {
			file, _ := json.Marshal(t.inConfig)
			_ = os.WriteFile("test.json", file, 0644)

			conf, err := config.GetConfigFromFile("test.json", &config.Config{
				ChainConfigs: []map[string]interface{}{
					{
						"id": 1,
					},
				},
			})

			_ = os.Remove("test.json")

			if t.shouldFail {
				s.NotNil(err)
				s.Equal(t.errorMsg, err.Error())
			} else {
				s.Nil(err)
				s.Equal(t.outConfig, *conf)
			}
		})
	}
}
