// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package signing_test

import (
	"context"
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/elector"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/keygen"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/signing"
	tsstest "github.com/ChainSafe/sygma-relayer/tss/test"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/sourcegraph/conc/pool"
	"github.com/stretchr/testify/suite"
)

type SigningTestSuite struct {
	tsstest.CoordinatorTestSuite
}

func TestRunSigningTestSuite(t *testing.T) {
	suite.Run(t, new(SigningTestSuite))
}

func (s *SigningTestSuite) Test_ValidSigningProcess() {
	communicationMap := make(map[peer.ID]*tsstest.TestCommunication)
	coordinators := []*tss.Coordinator{}
	processes := []tss.TssProcess{}

	for i, host := range s.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		fetcher := keyshare.NewECDSAKeyshareStore(fmt.Sprintf("../../test/keyshares/%d.keyshare", i))

		msgBytes := []byte("Message")
		msg := big.NewInt(0)
		msg.SetBytes(msgBytes)
		signing, err := signing.NewSigning(msg, "signing1", "signing1", host, &communication, fetcher)
		if err != nil {
			panic(err)
		}
		electorFactory := elector.NewCoordinatorElectorFactory(host, s.BullyConfig)
		coordinators = append(coordinators, tss.NewCoordinator(host, &communication, electorFactory))
		processes = append(processes, signing)
	}
	tsstest.SetupCommunication(communicationMap)

	resultChn := make(chan interface{}, 2)

	ctx, cancel := context.WithCancel(context.Background())
	pool := pool.New().WithContext(ctx)
	for i, coordinator := range coordinators {
		coordinator := coordinator
		pool.Go(func(ctx context.Context) error {
			return coordinator.Execute(ctx, []tss.TssProcess{processes[i]}, resultChn)
		})
	}

	sig1 := <-resultChn
	sig2 := <-resultChn
	s.NotEqual(sig1, sig2)
	if sig1 == nil && sig2 == nil {
		s.Fail("signature is nil")
	}

	time.Sleep(time.Millisecond * 100)
	cancel()
	err := pool.Wait()
	s.Nil(err)
}

func (s *SigningTestSuite) Test_SigningTimeout() {
	communicationMap := make(map[peer.ID]*tsstest.TestCommunication)
	coordinators := []*tss.Coordinator{}
	processes := []tss.TssProcess{}

	for i, host := range s.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		fetcher := keyshare.NewECDSAKeyshareStore(fmt.Sprintf("../../test/keyshares/%d.keyshare", i))

		msgBytes := []byte("Message")
		msg := big.NewInt(0)
		msg.SetBytes(msgBytes)
		signing, err := signing.NewSigning(msg, "signing2", "signing2", host, &communication, fetcher)
		if err != nil {
			panic(err)
		}
		electorFactory := elector.NewCoordinatorElectorFactory(host, s.BullyConfig)
		coordinator := tss.NewCoordinator(host, &communication, electorFactory)
		coordinator.TssTimeout = time.Nanosecond
		coordinators = append(coordinators, coordinator)
		processes = append(processes, signing)
	}
	tsstest.SetupCommunication(communicationMap)

	resultChn := make(chan interface{})
	pool := pool.New().WithContext(context.Background())
	for i, coordinator := range coordinators {
		coordinator := coordinator
		pool.Go(func(ctx context.Context) error {
			return coordinator.Execute(ctx, []tss.TssProcess{processes[i]}, resultChn)
		})
	}

	err := pool.Wait()
	s.NotNil(err)
}

func (s *SigningTestSuite) Test_PendingProcessExists() {
	communicationMap := make(map[peer.ID]*tsstest.TestCommunication)
	coordinators := []*tss.Coordinator{}
	processes := []tss.TssProcess{}
	for _, host := range s.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		keygen := keygen.NewKeygen("keygen3", s.Threshold, host, &communication, s.MockECDSAStorer)
		electorFactory := elector.NewCoordinatorElectorFactory(host, s.BullyConfig)
		coordinators = append(coordinators, tss.NewCoordinator(host, &communication, electorFactory))
		processes = append(processes, keygen)
	}
	tsstest.SetupCommunication(communicationMap)

	s.MockECDSAStorer.EXPECT().LockKeyshare().AnyTimes()
	s.MockECDSAStorer.EXPECT().UnlockKeyshare().AnyTimes()
	pool := pool.New().WithContext(context.Background()).WithCancelOnError()
	for i, coordinator := range coordinators {
		pool.Go(func(ctx context.Context) error { return coordinator.Execute(ctx, []tss.TssProcess{processes[i]}, nil) })
		pool.Go(func(ctx context.Context) error { return coordinator.Execute(ctx, []tss.TssProcess{processes[i]}, nil) })
	}

	err := pool.Wait()
	s.NotNil(err)
}
