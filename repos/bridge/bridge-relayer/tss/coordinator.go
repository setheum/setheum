// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package tss

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/comm/elector"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/common"
	"github.com/ChainSafe/sygma-relayer/tss/message"
	"github.com/binance-chain/tss-lib/tss"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"golang.org/x/exp/slices"
)

var (
	initiatePeriod     = 15 * time.Second
	coordinatorTimeout = 3 * time.Minute
	tssTimeout         = 15 * time.Minute
)

type TssProcess interface {
	Run(ctx context.Context, coordinator bool, resultChn chan interface{}, params []byte) error
	Stop()
	Ready(readyPeers []peer.ID, excludedPeers []peer.ID) (bool, error)
	Retryable() bool
	StartParams(readyPeers []peer.ID) []byte
	SessionID() string
	ValidCoordinators() []peer.ID
}

type Coordinator struct {
	host           host.Host
	communication  comm.Communication
	electorFactory *elector.CoordinatorElectorFactory

	pendingProcesses map[string]bool
	processLock      sync.Mutex

	CoordinatorTimeout time.Duration
	TssTimeout         time.Duration
	InitiatePeriod     time.Duration
}

func NewCoordinator(
	host host.Host,
	communication comm.Communication,
	electorFactory *elector.CoordinatorElectorFactory,
) *Coordinator {
	return &Coordinator{
		host:           host,
		communication:  communication,
		electorFactory: electorFactory,

		pendingProcesses: make(map[string]bool),

		CoordinatorTimeout: coordinatorTimeout,
		TssTimeout:         tssTimeout,
		InitiatePeriod:     initiatePeriod,
	}
}

// Execute calculates process leader and coordinates party readiness and start the tss processes.
// Array of processes can be passed if all the processes have to have the same peer subset and
// the result of all of them is needed. The processes should have an unique session ID for each one.
func (c *Coordinator) Execute(ctx context.Context, tssProcesses []TssProcess, resultChn chan interface{}) error {
	sessionID := tssProcesses[0].SessionID()
	value, ok := c.pendingProcesses[sessionID]
	if ok && value {
		log.Warn().Str("SessionID", sessionID).Msgf("Process already pending")
		return fmt.Errorf("process already pending")
	}

	c.processLock.Lock()
	c.pendingProcesses[sessionID] = true
	c.processLock.Unlock()

	ctx, cancel := context.WithCancel(ctx)
	p := pool.New().WithContext(ctx).WithCancelOnError()
	defer func() {
		cancel()
		c.communication.CloseSession(sessionID)
		c.processLock.Lock()
		c.pendingProcesses[sessionID] = false
		c.processLock.Unlock()
		for _, process := range tssProcesses {
			process.Stop()
		}
	}()

	coordinatorElector := c.electorFactory.CoordinatorElector(sessionID, elector.Static)
	coordinator, _ := coordinatorElector.Coordinator(ctx, tssProcesses[0].ValidCoordinators())

	log.Info().Str("SessionID", sessionID).Msgf("Starting process with coordinator %s", coordinator.Pretty())

	p.Go(func(ctx context.Context) error {
		err := c.start(ctx, tssProcesses, coordinator, resultChn, []peer.ID{})
		if err == nil {
			cancel()
		}
		return err
	})
	p.Go(func(ctx context.Context) error {
		return c.watchExecution(ctx, tssProcesses[0], coordinator)
	})
	err := p.Wait()
	if err == nil {
		return nil
	}

	if !tssProcesses[0].Retryable() {
		return err
	}

	return c.handleError(ctx, err, tssProcesses, resultChn)
}

