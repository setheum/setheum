// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package topology

import (
	"encoding/json"
	"os"
	"sync"
)

type TopologyStore struct {
	mu   sync.Mutex
	path string
}

func NewTopologyStore(filePath string) *TopologyStore {
	return &TopologyStore{
		path: filePath,
	}
}

// StoreTopology stores topology into a file
func (ts *TopologyStore) StoreTopology(topology *NetworkTopology) error {
	ts.mu.Lock()
	defer ts.mu.Unlock()

	f, err := os.OpenFile(ts.path, os.O_RDWR|os.O_CREATE|os.O_TRUNC, 0755)
	if err != nil {

		return err
	}
	defer f.Close()

	kb, err := json.Marshal(&topology)
	if err != nil {
		return err
	}

	_, err = f.Write(kb)
	return err
}

// Topology fetches current topology from file
func (ts *TopologyStore) Topology() (*NetworkTopology, error) {
	ts.mu.Lock()
	defer ts.mu.Unlock()
	t := &NetworkTopology{}
	tb, err := os.ReadFile(ts.path)
	if err != nil {
		return nil, err
	}

	err = json.Unmarshal(tb, &t)
	if err != nil {
		return nil, err
	}
	return t, err
}
