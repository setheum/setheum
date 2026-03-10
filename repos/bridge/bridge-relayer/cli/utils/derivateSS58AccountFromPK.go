// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package utils

import (
	"fmt"

	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/spf13/cobra"
)

var (
	derivateSS58AccountFromPKCMD = &cobra.Command{
		Use:   "derivateSS58",
		Short: "will print SS58 formatted address (Polkadot) for given PrivateKey in hex",
		Long:  "Will print SS58 formatted address (Polkadot) for given PrivateKey in hex",
		RunE:  derivateSS58,
	}
)

var (
	privateKey string
	networkID  uint16
)

func init() {
	derivateSS58AccountFromPKCMD.PersistentFlags().StringVar(&privateKey, "privateKey", "", "hex encoded private key")
	_ = derivateSS58AccountFromPKCMD.MarkFlagRequired("privateKey")
	derivateSS58AccountFromPKCMD.PersistentFlags().Uint16Var(&networkID, "networkID", 0, "network id for a checksum. Registry https://github.com/paritytech/ss58-registry/blob/main/ss58-registry.json")
	_ = derivateSS58AccountFromPKCMD.MarkFlagRequired("networkID")
}

func derivateSS58(cmd *cobra.Command, args []string) error {
	account, err := signature.KeyringPairFromSecret(privateKey, networkID)
	if err != nil {
		return err
	}

	fmt.Println(account.Address)
	return nil
}
