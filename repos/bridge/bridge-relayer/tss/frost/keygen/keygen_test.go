// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package keygen_test

import (
	"context"
	"testing"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/elector"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/frost/keygen"
	tsstest "github.com/ChainSafe/sygma-relayer/tss/test"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/sourcegraph/conc/pool"
	"github.com/stretchr/testify/suite"
	"go.uber.org/mock/gomock"
)

type KeygenTestSuite struct {
	tsstest.CoordinatorTestSuite
}

func TestRunKeygenTestSuite(t *testing.T) {
	suite.Run(t, new(KeygenTestSuite))
}

func (s *KeygenTestSuite) Test_ValidKeygenProcess() {
	communicationMap := make(map[peer.ID]*tsstest.TestCommunication)
	coordinators := []*tss.Coordinator{}
	processes := []tss.TssProcess{}

	for _, host := range s.CoordinatorTestSuite.Hosts {
		communication := tsstest.TestCommunication{
			Host:          host,
			Subscriptions: make(map[comm.SubscriptionID]chan *comm.WrappedMessage),
		}
		communicationMap[host.ID()] = &communication
		s.MockFrostStorer.EXPECT().LockKeyshare()
		keygen := keygen.NewKeygen("keygen", s.Threshold, host, &communication, s.MockFrostStorer)
		electorFactory := elector.NewCoordinatorElectorFactory(host, s.BullyConfig)
		coordinators = append(coordinators, tss.NewCoordinator(host, &communication, electorFactory))
		processes = append(processes, keygen)
	}
	tsstest.SetupCommunication(communicationMap)
	s.MockFrostStorer.EXPECT().StoreKeyshare(gomock.Any()).Times(3)
	s.MockFrostStorer.EXPECT().UnlockKeyshare().Times(3)

	pool := pool.New().WithContext(context.Background()).WithCancelOnError()
	for i, coordinator := range coordinators {
		pool.Go(func(ctx context.Context) error { return coordinator.Execute(ctx, []tss.TssProcess{processes[i]}, nil) })
	}

	err := pool.Wait()
	s.Nil(err)
}
