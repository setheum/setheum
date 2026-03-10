// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package client

import (
	"bytes"
	"context"
	"fmt"
	"math/big"
	"sync"
	"time"

	"github.com/centrifuge/go-substrate-rpc-client/v4/rpc/author"
	"github.com/centrifuge/go-substrate-rpc-client/v4/scale"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	"github.com/rs/zerolog/log"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/connection"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/events"
)

type SubstrateClient struct {
	key       *signature.KeyringPair // Keyring used for signing
	nonceLock sync.Mutex             // Locks nonce for updates
	nonce     types.U32              // Latest account nonce
	tip       uint64
	Conn      *connection.Connection
	ChainID   *big.Int
}

func NewSubstrateClient(conn *connection.Connection, key *signature.KeyringPair, chainID *big.Int, tip uint64) *SubstrateClient {
	return &SubstrateClient{
		key:     key,
		Conn:    conn,
		ChainID: chainID,
		tip:     tip,
	}
}

// Transact constructs and submits an extrinsic to call the method with the given arguments.
// All args are passed directly into GSRPC. GSRPC types are recommended to avoid serialization inconsistencies.
func (c *SubstrateClient) Transact(method string, args ...interface{}) (types.Hash, *author.ExtrinsicStatusSubscription, error) {
	log.Debug().Msgf("Submitting substrate call... method %s, sender %s", method, c.key.Address)

	// Create call and extrinsic
	meta := c.Conn.GetMetadata()
	call, err := types.NewCall(
		&meta,
		method,
		args...,
	)
	if err != nil {
		return types.Hash{}, nil, fmt.Errorf("failed to construct call: %w", err)
	}

	ext := types.NewExtrinsic(call)
	// Get latest runtime version
	rv, err := c.Conn.RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return types.Hash{}, nil, err
	}

	c.nonceLock.Lock()
	defer c.nonceLock.Unlock()

	nonce, err := c.nextNonce(&meta)
	if err != nil {
		return types.Hash{}, nil, err
	}

	// Sign the extrinsic
	o := types.SignatureOptions{
		BlockHash:          c.Conn.GenesisHash,
		Era:                types.ExtrinsicEra{IsMortalEra: false},
		GenesisHash:        c.Conn.GenesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(c.tip),
		TransactionVersion: rv.TransactionVersion,
	}
	sub, err := c.submitAndWatchExtrinsic(o, &ext)
	if err != nil {
		return types.Hash{}, nil, fmt.Errorf("submission of extrinsic failed: %w", err)
	}

	hash, err := ExtrinsicHash(ext)
	if err != nil {
		return types.Hash{}, nil, err
	}

	log.Info().Str("extrinsic", hash.Hex()).Msgf("Extrinsic call submitted... method %s, sender %s, nonce %d", method, c.key.Address, nonce)
	c.nonce = nonce + 1

	return hash, sub, nil
}

func (c *SubstrateClient) TrackExtrinsic(extHash types.Hash, sub *author.ExtrinsicStatusSubscription) error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(time.Minute*10))
	defer sub.Unsubscribe()
	defer cancel()
	subChan := sub.Chan()
	for {
		select {
		case status := <-subChan:
			{
				if status.IsInBlock {
					log.Debug().Str("extrinsic", extHash.Hex()).Msgf("Extrinsic in block with hash: %#x", status.AsInBlock)
				}
				if status.IsFinalized {
					log.Info().Str("extrinsic", extHash.Hex()).Msgf("Extrinsic is finalized in block with hash: %#x", status.AsFinalized)
					return c.checkExtrinsicSuccess(extHash, status.AsFinalized)
				}
			}
		case <-ctx.Done():
			return fmt.Errorf("extrinsic has timed out")
		}
	}
}

func (c *SubstrateClient) nextNonce(meta *types.Metadata) (types.U32, error) {
	key, err := types.CreateStorageKey(meta, "System", "Account", c.key.PublicKey, nil)
	if err != nil {
		return 0, err
	}

	var latestNonce types.U32
	var acct types.AccountInfo
	exists, err := c.Conn.RPC.State.GetStorageLatest(key, &acct)
	if err != nil {
		return 0, err
	}

	if !exists {
		latestNonce = 0
	} else {
		latestNonce = acct.Nonce
	}

	if latestNonce < c.nonce {
		return c.nonce, nil
	}

	return latestNonce, nil
}

func (c *SubstrateClient) submitAndWatchExtrinsic(opts types.SignatureOptions, ext *types.Extrinsic) (*author.ExtrinsicStatusSubscription, error) {
	err := ext.Sign(*c.key, opts)
	if err != nil {
		return nil, err
	}

	sub, err := c.Conn.RPC.Author.SubmitAndWatchExtrinsic(*ext)
	if err != nil {
		return nil, err
	}

	return sub, nil
}

func (c *SubstrateClient) checkExtrinsicSuccess(extHash types.Hash, blockHash types.Hash) error {
	block, err := c.Conn.Chain.GetBlock(blockHash)
	if err != nil {
		return err
	}

	evts, err := c.Conn.GetBlockEvents(blockHash)
	if err != nil {
		return err
	}

	for _, event := range evts {
		index := event.Phase.AsApplyExtrinsic
		hash, err := ExtrinsicHash(block.Block.Extrinsics[index])
		if err != nil {
			return err
		}

		if extHash != hash {
			continue
		}

		if event.Name == events.ExtrinsicFailedEvent {
			return fmt.Errorf("extrinsic failed")
		}
		if event.Name == events.FailedHandlerExecutionEvent {
			return fmt.Errorf("extrinsic failed with failed handler execution")
		}
		if event.Name == events.ExtrinsicSuccessEvent {
			return nil
		}
	}

	return fmt.Errorf("no event found")
}

func (c *SubstrateClient) LatestBlock() (*big.Int, error) {
	block, err := c.Conn.Chain.GetBlockLatest()
	if err != nil {
		return nil, err
	}
	return big.NewInt(int64(block.Block.Header.Number)), nil
}

func ExtrinsicHash(ext types.Extrinsic) (types.Hash, error) {
	extHash := bytes.NewBuffer([]byte{})
	encoder := scale.NewEncoder(extHash)
	err := ext.Encode(*encoder)
	if err != nil {
		return types.Hash{}, err
	}
	return types.NewHash(extHash.Bytes()), nil
}
