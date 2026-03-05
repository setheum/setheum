// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"strings"

	"github.com/ChainSafe/sygma-relayer/config/relayer"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
)

type NetworkTopology struct {
	Peers     []*peer.AddrInfo
	Threshold int
}

func (nt NetworkTopology) IsAllowedPeer(peer peer.ID) bool {
	for _, p := range nt.Peers {
		if p.ID == peer {
			return true
		}
	}

	return false
}

type RawTopology struct {
	Peers     []RawPeer `mapstructure:"Peers" json:"peers"`
	Threshold string    `mapstructure:"Threshold" json:"threshold"`
}

type RawPeer struct {
	PeerAddress string `mapstructure:"PeerAddress" json:"peerAddress"`
}
type Fetcher interface {
	Get(url string) (*http.Response, error)
}

type Decrypter interface {
	Decrypt(data []byte) []byte
}

type NetworkTopologyProvider interface {
	// NetworkTopology fetches latest topology from network and validates that
	// the version matches expected hash.
	NetworkTopology(hash string) (*NetworkTopology, error)
}

func NewNetworkTopologyProvider(config relayer.TopologyConfiguration, fetcher Fetcher) (NetworkTopologyProvider, error) {
	decrypter, err := NewAESEncryption([]byte(config.EncryptionKey))
	if err != nil {
		return nil, err
	}

	return &TopologyProvider{
		decrypter: decrypter,
		url:       config.Url,
		fetcher:   fetcher,
	}, nil
}

type TopologyProvider struct {
	url       string
	decrypter Decrypter
	fetcher   Fetcher
}

func (t *TopologyProvider) NetworkTopology(hash string) (*NetworkTopology, error) {
	log.Info().Msgf("Reading topology from URL: %s", t.url)

	resp, err := t.fetcher.Get(t.url)
	if err != nil {
		return nil, err
	}
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	response := strings.TrimSuffix(string(body), "\n")
	ct, err := hex.DecodeString(response)
	if err != nil {
		return nil, err
	}
	h := sha256.New()
	h.Write(ct)
	eh := hex.EncodeToString(h.Sum(nil))
	if hash != "" && eh != hash {
		return nil, fmt.Errorf("topology hash %s not matching expected hash %s", string(eh), hash)
	}

	unecryptedBody := t.decrypter.Decrypt(ct)
	rawTopology := &RawTopology{}
	err = json.Unmarshal(unecryptedBody, rawTopology)
	if err != nil {
		return nil, err
	}

	return ProcessRawTopology(rawTopology)
}

func ProcessRawTopology(rawTopology *RawTopology) (*NetworkTopology, error) {
	var peers []*peer.AddrInfo
	for _, p := range rawTopology.Peers {
		addrInfo, err := peer.AddrInfoFromString(p.PeerAddress)
		if err != nil {
			return nil, fmt.Errorf("invalid peer address %s: %w", p.PeerAddress, err)
		}
		peers = append(peers, addrInfo)
	}

	threshold, err := strconv.ParseInt(rawTopology.Threshold, 0, 0)
	if err != nil {
		return nil, fmt.Errorf("unable to parse mpc threshold from topology %v", err)
	}
	if threshold < 1 {
		return nil, fmt.Errorf("mpc threshold must be bigger then 0 %v", err)
	}
	return &NetworkTopology{Peers: peers, Threshold: int(threshold)}, nil
}
