// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package peer

import (
	"fmt"

	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/spf13/cobra"
)

var (
	peerInfoCMD = &cobra.Command{
		Use:   "info",
		Short: "Calculate peer ID from private key",
		Long:  "Calculate peer ID from private key",
		RunE:  peerInfo,
	}
)

var (
	privateKey string
)

func init() {
	peerInfoCMD.PersistentFlags().StringVar(&privateKey, "private-key", "", "Base64 encoded libp2p private key")
	_ = peerInfoCMD.MarkFlagRequired("private-key")
}

func peerInfo(cmd *cobra.Command, args []string) error {
	privBytes, err := crypto.ConfigDecodeKey(privateKey)
	if err != nil {
		return err
	}

	priv, err := crypto.UnmarshalPrivateKey(privBytes)
	if err != nil {
		return err
	}

	peerID, err := peer.IDFromPrivateKey(priv)
	if err != nil {
		return err
	}

	fmt.Printf(`
LibP2P peer identity: %s
`,
		peerID.Pretty(),
	)
	return nil
}