func (c *Coordinator) handleError(ctx context.Context, err error, tssProcesses []TssProcess, resultChn chan interface{}) error {
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	rp := pool.New().WithContext(ctx).WithCancelOnError()
	rp.Go(func(ctx context.Context) error {
		return c.watchExecution(ctx, tssProcesses[0], peer.ID(""))
	})
	sessionID := tssProcesses[0].SessionID()
	switch err := err.(type) {
	case *CoordinatorError:
		{
			log.Warn().Str("SessionID", sessionID).Msgf("Tss process failed with error %+v", err)

			excludedPeers := []peer.ID{err.Peer}
			rp.Go(func(ctx context.Context) error { return c.retry(ctx, tssProcesses, resultChn, excludedPeers) })
		}
	case *comm.CommunicationError:
		{
			log.Err(err).Str("SessionID", sessionID).Msgf("Tss process failed with error %+v", err)
			rp.Go(func(ctx context.Context) error { return c.retry(ctx, tssProcesses, resultChn, []peer.ID{}) })
		}
	case *tss.Error:
		{
			log.Err(err).Str("SessionID", sessionID).Msgf("Tss process failed with error %+v", err)
			excludedPeers, err := common.PeersFromParties(err.Culprits())
			if err != nil {
				return err
			}
			rp.Go(func(ctx context.Context) error { return c.retry(ctx, tssProcesses, resultChn, excludedPeers) })
		}
	case *SubsetError:
		{
			// wait for start message if existing singing process fails
			rp.Go(func(ctx context.Context) error {
				return c.waitForStart(ctx, tssProcesses, resultChn, peer.ID(""), c.TssTimeout)
			})
		}
	default:
		{
			return err
		}
	}
	return rp.Wait()
}

func (c *Coordinator) watchExecution(ctx context.Context, tssProcess TssProcess, coordinator peer.ID) error {
	failChn := make(chan *comm.WrappedMessage)
	subscriptionID := c.communication.Subscribe(tssProcess.SessionID(), comm.TssFailMsg, failChn)
	ticker := time.NewTicker(c.TssTimeout)
	defer func() {
		ticker.Stop()
		c.communication.UnSubscribe(subscriptionID)
	}()

	for {
		select {
		case <-ticker.C:
			{
				return fmt.Errorf("tss process timed out after %v", c.TssTimeout)
			}
		case <-ctx.Done():
			{
				return nil
			}
		case msg := <-failChn:
			{
				// ignore messages that are not from coordinator
				if msg.From.Pretty() != coordinator.Pretty() {
					continue
				}

				return fmt.Errorf("tss fail message received for process %s", tssProcess.SessionID())
			}
		}
	}
}

// start initiates listeners for coordinator and participants with static calculated coordinator
func (c *Coordinator) start(ctx context.Context, tssProcesses []TssProcess, coordinator peer.ID, resultChn chan interface{}, excludedPeers []peer.ID) error {
	if coordinator.Pretty() == c.host.ID().Pretty() {
		return c.initiate(ctx, tssProcesses, resultChn, excludedPeers)
	} else {
		return c.waitForStart(ctx, tssProcesses, resultChn, coordinator, c.CoordinatorTimeout)
	}
}

// retry initiates full bully process to calculate coordinator and starts a new tss process after
// an expected error ocurred during regular tss execution
func (c *Coordinator) retry(ctx context.Context, tssProcesses []TssProcess, resultChn chan interface{}, excludedPeers []peer.ID) error {
	coordinatorElector := c.electorFactory.CoordinatorElector(tssProcesses[0].SessionID(), elector.Bully)
	coordinator, err := coordinatorElector.Coordinator(ctx, common.ExcludePeers(tssProcesses[0].ValidCoordinators(), excludedPeers))
	if err != nil {
		return err
	}

	return c.start(ctx, tssProcesses, coordinator, resultChn, excludedPeers)
}

// broadcastInitiateMsg sends TssInitiateMsg to all peers
func (c *Coordinator) broadcastInitiateMsg(sessionID string) {
	log.Debug().Str("SessionID", sessionID).Msgf("broadcasted initiate message")
	_ = c.communication.Broadcast(
		c.host.Peerstore().Peers(), []byte{}, comm.TssInitiateMsg, sessionID,
	)
}

