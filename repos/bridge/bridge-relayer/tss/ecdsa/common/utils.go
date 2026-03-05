// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package common

import (
	"math/big"

	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/peer"
	"golang.org/x/exp/slices"
)

func CreatePartyID(peerID string) *tss.PartyID {
	key := big.NewInt(0).SetBytes([]byte(peerID))
	return tss.NewPartyID(peerID, peerID, key)
}

func PeersFromParties(parties []*tss.PartyID) ([]peer.ID, error) {
	peers := make([]peer.ID, len(parties))
	for i, party := range parties {
		peerID, err := peer.Decode(party.Id)
		if err != nil {
			return nil, err
		}

		peers[i] = peerID
	}

	return peers, nil
}

func PartiesFromPeers(peers peer.IDSlice) tss.SortedPartyIDs {
	unsortedParties := make(tss.UnSortedPartyIDs, len(peers))

	for i, peer := range peers {
		unsortedParties[i] = CreatePartyID(peer.String())
	}

	return tss.SortPartyIDs(unsortedParties)
}

func PeersFromIDS(peerIDS []string) ([]peer.ID, error) {
	peers := make([]peer.ID, len(peerIDS))

	for i, id := range peerIDS {
		peerID, err := peer.Decode(id)
		if err != nil {
			return nil, err
		}

		peers[i] = peerID
	}

	return peers, nil
}

func ExcludePeers(peers peer.IDSlice, excludedPeers peer.IDSlice) peer.IDSlice {
	includedPeers := make(peer.IDSlice, 0)
	for _, peer := range peers {
		if slices.Contains(excludedPeers, peer) {
			continue
		}

		includedPeers = append(includedPeers, peer)
	}
	return includedPeers
}

func PeersIntersection(oldPeers peer.IDSlice, newPeers peer.IDSlice) peer.IDSlice {
	includedPeers := make(peer.IDSlice, 0)
	for _, peer := range oldPeers {
		if !slices.Contains(newPeers, peer) {
			continue
		}

		includedPeers = append(includedPeers, peer)
	}
	return includedPeers
}
