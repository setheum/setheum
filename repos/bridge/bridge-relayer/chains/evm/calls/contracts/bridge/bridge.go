// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package bridge

import (
	"context"
	"math/big"
	"strconv"
	"strings"

	"github.com/ChainSafe/sygma-relayer/chains"
	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/consts"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/rs/zerolog/log"

	"github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/contracts"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"
)

const bridgeVersion = "3.1.0"

type BridgeProposal struct {
	OriginDomainID uint8
	ResourceID     [32]byte
	DepositNonce   uint64
	Data           []byte
}

type ChainClient interface {
	client.Client
	ChainID(ctx context.Context) (*big.Int, error)
}

type BridgeContract struct {
	contracts.Contract
	client ChainClient
}

func NewBridgeContract(
	client ChainClient,
	bridgeContractAddress common.Address,
	transactor transactor.Transactor,
) *BridgeContract {
	a, _ := abi.JSON(strings.NewReader(consts.BridgeABI))
	return &BridgeContract{
		Contract: contracts.NewContract(bridgeContractAddress, a, nil, client, transactor),
		client:   client,
	}
}

func (c *BridgeContract) ExecuteProposal(
	proposal *transfer.TransferProposal,
	signature []byte,
	opts transactor.TransactOptions,
) (*common.Hash, error) {
	log.Debug().
		Str("depositNonce", strconv.FormatUint(proposal.Data.DepositNonce, 10)).
		Str("resourceID", hexutil.Encode(proposal.Data.ResourceId[:])).
		Msgf("Execute proposal")
	return c.ExecuteTransaction(
		"executeProposal",
		opts,
		proposal.Source, proposal.Data.DepositNonce, proposal.Data, proposal.Data.ResourceId, signature,
	)
}

func (c *BridgeContract) ExecuteProposals(
	proposals []*transfer.TransferProposal,
	signature []byte,
	opts transactor.TransactOptions,
) (*common.Hash, error) {
	bridgeProposals := make([]BridgeProposal, 0)
	for _, prop := range proposals {

		bridgeProposals = append(bridgeProposals, BridgeProposal{
			OriginDomainID: prop.Source,
			ResourceID:     prop.Data.ResourceId,
			DepositNonce:   prop.Data.DepositNonce,
			Data:           prop.Data.Data,
		})
	}
	return c.ExecuteTransaction(
		"executeProposals",
		opts,
		bridgeProposals,
		signature,
	)
}

func (c *BridgeContract) ProposalsHash(proposals []*transfer.TransferProposal) ([]byte, error) {
	chainID, err := c.client.ChainID(context.Background())
	if err != nil {
		return []byte{}, err
	}
	return chains.ProposalsHash(proposals, chainID.Int64(), c.ContractAddress().Hex(), bridgeVersion)
}

func (c *BridgeContract) IsProposalExecuted(p *transfer.TransferProposal) (bool, error) {

	log.Debug().
		Str("depositNonce", strconv.FormatUint(p.Data.DepositNonce, 10)).
		Str("resourceID", hexutil.Encode(p.Data.ResourceId[:])).
		Msg("Getting is proposal executed")
	res, err := c.CallContract("isProposalExecuted", p.Source, new(big.Int).SetUint64(p.Data.DepositNonce))
	if err != nil {
		return false, err
	}
	out := *abi.ConvertType(res[0], new(bool)).(*bool)
	return out, nil
}

func (c *BridgeContract) GetHandlerAddressForResourceID(
	resourceID [32]byte,
) (common.Address, error) {
	log.Debug().Msgf("Getting handler address for resource %s", hexutil.Encode(resourceID[:]))
	res, err := c.CallContract("_resourceIDToHandlerAddress", resourceID)
	if err != nil {
		return common.Address{}, err
	}
	out := *abi.ConvertType(res[0], new(common.Address)).(*common.Address)
	return out, nil
}

func (c *BridgeContract) Retry(hash common.Hash, opts transactor.TransactOptions) (*common.Hash, error) {
	log.Debug().Msgf("Retrying deposit from transaction: %s", hash.Hex())
	return c.ExecuteTransaction("retry", opts, hash.Hex())
}
