// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package listener

import (
	"github.com/ChainSafe/sygma-relayer/chains/substrate/events"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/mitchellh/mapstructure"
)

func DecodeDepositEvent(evtFields registry.DecodedFields) (events.Deposit, error) {
	var d events.Deposit

	for _, evtField := range evtFields {
		switch evtField.Name {
		case "dest_domain_id":
			err := mapstructure.Decode(evtField.Value, &d.DestDomainID)
			if err != nil {
				return events.Deposit{}, err
			}
		case "resource_id":
			err := mapstructure.Decode(evtField.Value, &d.ResourceID)
			if err != nil {
				return events.Deposit{}, err
			}
		case "deposit_nonce":
			err := mapstructure.Decode(evtField.Value, &d.DepositNonce)
			if err != nil {
				return events.Deposit{}, err
			}
		case "sygma_traits_TransferType":
			err := mapstructure.Decode(evtField.Value, &d.TransferType)
			if err != nil {
				return events.Deposit{}, err
			}
		case "deposit_data":
			err := mapstructure.Decode(evtField.Value, &d.CallData)
			if err != nil {
				return events.Deposit{}, err
			}
		case "handler_response":
			err := mapstructure.Decode(evtField.Value, &d.Handler)
			if err != nil {
				return events.Deposit{}, err
			}
		case "block_timestamp":
			err := mapstructure.Decode(evtField.Value, &d.Timestamp)
			if err != nil {
				return events.Deposit{}, err
			}
		}
	}
	return d, nil
}

func DecodeRetryEvent(evtFields registry.DecodedFields) (events.Retry, error) {
	var er events.Retry

	for _, evtField := range evtFields {
		switch evtField.Name {
		case "deposit_on_block_height":
			err := mapstructure.Decode(evtField.Value, &er.DepositOnBlockHeight)
			if err != nil {
				return events.Retry{}, err
			}
		case "dest_domain_id":
			err := mapstructure.Decode(evtField.Value, &er.DestDomainID)
			if err != nil {
				return events.Retry{}, err
			}
		}
	}

	return er, nil
}
