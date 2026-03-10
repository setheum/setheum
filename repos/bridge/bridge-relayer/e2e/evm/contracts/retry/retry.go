// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package retry

import (
	"math/big"
	"strings"

	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/consts"
	"github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/contracts"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
)

type RetryContract struct {
	contracts.Contract
}

func NewRetryContract(
	client client.Client,
	address common.Address,
	transactor transactor.Transactor,
) *RetryContract {
	a, _ := abi.JSON(strings.NewReader(consts.RetryABI))
	return &RetryContract{contracts.NewContract(address, a, nil, client, transactor)}
}

func (c *RetryContract) Retry(
	source uint8,
	destination uint8,
	block *big.Int,
	resourceID [32]byte,
	opts transactor.TransactOptions) (*common.Hash, error) {
	return c.ExecuteTransaction("retry", opts, source, destination, block, resourceID)
}
