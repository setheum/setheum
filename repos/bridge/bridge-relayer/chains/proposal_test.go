// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package chains

import (
	"testing"

	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/stretchr/testify/suite"
)

const bridgeVersion = "3.1.0"
const verifyingContract = "6CdE2Cd82a4F8B74693Ff5e194c19CA08c2d1c68"

type ProposalTestSuite struct {
	suite.Suite
}

func TestRunProposalTestSuite(t *testing.T) {
	suite.Run(t, new(ProposalTestSuite))
}

func (s *ProposalTestSuite) Test_ProposalsHash() {
	data := []byte{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 243, 16, 122, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 1, 1, 0, 212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125}

	prop := []*transfer.TransferProposal{{
		Source:      1,
		Destination: 2,
		Data: transfer.TransferProposalData{
			DepositNonce: 15078986465725403975,
			ResourceId:   [32]byte{3},
			Metadata:     nil,
			Data:         data,
		},
	}}
	correctRes := []byte{0xde, 0x7b, 0x5c, 0x9e, 0x8, 0x7a, 0xb4, 0xf5, 0xfb, 0xe, 0x9f, 0x73, 0xa7, 0xe5, 0xbd, 0xb, 0xdf, 0x9e, 0xeb, 0x4, 0xaa, 0xbb, 0xd0, 0xe8, 0xf8, 0xde, 0x58, 0xa2, 0x4, 0xa3, 0x3e, 0x55}

	res, err := ProposalsHash(prop, 5, verifyingContract, bridgeVersion)
	s.Nil(err)
	s.Equal(correctRes, res)
}
