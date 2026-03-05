// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package cli

import (
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"

	"github.com/ChainSafe/sygma-relayer/cli/keygen"
	"github.com/ChainSafe/sygma-relayer/cli/peer"
	"github.com/ChainSafe/sygma-relayer/cli/topology"
	"github.com/ChainSafe/sygma-relayer/cli/utils"
	"github.com/ChainSafe/sygma-relayer/config"
)

var (
	rootCMD = &cobra.Command{
		Use: "",
	}
)

func init() {
	config.BindFlags(rootCMD)
	rootCMD.PersistentFlags().String("name", "", "relayer name")
	_ = viper.BindPFlag("name", rootCMD.PersistentFlags().Lookup("name"))

	rootCMD.PersistentFlags().String("config-url", "", "URL of shared configuration")
	_ = viper.BindPFlag("config-url", rootCMD.PersistentFlags().Lookup("config-url"))
}

func Execute() {
	rootCMD.AddCommand(runCMD, peer.PeerCLI, topology.TopologyCLI, utils.UtilsCLI, keygen.KeygenCLI)
	if err := rootCMD.Execute(); err != nil {
		log.Fatal().Err(err).Msg("failed to execute root cmd")
	}
}
