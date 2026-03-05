// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package executor

import (
	"context"
	"fmt"
	"math/big"
	"sync"
	"time"

	"github.com/binance-chain/tss-lib/common"
	"github.com/sourcegraph/conc/pool"

	ethCommon "github.com/ethereum/go-ethereum/common"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/rs/zerolog/log"

	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/ecdsa/signing"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
)

type Batch struct {
	proposals []*transfer.TransferProposal
	gasLimit  uint64
}

var (
	executionCheckPeriod = time.Minute
	signingTimeout       = 30 * time.Minute
)

type BridgeContract interface {
	IsProposalExecuted(p *transfer.TransferProposal) (bool, error)
	ExecuteProposals(proposals []*transfer.TransferProposal, signature []byte, opts transactor.TransactOptions) (*ethCommon.Hash, error)
	ProposalsHash(proposals []*transfer.TransferProposal) ([]byte, error)
}

type Executor struct {
	coordinator       *tss.Coordinator
	host              host.Host
	comm              comm.Communication
	fetcher           signing.SaveDataFetcher
	bridge            BridgeContract
	exitLock          *sync.RWMutex
	transactionMaxGas uint64
	transferGasCost   uint64
}

func NewExecutor(
	host host.Host,
	comm comm.Communication,
	coordinator *tss.Coordinator,
	bridgeContract BridgeContract,
	fetcher signing.SaveDataFetcher,
	exitLock *sync.RWMutex,
	transactionMaxGas uint64,
	transferGasCost uint64,
) *Executor {
	return &Executor{
		host:              host,
		comm:              comm,
		coordinator:       coordinator,
		bridge:            bridgeContract,
		fetcher:           fetcher,
		exitLock:          exitLock,
		transactionMaxGas: transactionMaxGas,
		transferGasCost:   transferGasCost,
	}
}

// Execute starts a signing process and executes proposals when signature is generated
func (e *Executor) Execute(proposals []*proposal.Proposal) error {
	e.exitLock.RLock()
	defer e.exitLock.RUnlock()
	batches, err := e.proposalBatches(proposals)
	if err != nil {
		return err
	}

	p := pool.New().WithErrors()
	for i, batch := range batches {
		if len(batch.proposals) == 0 {
			continue
		}
		messageID := batch.proposals[0].MessageID

		b := batch
		p.Go(func() error {
			propHash, err := e.bridge.ProposalsHash(b.proposals)
			if err != nil {
				return err
			}

			sessionID := fmt.Sprintf("%s-%d", messageID, i)
			log.Info().Str("messageID", batch.proposals[0].MessageID).Msgf("Starting session with ID: %s", sessionID)

			msg := big.NewInt(0)
			msg.SetBytes(propHash)
			signing, err := signing.NewSigning(
				msg,
				messageID,
				sessionID,
				e.host,
				e.comm,
				e.fetcher)
			if err != nil {
				return err
			}

			sigChn := make(chan interface{})
			executionContext, cancelExecution := context.WithCancel(context.Background())
			watchContext, cancelWatch := context.WithCancel(context.Background())
			ep := pool.New().WithErrors()
			ep.Go(func() error {
				err := e.coordinator.Execute(executionContext, []tss.TssProcess{signing}, sigChn)
				if err != nil {
					cancelWatch()
				}

				return err
			})
			ep.Go(func() error { return e.watchExecution(watchContext, cancelExecution, b, sigChn, sessionID, messageID) })
			return ep.Wait()
		})
	}
	return p.Wait()
}

func (e *Executor) watchExecution(
	ctx context.Context,
	cancelExecution context.CancelFunc,
	batch *Batch,
	sigChn chan interface{},
	sessionID string,
	messageID string) error {
	ticker := time.NewTicker(executionCheckPeriod)
	timeout := time.NewTicker(signingTimeout)
	defer ticker.Stop()
	defer timeout.Stop()
	defer cancelExecution()

	for {
		select {
		case sigResult := <-sigChn:
			{
				cancelExecution()
				if sigResult == nil {
					continue
				}

				signatureData := sigResult.(*common.SignatureData)
				hash, err := e.executeBatch(batch, signatureData)
				if err != nil {
					_ = e.comm.Broadcast(e.host.Peerstore().Peers(), []byte{}, comm.TssFailMsg, sessionID)
					return err
				}

				log.Info().Str("messageID", messageID).Msgf("Sent proposals execution with hash: %s", hash)
			}
		case <-ticker.C:
			{
				if !e.areProposalsExecuted(batch.proposals) {
					continue
				}

				log.Info().Str("messageID", messageID).Msgf("Successfully executed proposals")
				return nil
			}
		case <-timeout.C:
			{
				return fmt.Errorf("execution timed out in %s", signingTimeout)
			}
		case <-ctx.Done():
			{
				return nil
			}
		}
	}
}

func (e *Executor) proposalBatches(proposals []*proposal.Proposal) ([]*Batch, error) {
	batches := make([]*Batch, 1)
	currentBatch := &Batch{
		proposals: make([]*transfer.TransferProposal, 0),
		gasLimit:  0,
	}
	batches[0] = currentBatch

	for _, prop := range proposals {
		transferProposal := &transfer.TransferProposal{
			Source:      prop.Source,
			Destination: prop.Destination,
			Data:        prop.Data.(transfer.TransferProposalData),
			Type:        prop.Type,
			MessageID:   prop.MessageID,
		}

		isExecuted, err := e.bridge.IsProposalExecuted(transferProposal)
		if err != nil {
			return nil, err
		}
		if isExecuted {
			log.Info().Str("messageID", transferProposal.MessageID).Msgf("Proposal %p already executed", transferProposal)
			continue
		}

		var propGasLimit uint64
		l, ok := transferProposal.Data.Metadata["gasLimit"]
		if ok {
			propGasLimit = l.(uint64) + e.transferGasCost
		} else {
			propGasLimit = e.transferGasCost
		}
		currentBatch.gasLimit += propGasLimit
		if currentBatch.gasLimit >= e.transactionMaxGas {
			currentBatch = &Batch{
				proposals: make([]*transfer.TransferProposal, 0),
				gasLimit:  0,
			}
			batches = append(batches, currentBatch)
		}

		currentBatch.proposals = append(currentBatch.proposals, transferProposal)
	}

	return batches, nil
}

func (e *Executor) executeBatch(batch *Batch, signatureData *common.SignatureData) (*ethCommon.Hash, error) {
	sig := []byte{}
	sig = append(sig[:], ethCommon.LeftPadBytes(signatureData.R, 32)...)
	sig = append(sig[:], ethCommon.LeftPadBytes(signatureData.S, 32)...)
	sig = append(sig[:], signatureData.SignatureRecovery...)
	sig[len(sig)-1] += 27 // Transform V from 0/1 to 27/28

	hash, err := e.bridge.ExecuteProposals(batch.proposals, sig, transactor.TransactOptions{
		GasLimit: batch.gasLimit,
	})
	if err != nil {
		return nil, err
	}

	return hash, err
}

func (e *Executor) areProposalsExecuted(proposals []*transfer.TransferProposal) bool {
	for _, prop := range proposals {
		isExecuted, err := e.bridge.IsProposalExecuted(prop)
		if err != nil || !isExecuted {
			return false
		}
	}

	return true
}