// initiate sends initiate message to all peers and waits
// for ready response. After tss process declares that enough
// peers are ready, start message is broadcasted and tss process is started.
func (c *Coordinator) initiate(ctx context.Context, tssProcesses []TssProcess, resultChn chan interface{}, excludedPeers []peer.ID) error {
	readyChan := make(chan *comm.WrappedMessage)
	readyPeers := make([]peer.ID, 0)
	readyPeers = append(readyPeers, c.host.ID())

	tssProcess := tssProcesses[0]
	subID := c.communication.Subscribe(tssProcess.SessionID(), comm.TssReadyMsg, readyChan)
	defer c.communication.UnSubscribe(subID)

	ticker := time.NewTicker(c.InitiatePeriod)
	defer ticker.Stop()
	c.broadcastInitiateMsg(tssProcess.SessionID())
	for {
		select {
		case wMsg := <-readyChan:
			{
				log.Debug().Str("SessionID", tssProcess.SessionID()).Msgf("received ready message from %s", wMsg.From)
				if !slices.Contains(excludedPeers, wMsg.From) && !slices.Contains(readyPeers, wMsg.From) {
					readyPeers = append(readyPeers, wMsg.From)
				}
				ready, err := tssProcess.Ready(readyPeers, excludedPeers)
				if err != nil {
					return err
				}
				if !ready {
					continue
				}

				startParams := tssProcess.StartParams(readyPeers)
				startMsgBytes, err := message.MarshalStartMessage(startParams)
				if err != nil {
					return err
				}

				_ = c.communication.Broadcast(c.host.Peerstore().Peers(), startMsgBytes, comm.TssStartMsg, tssProcess.SessionID())
				p := pool.New().WithContext(ctx).WithCancelOnError()
				for _, process := range tssProcesses {
					tssProcess := process
					p.Go(func(ctx context.Context) error {
						return tssProcess.Run(ctx, true, resultChn, startParams)
					})
				}
				return p.Wait()
			}
		case <-ticker.C:
			{
				c.broadcastInitiateMsg(tssProcess.SessionID())
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}

// waitForStart responds to initiate messages and starts the tss process
// when it receives the start message.
func (c *Coordinator) waitForStart(
	ctx context.Context,
	tssProcesses []TssProcess,
	resultChn chan interface{},
	coordinator peer.ID,
	timeout time.Duration,
) error {
	msgChan := make(chan *comm.WrappedMessage)
	startMsgChn := make(chan *comm.WrappedMessage)

	tssProcess := tssProcesses[0]
	initSubID := c.communication.Subscribe(tssProcess.SessionID(), comm.TssInitiateMsg, msgChan)
	defer c.communication.UnSubscribe(initSubID)
	startSubID := c.communication.Subscribe(tssProcess.SessionID(), comm.TssStartMsg, startMsgChn)
	defer c.communication.UnSubscribe(startSubID)

	coordinatorTimeoutTicker := time.NewTicker(timeout)
	defer coordinatorTimeoutTicker.Stop()
	for {
		select {
		case wMsg := <-msgChan:
			{
				if coordinator != "" && wMsg.From != coordinator {
					log.Warn().Msgf("Received initate message from a peer %s that is not the coordinator %s", wMsg.From.Pretty(), coordinator.Pretty())
					continue
				}

				coordinatorTimeoutTicker.Reset(timeout)

				log.Debug().Str("SessionID", tssProcess.SessionID()).Msgf("sent ready message to %s", wMsg.From)
				_ = c.communication.Broadcast(
					peer.IDSlice{wMsg.From}, []byte{}, comm.TssReadyMsg, tssProcess.SessionID(),
				)
			}
		case startMsg := <-startMsgChn:
			{
				log.Debug().Str("SessionID", tssProcess.SessionID()).Msgf("received start message from %s", startMsg.From)

				// having startMsg.From as "" is special case when peer is not selected in subset
				// but should wait for start message if existing singing process fails
				if coordinator != "" && startMsg.From != coordinator {
					log.Warn().Msgf("Received start message from a peer %s that is not the coordinator %s", startMsg.From.Pretty(), coordinator.Pretty())
					continue
				}

				msg, err := message.UnmarshalStartMessage(startMsg.Payload)
				if err != nil {
					return err
				}

				p := pool.New().WithContext(ctx).WithCancelOnError()
				for _, process := range tssProcesses {
					tssProcess := process
					p.Go(func(ctx context.Context) error {
						return tssProcess.Run(ctx, false, resultChn, msg.Params)
					})
				}
				return p.Wait()
			}
		case <-coordinatorTimeoutTicker.C:
			{
				return &CoordinatorError{Peer: coordinator}
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}
