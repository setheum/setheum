// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package elector

import (
	"context"
	"sync"
	"time"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/ChainSafe/sygma-relayer/tss/util"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
)

// bullyCoordinatorElector is used to execute bully coordinator discovery
type bullyCoordinatorElector struct {
	sessionID    string
	receiveChan  chan *comm.WrappedMessage
	electionChan chan *comm.WrappedMessage
	msgChan      chan *comm.WrappedMessage
	pingChan     chan *comm.WrappedMessage
	comm         comm.Communication
	hostID       peer.ID
	conf         relayer.BullyConfig
	mu           *sync.RWMutex
	coordinator  peer.ID
	sortedPeers  util.SortablePeerSlice
}

func NewBullyCoordinatorElector(
	sessionID string, host host.Host, config relayer.BullyConfig, communication comm.Communication,
) CoordinatorElector {
	bully := &bullyCoordinatorElector{
		sessionID:    sessionID,
		receiveChan:  make(chan *comm.WrappedMessage),
		electionChan: make(chan *comm.WrappedMessage, 1),
		msgChan:      make(chan *comm.WrappedMessage),
		pingChan:     make(chan *comm.WrappedMessage),
		comm:         communication,
		conf:         config,
		hostID:       host.ID(),
		mu:           &sync.RWMutex{},
		coordinator:  host.ID(),
	}

	return bully
}

// Coordinator starts coordinator discovery using bully algorithm and returns current leader
// Bully coordination is executed on provided peers
func (bc *bullyCoordinatorElector) Coordinator(ctx context.Context, peers peer.IDSlice) (peer.ID, error) {
	log.Info().Str("SessionID", bc.sessionID).Msgf("Starting bully process")

	ctx, cancel := context.WithCancel(ctx)
	go bc.listen(ctx)
	defer cancel()

	bc.sortedPeers = util.SortPeersForSession(peers, bc.sessionID)
	errChan := make(chan error)
	go bc.startBullyCoordination(errChan)

	select {
	case err := <-errChan:
		return "", err
	case <-time.After(bc.conf.BullyWaitTime):
		break
	}

	return bc.getCoordinator(), nil
}

// listen starts listening for coordinator relevant messages
func (bc *bullyCoordinatorElector) listen(ctx context.Context) {
	bc.comm.Subscribe(bc.sessionID, comm.CoordinatorPingMsg, bc.msgChan)
	bc.comm.Subscribe(bc.sessionID, comm.CoordinatorElectionMsg, bc.msgChan)
	bc.comm.Subscribe(bc.sessionID, comm.CoordinatorAliveMsg, bc.msgChan)
	bc.comm.Subscribe(bc.sessionID, comm.CoordinatorPingResponseMsg, bc.msgChan)
	bc.comm.Subscribe(bc.sessionID, comm.CoordinatorSelectMsg, bc.msgChan)
	bc.comm.Subscribe(bc.sessionID, comm.CoordinatorLeaveMsg, bc.msgChan)

	for {
		select {
		case <-ctx.Done():
			return
		case msg := <-bc.msgChan:
			switch msg.MessageType {
			case comm.CoordinatorAliveMsg:
				// check if peer that sent alive msg has higher order
				if bc.isPeerIDHigher(bc.hostID, msg.From) {
					select {
					// waits for confirmation that elector is alive
					case bc.electionChan <- msg:
						break
					case <-time.After(500 * time.Millisecond):
						break
					}
				}
			case comm.CoordinatorSelectMsg:
				bc.receiveChan <- msg
			case comm.CoordinatorElectionMsg:
				bc.receiveChan <- msg
			case comm.CoordinatorPingResponseMsg:
				bc.pingChan <- msg
			case comm.CoordinatorPingMsg:
				_ = bc.comm.Broadcast(
					[]peer.ID{msg.From}, nil, comm.CoordinatorPingResponseMsg, bc.sessionID,
				)
			default:
				break
			}
		}
	}
}

func (bc *bullyCoordinatorElector) elect(errChan chan error) {
	for _, p := range bc.sortedPeers {
		if bc.isPeerIDHigher(p.ID, bc.hostID) {
			_ = bc.comm.Broadcast(peer.IDSlice{p.ID}, nil, comm.CoordinatorElectionMsg, bc.sessionID)
		}
	}

	select {
	case <-bc.electionChan:
		return
	case <-time.After(bc.conf.ElectionWaitTime):
		bc.setCoordinator(bc.hostID)
		_ = bc.comm.Broadcast(bc.sortedPeers.GetPeerIDs(), []byte{}, comm.CoordinatorSelectMsg, bc.sessionID)
		return
	}
}

func (bc *bullyCoordinatorElector) startBullyCoordination(errChan chan error) {
	bc.elect(errChan)
	for msg := range bc.receiveChan {
		if msg.MessageType == comm.CoordinatorElectionMsg && !bc.isPeerIDHigher(msg.From, bc.hostID) {
			_ = bc.comm.Broadcast([]peer.ID{msg.From}, []byte{}, comm.CoordinatorAliveMsg, bc.sessionID)
			bc.elect(errChan)
		} else if msg.MessageType == comm.CoordinatorSelectMsg {
			bc.setCoordinator(msg.From)
		}
	}
}

func (bc *bullyCoordinatorElector) isPeerIDHigher(p1 peer.ID, p2 peer.ID) bool {
	var i1, i2 int
	for i := range bc.sortedPeers {
		if p1 == bc.sortedPeers[i].ID {
			i1 = i
		}
		if p2 == bc.sortedPeers[i].ID {
			i2 = i
		}
	}
	return i1 < i2
}

func (bc *bullyCoordinatorElector) setCoordinator(ID peer.ID) {
	bc.mu.Lock()
	defer bc.mu.Unlock()

	if bc.isPeerIDHigher(ID, bc.coordinator) || ID == bc.hostID {
		bc.coordinator = ID
	}
}

func (bc *bullyCoordinatorElector) getCoordinator() peer.ID {
	return bc.coordinator
}
