// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package centrifuge

import (
	"strings"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/contracts"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
)

type AssetStoreContract struct {
	contracts.Contract
}

func NewAssetStoreContract(
	client client.Client,
	assetStoreContractAddress common.Address,
	transactor transactor.Transactor,
) *AssetStoreContract {
	a, _ := abi.JSON(strings.NewReader(CentrifugeAssetStoreABI))
	b := common.FromHex(CentrifugeAssetStoreBin)
	return &AssetStoreContract{contracts.NewContract(assetStoreContractAddress, a, b, client, transactor)}
}

func (c *AssetStoreContract) IsCentrifugeAssetStored(hash [32]byte) (bool, error) {
	log.Debug().
		Str("hash", hexutil.Encode(hash[:])).
		Msgf("Getting is centrifuge asset stored")
	res, err := c.CallContract("_assetsStored", hash)
	if err != nil {
		return false, err
	}

	isAssetStored := *abi.ConvertType(res[0], new(bool)).(*bool)
	return isAssetStored, nil
}
