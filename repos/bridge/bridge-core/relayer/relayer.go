// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

package relayer

import (
	"context"

	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type RelayedChain interface {
	// PollEvents starts listening for on-chain events
	PollEvents(ctx context.Context)
	// ReceiveMessage accepts the message from the source chain and converts it into
	// a Proposal to be submitted on-chain
	ReceiveMessage(m *message.Message) (*proposal.Proposal, error)
	// Write submits proposals on-chain.
	// If multiple proposals submitted they are expected to be able to be batched.
	Write(proposals []*proposal.Proposal) error
	DomainID() uint8
}

type MessageTracker interface {
	TrackMessages(msgs []*message.Message, status message.MessageStatus)
}

func NewRelayer(chains map[uint64]RelayedChain, messageTracker MessageTracker) *Relayer {
	return &Relayer{
		relayedChains:  chains,
		messageTracker: messageTracker,
	}
}

type Relayer struct {
	relayedChains  map[uint64]RelayedChain
	messageTracker MessageTracker
}

// Start function starts polling events for each chain and listens to cross-chain messages.
// If an array of messages is sent to the channel they are expected to be to the same destination and
// able to be handled in batches.
func (r *Relayer) Start(ctx context.Context, msgChan chan []*message.Message) {
	log.Info().Msgf("Starting relayer")

	for _, c := range r.relayedChains {
		log.Debug().Msgf("Starting chain %v", c.DomainID())
		go c.PollEvents(ctx)
	}

	for {
		select {
		case m := <-msgChan:
			go r.route(m)
			continue
		case <-ctx.Done():
			return
		}
	}
}

// Route function routes the messages to the destination chain.
func (r *Relayer) route(msgs []*message.Message) {
	r.messageTracker.TrackMessages(msgs, message.PendingMessage)
	destChain, ok := r.relayedChains[msgs[0].Destination]
	if !ok {
		log.Error().Uint64("domainID", msgs[0].Destination).Msgf("No chain registered for destination domain")
		return
	}

	log := log.With().Uint8("domainID", destChain.DomainID()).Str("messageID", msgs[0].ID).Logger()
	props := make([]*proposal.Proposal, 0)
	for _, m := range msgs {
		log.Debug().Msgf("Sending message")

		prop, err := destChain.ReceiveMessage(m)
		if err != nil {
			log.Err(err).Msgf("Failed receiving message %+v", m)
			r.messageTracker.TrackMessages([]*message.Message{m}, message.FailedMessage)
			continue
		}

		log.Debug().Msgf("Received message")

		if prop != nil {
			props = append(props, prop)
		}
	}
	if len(props) == 0 {
		return
	}

	log.Debug().Msgf("Writing message")
	err := destChain.Write(props)
	if err != nil {
		r.messageTracker.TrackMessages(msgs, message.FailedMessage)
		log.Err(err).Msgf("Failed writing message")
		return
	}
	r.messageTracker.TrackMessages(msgs, message.SuccessfulMessage)
}
