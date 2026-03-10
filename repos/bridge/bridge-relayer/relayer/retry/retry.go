// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package retry

import (
	"math/big"

	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
)

const (
	RetryMessageType message.MessageType = "RetryMessage"
)

type RetryMessageData struct {
	SourceDomainID      uint8
	DestinationDomainID uint8
	BlockHeight         *big.Int
	ResourceID          [32]byte
}

type PropStorer interface {
	StorePropStatus(source, destination uint8, depositNonce uint64, status store.PropStatus) error
	PropStatus(source, destination uint8, depositNonce uint64) (store.PropStatus, error)
}

// FilterDeposits filters deposits per domain and resource
// that are to be retried.
func FilterDeposits(
	propStorer PropStorer,
	domainDeposits map[uint8][]*message.Message,
	resourceID [32]byte,
	destination uint8) ([]*message.Message, error) {
	filteredDeposits := make([]*message.Message, 0)
	for domain, deposits := range domainDeposits {
		if domain != destination {
			continue
		}

		for _, deposit := range deposits {
			data := deposit.Data.(transfer.TransferMessageData)
			if data.ResourceId != resourceID {
				continue
			}

			isExecuted, err := isExecuted(deposit, propStorer)
			if err != nil {
				log.Err(err).Str("messageID", deposit.ID).Msgf("Failed checking if deposit executed %+v", deposit)
				continue
			}
			if isExecuted {
				log.Debug().Str("messageID", deposit.ID).Msgf("Deposit marked as executed %+v", deposit)
				continue
			}

			filteredDeposits = append(filteredDeposits, deposit)
		}
	}
	return filteredDeposits, nil
}

func isExecuted(msg *message.Message, propStorer PropStorer) (bool, error) {
	var err error
	propStatus, err := propStorer.PropStatus(
		msg.Source,
		msg.Destination,
		msg.Data.(transfer.TransferMessageData).DepositNonce)
	if err != nil {
		return true, err
	}

	if propStatus == store.ExecutedProp {
		return true, nil
	}

	// change the status to failed if proposal is stuck to be able to retry it
	if propStatus == store.PendingProp {
		err = propStorer.StorePropStatus(
			msg.Source,
			msg.Destination,
			msg.Data.(transfer.TransferMessageData).DepositNonce,
			store.FailedProp)
	}
	return false, err
}
