// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package connection

import (
	"bytes"
	"math/big"
	"sync"
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/client"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/parser"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/retriever"
	"github.com/centrifuge/go-substrate-rpc-client/v4/registry/state"
	"github.com/vedhavyas/go-subkey/scale"

	"github.com/centrifuge/go-substrate-rpc-client/v4/rpc"
	"github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

type Connection struct {
	chain.Chain
	client.Client
	*rpc.RPC
	meta        types.Metadata // Latest chain metadata
	metaLock    sync.RWMutex   // Lock metadata for updates, allows concurrent reads
	GenesisHash types.Hash     // Chain genesis hash
}

func NewSubstrateConnection(url string) (*Connection, error) {
	client, err := client.Connect(url)
	if err != nil {
		return nil, err
	}
	rpc, err := rpc.NewRPC(client)
	if err != nil {
		return nil, err
	}

	meta, err := rpc.State.GetMetadataLatest()
	if err != nil {
		return nil, err
	}
	genesisHash, err := rpc.Chain.GetBlockHash(0)
	if err != nil {
		return nil, err
	}

	return &Connection{
		meta: *meta,

		RPC:         rpc,
		Chain:       rpc.Chain,
		Client:      client,
		GenesisHash: genesisHash,
	}, nil
}

func (c *Connection) GetMetadata() (meta types.Metadata) {
	c.metaLock.RLock()
	meta = c.meta
	c.metaLock.RUnlock()
	return meta
}

func (c *Connection) UpdateMetatdata() error {
	c.metaLock.Lock()
	meta, err := c.RPC.State.GetMetadataLatest()
	if err != nil {
		c.metaLock.Unlock()
		return err
	}
	c.meta = *meta
	c.metaLock.Unlock()
	return nil
}

func (c *Connection) GetBlockEvents(hash types.Hash) ([]*parser.Event, error) {
	provider := state.NewEventProvider(c.State)
	eventRetriever, err := retriever.NewDefaultEventRetriever(provider, c.State)
	if err != nil {
		return nil, err
	}

	evts, err := eventRetriever.GetEvents(hash)
	if err != nil {
		return nil, err
	}

	timestamp, err := c.GetBlockTimestamp(hash)
	if err != nil {
		return nil, err
	}

	for _, e := range evts {
		e.Fields = append(e.Fields, &registry.DecodedField{
			Value: timestamp,
			Name:  "block_timestamp",
		})
	}
	return evts, nil
}

func (c *Connection) GetBlockTimestamp(hash types.Hash) (time.Time, error) {
	callIndex, err := c.meta.FindCallIndex("Timestamp.set")
	if err != nil {
		return time.Now(), err
	}

	block, err := c.GetBlock(hash)
	if err != nil {
		return time.Now(), err
	}

	timestamp := new(big.Int)
	for _, extrinsic := range block.Block.Extrinsics {
		if extrinsic.Method.CallIndex != callIndex {
			continue
		}
		timeDecoder := scale.NewDecoder(bytes.NewReader(extrinsic.Method.Args))
		timestamp, err = timeDecoder.DecodeUintCompact()
		if err != nil {
			return time.Now(), err
		}
		break
	}
	msec := timestamp.Int64()
	return time.Unix(msec/1e3, (msec%1e3)*1e6), nil
}

func (c *Connection) FetchEvents(startBlock, endBlock *big.Int) ([]*parser.Event, error) {
	evts := make([]*parser.Event, 0)
	for i := new(big.Int).Set(startBlock); i.Cmp(endBlock) <= 0; i.Add(i, big.NewInt(1)) {
		hash, err := c.GetBlockHash(i.Uint64())
		if err != nil {
			return nil, err
		}

		evt, err := c.GetBlockEvents(hash)
		if err != nil {
			return nil, err
		}
		evts = append(evts, evt...)
	}
	return evts, nil
}
