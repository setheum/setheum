// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package keygen

import (
	"github.com/spf13/cobra"
)

var KeygenCLI = &cobra.Command{
	Use:   "keygen",
	Short: "Key generation",
}

func init() {
	KeygenCLI.AddCommand(generateKeyCMD)
}
