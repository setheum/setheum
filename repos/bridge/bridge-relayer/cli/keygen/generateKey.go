// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package keygen

import (
	"encoding/hex"
	"fmt"

	"github.com/spf13/cobra"
	"github.com/sygmaprotocol/sygma-core/crypto/secp256k1"
)

var (
	generateKeyCMD = &cobra.Command{
		Use:   "gen-key",
		Short: "Generate keys for signing transactions",
		Long:  "Generate keys for signing transactions",
		RunE:  generateKey,
	}
)

func generateKey(cmd *cobra.Command, args []string) error {
	kp, err := secp256k1.GenerateKeypair()
	if err != nil {
		return err
	}

	privHex := hex.EncodeToString(kp.Encode())

	fmt.Printf("Public key: %s\n", kp.PublicKey())
	fmt.Printf("Private key: 0x%s\n", privHex)
	return nil
}
