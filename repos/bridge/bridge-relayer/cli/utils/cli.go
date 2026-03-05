// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package utils

import "github.com/spf13/cobra"

var UtilsCLI = &cobra.Command{
	Use:   "utils",
	Short: "set of utility CLI commands",
}

func init() {
	UtilsCLI.AddCommand(derivateSS58AccountFromPKCMD)
}
