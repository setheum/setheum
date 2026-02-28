// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package signing_test

import (
	"context"
	"encoding/hex"
	"fmt"
	"testing"
	"time"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/elector"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/frost/signing"
	tsstest "github.com/ChainSafe/sygma-relayer/tss/test"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/sourcegraph/conc/pool"
	"github.com/stretchr/testify/suite"
	"github.com/taurusgroup/multi-party-sig/pkg/math/curve"
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

	tweak := "c82aa6ae534bb28aaafeb3660c31d6a52e187d8f05d48bb6bdb9b733a9b42212"
	tweakBytes, err := hex.DecodeString(tweak)
	s.Nil(err)
	h := &curve.Secp256k1Scalar{}
	err = h.UnmarshalBinary(tweakBytes)
	s.Nil(err)

	fetcher := keyshare.NewFrostKeyshareStore(fmt.Sprintf("../../test/keyshares/%d-frost.keyshare", 0))
	testKeyshare, err := fetcher.GetKeyshare()
	s.Nil(err)
	tweakedKeyshare, err := testKeyshare.Key.Derive(h, nil)
	s.Nil(err)

	msgBytes := []byte("Message")
	for i, host := range s.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		fetcher := keyshare.NewFrostKeyshareStore(fmt.Sprintf("../../test/keyshares/%d-frost.keyshare", i))

		signing, err := signing.NewSigning(1, msgBytes, tweak, "signing1", "signing1", host, &communication, fetcher)
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
	tSig1 := sig1.(signing.Signature)
	tSig2 := sig2.(signing.Signature)
	s.Equal(tweakedKeyshare.PublicKey.Verify(tSig1.Signature, msgBytes), true)
	s.Equal(tweakedKeyshare.PublicKey.Verify(tSig2.Signature, msgBytes), true)
	cancel()
	err = pool.Wait()
	s.Nil(err)
}

func (s *SigningTestSuite) Test_MultipleProcesses() {
	communicationMap := make(map[peer.ID]*tsstest.TestCommunication)
	coordinators := []*tss.Coordinator{}
	processes := [][]tss.TssProcess{}

	tweak := "c82aa6ae534bb28aaafeb3660c31d6a52e187d8f05d48bb6bdb9b733a9b42212"
	tweakBytes, err := hex.DecodeString(tweak)
	s.Nil(err)
	h := &curve.Secp256k1Scalar{}
	err = h.UnmarshalBinary(tweakBytes)
	s.Nil(err)

	msgBytes := []byte("Message")
	for i, host := range s.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		fetcher := keyshare.NewFrostKeyshareStore(fmt.Sprintf("../../test/keyshares/%d-frost.keyshare", i))

		signing1, err := signing.NewSigning(1, msgBytes, tweak, "signing1", "signing1", host, &communication, fetcher)
		if err != nil {
			panic(err)
		}
		signing2, err := signing.NewSigning(1, msgBytes, tweak, "signing1", "signing2", host, &communication, fetcher)
		if err != nil {
			panic(err)
		}
		signing3, err := signing.NewSigning(1, msgBytes, tweak, "signing1", "signing3", host, &communication, fetcher)
		if err != nil {
			panic(err)
		}
		electorFactory := elector.NewCoordinatorElectorFactory(host, s.BullyConfig)
		coordinator := tss.NewCoordinator(host, &communication, electorFactory)
		coordinators = append(coordinators, coordinator)
		processes = append(processes, []tss.TssProcess{signing1, signing2, signing3})
	}
	tsstest.SetupCommunication(communicationMap)

	resultChn := make(chan interface{}, 6)
	ctx, cancel := context.WithCancel(context.Background())
	pool := pool.New().WithContext(ctx)
	for i, coordinator := range coordinators {
		coordinator := coordinator

		pool.Go(func(ctx context.Context) error {
			return coordinator.Execute(ctx, processes[i], resultChn)
		})
	}

	results := make([]signing.Signature, 6)
	i := 0
	for result := range resultChn {
		sig := result.(signing.Signature)
		results[i] = sig
		i++
		if i == 5 {
			break
		}
	}
	err = pool.Wait()
	s.NotNil(err)
	cancel()
}

func (s *SigningTestSuite) Test_ProcessTimeout() {
	communicationMap := make(map[peer.ID]*tsstest.TestCommunication)
	coordinators := []*tss.Coordinator{}
	processes := []tss.TssProcess{}

	tweak := "c82aa6ae534bb28aaafeb3660c31d6a52e187d8f05d48bb6bdb9b733a9b42212"
	tweakBytes, err := hex.DecodeString(tweak)
	s.Nil(err)
	h := &curve.Secp256k1Scalar{}
	err = h.UnmarshalBinary(tweakBytes)
	s.Nil(err)

	msgBytes := []byte("Message")
	for i, host := range s.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		fetcher := keyshare.NewFrostKeyshareStore(fmt.Sprintf("../../test/keyshares/%d-frost.keyshare", i))

		signing, err := signing.NewSigning(1, msgBytes, tweak, "signing1", "signing1", host, &communication, fetcher)
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

	ctx, cancel := context.WithCancel(context.Background())
	pool := pool.New().WithContext(ctx)
	for i, coordinator := range coordinators {
		coordinator := coordinator
		pool.Go(func(ctx context.Context) error {
			return coordinator.Execute(ctx, []tss.TssProcess{processes[i]}, resultChn)
		})
	}

	err = pool.Wait()
	s.NotNil(err)
	cancel()
}
