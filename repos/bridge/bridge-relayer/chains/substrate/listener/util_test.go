// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener_test

import (
	"math/big"
	"testing"

	"github.com/ChainSafe/sygma-relayer/chains/substrate/events"
	"github.com/ChainSafe/sygma-relayer/chains/substrate/listener"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/stretchr/testify/suite"
)

type DecodeEventsSuite struct {
	suite.Suite
}

func TestRunDecodeEventsSuite(t *testing.T) {
	suite.Run(t, new(DecodeEventsSuite))
}

func (s *DecodeEventsSuite) TestDecodeDepositEvent() {
	evtFields := registry.DecodedFields{
		&registry.DecodedField{Name: "dest_domain_id", Value: 1, LookupIndex: 0},
		&registry.DecodedField{Name: "resource_id", Value: types.Bytes32{3}, LookupIndex: 0},
		&registry.DecodedField{Name: "deposit_nonce", Value: 0, LookupIndex: 0},
		&registry.DecodedField{Name: "sygma_traits_TransferType", Value: 0, LookupIndex: 0},
		&registry.DecodedField{Name: "deposit_data", Value: []byte{}, LookupIndex: 0},
		&registry.DecodedField{Name: "handler_response", Value: [1]byte{0}, LookupIndex: 0},
	}

	deposit, err := listener.DecodeDepositEvent(evtFields)
	s.Nil(err)
	s.Equal(deposit, events.Deposit{DestDomainID: 1, ResourceID: types.Bytes32{3}, DepositNonce: 0, TransferType: 0, CallData: []byte{}, Handler: [1]byte{}})
}

func (s *DecodeEventsSuite) TestDecodeRetryEvent() {
	evtFields := registry.DecodedFields{
		&registry.DecodedField{Name: "deposit_on_block_height", Value: types.NewU128(*big.NewInt(1)), LookupIndex: 0},
		&registry.DecodedField{Name: "dest_domain_id", Value: 2, LookupIndex: 0},
	}

	retry, err := listener.DecodeRetryEvent(evtFields)
	s.Nil(err)
	s.Equal(retry, events.Retry{DepositOnBlockHeight: types.NewU128(*big.NewInt(1)), DestDomainID: 2})
}
