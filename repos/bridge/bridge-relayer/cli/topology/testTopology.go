// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology

import (
	"fmt"
	"net/http"

	"github.com/spf13/cobra"

	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/ChainSafe/sygma-relayer/topology"
)

var (
	testTopologyCMD = &cobra.Command{
		Use:   "test",
		Short: "Test topology url",
		Long: "CLI tests does provided url contain topology that could be well " +
			"decrypted with provided password and then parsed accordingly",
		RunE: testTopology,
	}
)

var (
	url           string
	hash          string
	decryptionKey string
)

func init() {
	testTopologyCMD.PersistentFlags().StringVar(&decryptionKey, "decryption-key", "", "password to decrypt topology")
	_ = testTopologyCMD.MarkFlagRequired("decryption-key")
	testTopologyCMD.PersistentFlags().StringVar(&url, "url", "", "url to fetch topology")
	_ = testTopologyCMD.MarkFlagRequired("url")
	testTopologyCMD.PersistentFlags().StringVar(&hash, "hash", "", "hash of topology")

}

func testTopology(cmd *cobra.Command, args []string) error {
	config := relayer.TopologyConfiguration{
		EncryptionKey: decryptionKey,
		Url:           url,
		Path:          "",
	}
	nt, err := topology.NewNetworkTopologyProvider(config, http.DefaultClient)
	if err != nil {
		return err
	}
	decryptedTopology, err := nt.NetworkTopology(hash)
	if err != nil {
		return err
	}

	fmt.Printf("Everything is fine your topology is \n")
	fmt.Printf("%+v", decryptedTopology)
	return nil
}
