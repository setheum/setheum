// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package connection

import (
	"github.com/btcsuite/btcd/rpcclient"
	"github.com/rs/zerolog/log"
)

type Connection struct {
	*rpcclient.Client
}

func NewBtcConnection(url string, username string, password string, disableTLS bool) (*Connection, error) {
	// Connect to a Bitcoin node using RPC
	connConfig := &rpcclient.ConnConfig{
		HTTPPostMode: true,
		Host:         url,
		User:         username,
		Pass:         password,
		DisableTLS:   disableTLS,
	}

	client, err := rpcclient.New(connConfig, nil)
	if err != nil {
		return nil, err
	}

	info, err := client.GetBlockChainInfo()
	if err != nil {
		return nil, err
	}
	log.Debug().Msgf("Connected to bitcoin node %s ", info.Chain)

	return &Connection{
		Client: client,
	}, nil
}
