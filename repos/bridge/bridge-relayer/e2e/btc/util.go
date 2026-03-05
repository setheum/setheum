package btc

import (
	"errors"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/btc/connection"
	"github.com/btcsuite/btcd/btcutil"
)

const (
	BtcEndpoint = "localhost:18443"
)

var TestTimeout = time.Minute * 3

func WaitForProposalExecuted(conn *connection.Connection, addr btcutil.Address) error {
	timeout := time.After(TestTimeout)
	ticker := time.NewTicker(5 * time.Second)

	for {
		select {
		case <-ticker.C:
			balance, err := conn.ListUnspentMinMaxAddresses(0, 1000, []btcutil.Address{addr})
			if err != nil {
				ticker.Stop()
				return err
			}
			if len(balance) > 0 {
				ticker.Stop()
				return nil
			}
		case <-timeout:
			ticker.Stop()
			return errors.New("test timed out waiting for proposal execution event")
		}
	}
}
