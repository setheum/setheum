// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package erc1155

import (
	"math/big"
	"strings"

	"github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/contracts"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"
)

type ERC1155Contract struct {
	contracts.Contract
}

func NewErc1155Contract(
	client client.Client,
	erc1155ContractAddress common.Address,
	t transactor.Transactor,
) *ERC1155Contract {
	a, _ := abi.JSON(strings.NewReader(ERC1155PresetMinterPauserABI))
	b := common.FromHex(ERC1155PresetMinterPauserABI)
	return &ERC1155Contract{contracts.NewContract(erc1155ContractAddress, a, b, client, t)}
}

func (c *ERC1155Contract) Approve(
	tokenId *big.Int, recipient common.Address, opts transactor.TransactOptions,
) (*common.Hash, error) {
	log.Debug().Msgf("Approving %s token for %s", tokenId.String(), recipient.String())
	return c.ExecuteTransaction("setApprovalForAll", opts, recipient, true)
}

func (c *ERC1155Contract) Mint(
	tokenId *big.Int, amount *big.Int, metadata []byte, destination common.Address, opts transactor.TransactOptions,
) (*common.Hash, error) {
	log.Debug().Msgf("Minting tokens %s to %s", tokenId.String(), destination.String())
	return c.ExecuteTransaction("mint", opts, destination, tokenId, amount, metadata)
}

func (c *ERC1155Contract) BalanceOf(account common.Address, id *big.Int) (*big.Int, error) {
	res, err := c.CallContract("balanceOf", account, id)
	if err != nil {
		return nil, err
	}

	amount := abi.ConvertType(res[0], new(big.Int)).(*big.Int)
	return amount, nil
}
