// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm

import (
	"time"

	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
)

const HealthTimeout = 10 * time.Second

func ExecuteCommHealthCheck(communication Communication, peers peer.IDSlice) []*CommunicationError {
	sessionID := "health-session"
	defer communication.CloseSession(sessionID)
	log.Debug().Msgf("ExecuteCommHealthCheck for peers %s", peers.String())
	errors := make([]*CommunicationError, 0)
	for _, p := range peers {
		err := communication.Broadcast([]peer.ID{p}, []byte{}, Unknown, sessionID)
		if err != nil {
			errors = append(errors, err.(*CommunicationError))
		}
	}
	return errors
}
