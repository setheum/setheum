// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/ChainSafe/sygma-relayer/topology"

	"github.com/spf13/cobra"
)

var (
	encryptTopologyCMD = &cobra.Command{
		Use:   "encrypt",
		Short: "encrypt provided topology with AES",
		Long:  "Algorithm used is AES CTR. IV and CT returned are in hex.",
		RunE:  encryptTopology,
	}
)

var (
	path          string
	encryptionKey string
)

func init() {
	encryptTopologyCMD.PersistentFlags().StringVar(&path, "path", "", "path to json file with network topology")
	_ = encryptTopologyCMD.MarkFlagRequired("path")
	encryptTopologyCMD.PersistentFlags().StringVar(&encryptionKey, "encryption-key", "", "password to encrypt topology")
	_ = encryptTopologyCMD.MarkFlagRequired("encryption-key")
}

func encryptTopology(cmd *cobra.Command, args []string) error {
	cipherKey := []byte(encryptionKey)
	aesEncryption, _ := topology.NewAESEncryption(cipherKey)
	topologyFile, err := os.Open(path)
	defer func() {
		err := topologyFile.Close()
		if err != nil {
			panic(err)
		}
	}()
	if err != nil {
		return err
	}
	byteValue, err := io.ReadAll(topologyFile)
	if err != nil {
		return err
	}
	// Testing that topology was well-formed
	testTopology := topology.RawTopology{}
	err = json.Unmarshal(byteValue, &testTopology)
	if err != nil {
		return fmt.Errorf("topology was wrong formed %s", err.Error())
	}
	ct, err := aesEncryption.Encrypt(byteValue)
	if err != nil {
		return err
	}

	fmt.Printf("Encrypted topology is: %x \n", ct)
	h := sha256.New()
	h.Write(ct)
	eh := hex.EncodeToString(h.Sum(nil))
	fmt.Printf("Hash of the topology %s", eh)
	return nil
}
