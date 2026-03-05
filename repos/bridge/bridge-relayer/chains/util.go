// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package chains

import (
	"fmt"
	"math/big"
)

// CalculateStartingBlock returns first block number (smaller or equal) that is dividable with block confirmations
func CalculateStartingBlock(startBlock *big.Int, blockConfirmations *big.Int) (*big.Int, error) {
	if startBlock == nil || blockConfirmations == nil {
		return nil, fmt.Errorf("startBlock or blockConfirmations can not be nill when calculating CalculateStartingBlock")
	}
	mod := big.NewInt(0).Mod(startBlock, blockConfirmations)
	startBlock.Sub(startBlock, mod)
	return startBlock, nil
}
