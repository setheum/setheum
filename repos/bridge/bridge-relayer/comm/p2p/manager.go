// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p

import (
	"fmt"
	"sync"

	"github.com/libp2p/go-libp2p/core/network"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
)

// StreamManager manages instances of network.Stream
//
// Each stream is mapped to a specific session, by sessionID
type StreamManager struct {
	streamsBySessionID map[string]map[peer.ID]network.Stream
	streamLocker       *sync.Mutex
}

// NewStreamManager creates new StreamManager
func NewStreamManager() *StreamManager {
	return &StreamManager{
		streamsBySessionID: make(map[string]map[peer.ID]network.Stream),
		streamLocker:       &sync.Mutex{},
	}
}

// ReleaseStream removes reference on streams mapped to provided sessionID and closes them
func (sm *StreamManager) ReleaseStreams(sessionID string) {
	sm.streamLocker.Lock()
	defer sm.streamLocker.Unlock()

	streams, ok := sm.streamsBySessionID[sessionID]
	if !ok {
		return
	}

	for peer, stream := range streams {
		err := stream.Close()
		if err != nil {
			log.Err(err).Msgf("Cannot close stream to peer %s", peer.Pretty())
		}
	}

	delete(sm.streamsBySessionID, sessionID)
}

// AddStream saves and maps provided stream to sessionID
func (sm *StreamManager) AddStream(sessionID string, peerID peer.ID, stream network.Stream) {
	sm.streamLocker.Lock()
	defer sm.streamLocker.Unlock()

	_, ok := sm.streamsBySessionID[sessionID]
	if !ok {
		sm.streamsBySessionID[sessionID] = make(map[peer.ID]network.Stream)
	}

	_, ok = sm.streamsBySessionID[sessionID][peerID]
	if ok {
		return
	}

	sm.streamsBySessionID[sessionID][peerID] = stream
}

// Stream fetches stream by peer and session ID
func (sm *StreamManager) Stream(sessionID string, peerID peer.ID) (network.Stream, error) {
	sm.streamLocker.Lock()
	defer sm.streamLocker.Unlock()

	stream, ok := sm.streamsBySessionID[sessionID][peerID]
	if !ok {
		return nil, fmt.Errorf("no stream for peerID %s", peerID)
	}

	return stream, nil
}
