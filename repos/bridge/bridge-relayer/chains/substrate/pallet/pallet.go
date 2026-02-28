// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package pallet

import (
	"strconv"

	"github.com/ChainSafe/sygma-relayer/chains"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/client"

	"github.com/centrifuge/go-substrate-rpc-client/v4/rpc/author"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/rs/zerolog/log"
)

const bridgeVersion = "3.1.0"
const verifyingContract = "6CdE2Cd82a4F8B74693Ff5e194c19CA08c2d1c68"

type BridgeProposal struct {
	OriginDomainID uint8
	DepositNonce   uint64
	ResourceID     [32]byte
	Data           []byte
}

type Pallet struct {
	*client.SubstrateClient
}

func NewPallet(
	client *client.SubstrateClient,
) *Pallet {
	return &Pallet{
		client,
	}
}

func (p *Pallet) ExecuteProposals(
	proposals []*transfer.TransferProposal,
	signature []byte,
) (types.Hash, *author.ExtrinsicStatusSubscription, error) {
	bridgeProposals := make([]BridgeProposal, 0)
	for _, prop := range proposals {
		bridgeProposals = append(bridgeProposals, BridgeProposal{
			OriginDomainID: prop.Source,
			DepositNonce:   prop.Data.DepositNonce,
			ResourceID:     prop.Data.ResourceId,
			Data:           prop.Data.Data,
		})
	}

	return p.Transact(
		"SygmaBridge.execute_proposal",
		bridgeProposals,
		signature,
	)
}

func (p *Pallet) ProposalsHash(proposals []*transfer.TransferProposal) ([]byte, error) {
	return chains.ProposalsHash(proposals, p.ChainID.Int64(), verifyingContract, bridgeVersion)
}

func (p *Pallet) IsProposalExecuted(prop *transfer.TransferProposal) (bool, error) {

	log.Debug().
		Str("depositNonce", strconv.FormatUint(prop.Data.DepositNonce, 10)).
		Str("resourceID", hexutil.Encode(prop.Data.ResourceId[:])).
		Msg("Getting is proposal executed")
	var res bool
	err := p.Conn.Call(&res, "sygma_isProposalExecuted", prop.Data.DepositNonce, prop.Source)
	if err != nil {
		return false, err
	}

	return res, nil
}
