// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package config

import (
	"os"
	"testing"

	"github.com/stretchr/testify/suite"

	"github.com/ChainSafe/sygma-relayer/config/relayer"
)

type LoadFromEnvTestSuite struct {
	suite.Suite
}

func (s *LoadFromEnvTestSuite) TearDownTest() {
	os.Clearenv()
}

func TestRunLoadFromEnvTestSuite(t *testing.T) {
	suite.Run(t, new(LoadFromEnvTestSuite))
}

func (s *LoadFromEnvTestSuite) SetupTest() {
	os.Clearenv()
}

func (s *LoadFromEnvTestSuite) Test_ValidRelayerConfig() {
	_ = os.Setenv("SYG_RELAYER_OPENTELEMETRYCOLLECTORURL", "test.opentelemetry.url")
	_ = os.Setenv("SYG_RELAYER_LOGLEVEL", "info")
	_ = os.Setenv("SYG_RELAYER_LOGFILE", "test.log")
	_ = os.Setenv("SYG_RELAYER_HEALTHPORT", "4000")

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEY", "test-pk")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEYSHAREPATH", "/cfg/keyshares/0.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_PORT", "9000")

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_ENCRYPTIONKEY", "test-encryption-key")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_URL", "url")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_PATH", "path")

	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGBACKOFF", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGINTERVAL", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_ELECTIONWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_BULLYWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_ENV", "TEST")
	_ = os.Setenv("SYG_RELAYER_ID", "123")

	env, err := loadFromEnv()

	s.Nil(err)
	s.Equal(relayer.RawRelayerConfig{
		OpenTelemetryCollectorURL: "test.opentelemetry.url",
		LogLevel:                  "info",
		LogFile:                   "test.log",
		Id:                        "123",
		Env:                       "TEST",
		HealthPort:                "4000",
		MpcConfig: relayer.RawMpcRelayerConfig{
			KeysharePath: "/cfg/keyshares/0.keyshare",
			Key:          "test-pk",
			Port:         "9000",
			TopologyConfiguration: relayer.TopologyConfiguration{
				EncryptionKey: "test-encryption-key",
				Url:           "url",
				Path:          "path",
			},
		},
		BullyConfig: relayer.RawBullyConfig{
			PingWaitTime:     "2s",
			PingBackOff:      "2s",
			PingInterval:     "2s",
			ElectionWaitTime: "2s",
			BullyWaitTime:    "2s",
		},
	}, env.RelayerConfig)
}

func (s *LoadFromEnvTestSuite) Test_ValidChainConfig() {
	_ = os.Setenv("SYG_RELAYER_OPENTELEMETRYCOLLECTORURL", "test.opentelemetry.url")
	_ = os.Setenv("SYG_RELAYER_LOGLEVEL", "info")
	_ = os.Setenv("SYG_RELAYER_LOGFILE", "test.log")
	_ = os.Setenv("SYG_RELAYER_HEALTHPORT", "4000")

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEY", "test-pk")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEYSHAREPATH", "/cfg/keyshares/0.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_PORT", "9000")

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_ENCRYPTIONKEY", "test-encryption-key")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_URL", "url")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_PATH", "path")

	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGBACKOFF", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGINTERVAL", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_ELECTIONWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_BULLYWAITTIME", "2s")
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

	env, err := loadFromEnv()

	s.Nil(err)
	s.Equal([]map[string]interface{}{
		{
			"id":                 float64(1),
			"type":               "evm",
			"bridge":             "0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
			"erc721Handler":      "0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
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
			"maxGasPrice":        2e+10,
			"from":               "0xff93B45308FD417dF303D6515aB04D9e89a750Ca",
			"name":               "evm2",
			"endpoint":           "ws://evm2-1:8546",
			"erc20Handler":       "0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
			"genericHandler":     "0xe1588E2c6a002AE93AeD325A910Ed30961874109",
			"blockConfirmations": float64(2),
		}}, env.ChainConfigs)
}

func (s *LoadFromEnvTestSuite) Test_InvalidChainConfig() {
	_ = os.Setenv("SYG_RELAYER_OPENTELEMETRYCOLLECTORURL", "test.opentelemetry.url")
	_ = os.Setenv("SYG_RELAYER_LOGLEVEL", "info")
	_ = os.Setenv("SYG_RELAYER_LOGFILE", "test.log")
	_ = os.Setenv("SYG_RELAYER_HEALTHPORT", "4000")

	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEY", "test-pk")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_KEYSHAREPATH", "/cfg/keyshares/0.keyshare")
	_ = os.Setenv("SYG_RELAYER_MPCCONFIG_PORT", "9000")

	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGBACKOFF", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_PINGINTERVAL", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_ELECTIONWAITTIME", "2s")
	_ = os.Setenv("SYG_RELAYER_BULLYCONFIG_BULLYWAITTIME", "2s")
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
   "endpoint":"ws://evm2-1:8546",
   "bridge":"0xd606A00c1A39dA53EA7Bb3Ab570BBE40b156EB66",
   "erc20Handler":"0x3cA3808176Ad060Ad80c4e08F30d85973Ef1d99e",
   "erc721Handler":"0x75dF75bcdCa8eA2360c562b4aaDBAF3dfAf5b19b",
   "genericHandler":"0xe1588E2c6a002AE93AeD325A910Ed30961874109",
   "maxGasPrice":20000000000,
   "blockConfirmations":2
}
]`)

	_, err := loadFromEnv()

	s.NotNil(err)
}
