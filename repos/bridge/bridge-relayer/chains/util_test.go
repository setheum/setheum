// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package chains

import (
	"math/big"
	"testing"

	"github.com/stretchr/testify/suite"
)

type UtilTestSuite struct {
	suite.Suite
}

func TestRunNewEVMConfigTestSuite(t *testing.T) {
	suite.Run(t, new(UtilTestSuite))
}

func (s *UtilTestSuite) Test_CalculateStartingBlock_ProperAdjustment() {
	res, err := CalculateStartingBlock(big.NewInt(104), big.NewInt(5))
	s.Equal(big.NewInt(100), res)
	s.Nil(err)
}

func (s *UtilTestSuite) Test_CalculateStartingBlock_NoAdjustment() {
	res, err := CalculateStartingBlock(big.NewInt(200), big.NewInt(5))
	s.Equal(big.NewInt(200), res)
	s.Nil(err)
}

func (s *UtilTestSuite) Test_CalculateStartingBlock_Nil() {
	res, err := CalculateStartingBlock(nil, nil)
	s.Nil(res)
	s.NotNil(err)
}
