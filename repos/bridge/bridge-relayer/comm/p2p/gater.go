// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package p2p

import (
	"github.com/libp2p/go-libp2p/core/control"
	"github.com/libp2p/go-libp2p/core/network"
	"github.com/libp2p/go-libp2p/core/peer"
	ma "github.com/multiformats/go-multiaddr"

	"github.com/ChainSafe/sygma-relayer/topology"
)

// ConnectionGate implements libp2p ConnectionGater to prevent inbound and
// outbound requests to peers not specified in topology
type ConnectionGate struct {
	topology *topology.NetworkTopology
}

func NewConnectionGate(topology *topology.NetworkTopology) *ConnectionGate {
	return &ConnectionGate{
		topology: topology,
	}
}

func (cg *ConnectionGate) SetTopology(topology *topology.NetworkTopology) {
	cg.topology = topology
}

func (cg *ConnectionGate) InterceptPeerDial(p peer.ID) (allow bool) {
	return cg.topology.IsAllowedPeer(p)
}

func (cg *ConnectionGate) InterceptSecured(nd network.Direction, p peer.ID, cm network.ConnMultiaddrs) (allow bool) {
	return cg.topology.IsAllowedPeer(p)
}

func (cg *ConnectionGate) InterceptAddrDial(peer.ID, ma.Multiaddr) (allow bool) {
	return true
}

func (cg *ConnectionGate) InterceptAccept(network.ConnMultiaddrs) (allow bool) {
	return true
}

func (cg *ConnectionGate) InterceptUpgraded(network.Conn) (allow bool, reason control.DisconnectReason) {
	return true, 0
}
