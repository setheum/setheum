// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package comm

import (
	"fmt"

	"github.com/libp2p/go-libp2p/core/peer"
)

type CommunicationError struct {
	Peer peer.ID
	Err  error
}

func (ce *CommunicationError) Error() string {
	return fmt.Sprintf("failed communicating with peer %s because of: %s", ce.Peer.Pretty(), ce.Err.Error())
}
