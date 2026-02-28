// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package events

import (
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

type Deposit struct {
	DestDomainID types.U8      `mapstructure:"dest_domain_id"`
	ResourceID   types.Bytes32 `mapstructure:"resource_id"`
	DepositNonce types.U64     `mapstructure:"deposit_nonce"`
	TransferType types.U8      `mapstructure:"sygma_traits_TransferType"`
	CallData     []byte        `mapstructure:"deposit_data"`
	Handler      [1]byte       `mapstructure:"handler_response"`
	Timestamp    time.Time     `mapstructure:"block_timestamp"`
}

type Retry struct {
	DepositOnBlockHeight types.U128 `mapstructure:"deposit_on_block_height"`
	DestDomainID         types.U8   `mapstructure:"dest_domain_id"`
}

const (
	ParachainUpdatedEvent       = "ParachainSystem.ValidationFunctionApplied"
	ExtrinsicFailedEvent        = "System.ExtrinsicFailed"
	ExtrinsicSuccessEvent       = "System.ExtrinsicSuccess"
	RetryEvent                  = "SygmaBridge.Retry"
	DepositEvent                = "SygmaBridge.Deposit"
	FailedHandlerExecutionEvent = "SygmaBridge.FailedHandlerExecution"
)
