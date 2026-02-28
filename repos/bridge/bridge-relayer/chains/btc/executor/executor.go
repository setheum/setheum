package executor

import (
	"context"
	"encoding/hex"
	"fmt"
	"sync"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains/btc/config"
	"github.com/ChainSafe/sygma-relayer/chains/btc/connection"
	"github.com/ChainSafe/sygma-relayer/chains/btc/mempool"
	"github.com/ChainSafe/sygma-relayer/chains/btc/uploader"
	"github.com/ChainSafe/sygma-relayer/comm"
	"github.com/ChainSafe/sygma-relayer/store"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ChainSafe/sygma-relayer/tss/frost/signing"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/btcsuite/btcd/txscript"
	"github.com/btcsuite/btcd/wire"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/rs/zerolog/log"
	"github.com/sourcegraph/conc/pool"
	"github.com/sygmaprotocol/sygma-core/relayer/proposal"
	"github.com/taurusgroup/multi-party-sig/pkg/taproot"
	"go.uber.org/zap/buffer"
)

var (
	signingTimeout = 30 * time.Minute

	INPUT_SIZE          uint64 = 180
	OUTPUT_SIZE         uint64 = 34
	FEE_ROUNDING_FACTOR uint64 = 5
)

type MempoolAPI interface {
	RecommendedFee() (*mempool.Fee, error)
	Utxos(address string) ([]mempool.Utxo, error)
}

type Executor struct {
	coordinator *tss.Coordinator
	host        host.Host
	comm        comm.Communication

	conn      *connection.Connection
	resources map[[32]byte]config.Resource
	chainCfg  chaincfg.Params
	mempool   MempoolAPI
	fetcher   signing.SaveDataFetcher

	propStorer PropStorer
	propMutex  sync.Mutex

	exitLock *sync.RWMutex
	uploader uploader.Uploader
}

func NewExecutor(
	propStorer PropStorer,
	host host.Host,
	comm comm.Communication,
	coordinator *tss.Coordinator,
	fetcher signing.SaveDataFetcher,
	conn *connection.Connection,
	mempool MempoolAPI,
	resources map[[32]byte]config.Resource,
	chainCfg chaincfg.Params,
	exitLock *sync.RWMutex,
	uploader uploader.Uploader,
) *Executor {
	return &Executor{
		propStorer:  propStorer,
		host:        host,
		comm:        comm,
		coordinator: coordinator,
		exitLock:    exitLock,
		fetcher:     fetcher,
		conn:        conn,
		resources:   resources,
		mempool:     mempool,
		chainCfg:    chainCfg,
		uploader:    uploader,
	}
}

// Execute starts a signing process and executes proposals when signature is generated
func (e *Executor) Execute(proposals []*proposal.Proposal) error {
	e.exitLock.RLock()
	defer e.exitLock.RUnlock()

	messageID := proposals[0].MessageID
	props, err := e.proposalsForExecution(proposals, messageID)
	if err != nil {
		return err
	}
	if len(props) == 0 {
		return nil
	}

	propsPerResource := make(map[[32]byte][]*BtcTransferProposal)
	for _, prop := range props {
		propsPerResource[prop.Data.ResourceId] = append(propsPerResource[prop.Data.ResourceId], prop)
	}

	p := pool.New().WithErrors()
	for resourceID, props := range propsPerResource {
		resourceID := resourceID
		props := props

		p.Go(func() error {
			resource, ok := e.resources[resourceID]
			if !ok {
				return fmt.Errorf("no resource for ID %s", hex.EncodeToString(resourceID[:]))
			}

			return e.executeResourceProps(props, resource, messageID)
		})
	}
	return p.Wait()
}

func (e *Executor) executeResourceProps(props []*BtcTransferProposal, resource config.Resource, messageID string) error {
	log.Info().Str("messageID", messageID).Msgf("Executing proposals %+v for resource %s", props, hex.EncodeToString(resource.ResourceID[:]))

	tx, utxos, err := e.rawTx(props, resource)
	if err != nil {
		return err
	}

	sigChn := make(chan interface{}, len(tx.TxIn))
	p := pool.New().WithErrors()
	executionContext, cancelExecution := context.WithCancel(context.Background())
	watchContext, cancelWatch := context.WithCancel(context.Background())
	sessionID := fmt.Sprintf("%s-%s", messageID, hex.EncodeToString(resource.ResourceID[:]))
	defer cancelWatch()
	p.Go(func() error {
		return e.watchExecution(watchContext, cancelExecution, tx, props, sigChn, sessionID, messageID)
	})
	prevOuts := make(map[wire.OutPoint]*wire.TxOut)
	for _, utxo := range utxos {
		txOut := wire.NewTxOut(int64(utxo.Value), resource.Script)
		hash, err := chainhash.NewHashFromStr(utxo.TxID)
		if err != nil {
			return err
		}
		prevOuts[*wire.NewOutPoint(hash, utxo.Vout)] = txOut
	}
	prevOutputFetcher := txscript.NewMultiPrevOutFetcher(prevOuts)
	sigHashes := txscript.NewTxSigHashes(tx, prevOutputFetcher)

	var buf buffer.Buffer
	_ = tx.Serialize(&buf)
	bytes := buf.Bytes()
	log.Info().Str("messageID", messageID).Msgf("Assembled raw unsigned transaction %s", hex.EncodeToString(bytes))

	// we need to sign each input individually
	tssProcesses := make([]tss.TssProcess, len(tx.TxIn))
	for i := range tx.TxIn {
		signingHash, err := txscript.CalcTaprootSignatureHash(sigHashes, txscript.SigHashDefault, tx, i, prevOutputFetcher)
		sessionID := hex.EncodeToString(signingHash)
		if err != nil {
			return err
		}
		signing, err := signing.NewSigning(
			i,
			signingHash,
			resource.Tweak,
			messageID,
			sessionID,
			e.host,
			e.comm,
			e.fetcher)
		if err != nil {
			return err
		}
		tssProcesses[i] = signing
	}
	p.Go(func() error {
		return e.coordinator.Execute(executionContext, tssProcesses, sigChn)
	})
	return p.Wait()
}

func (e *Executor) watchExecution(
	ctx context.Context,
	cancelExecution context.CancelFunc,
	tx *wire.MsgTx,
	proposals []*BtcTransferProposal,
	sigChn chan interface{},
	sessionID string,
	messageID string) error {
	timeout := time.NewTicker(signingTimeout)
	defer timeout.Stop()
	defer cancelExecution()
	signatures := make([]taproot.Signature, len(tx.TxIn))

	for {
		select {
		case sigResult := <-sigChn:
			{
				if sigResult == nil {
					continue
				}
				signatureData := sigResult.(signing.Signature)
				signatures[signatureData.Id] = signatureData.Signature
				if !e.signaturesFilled(signatures) {
					continue
				}
				cancelExecution()

				hash, err := e.sendTx(tx, signatures, messageID)
				if err != nil {
					_ = e.comm.Broadcast(e.host.Peerstore().Peers(), []byte{}, comm.TssFailMsg, sessionID)
					e.storeProposalsStatus(proposals, store.FailedProp)
					return err
				}

				e.storeProposalsStatus(proposals, store.ExecutedProp)
				log.Info().Str("messageID", messageID).Msgf("Sent proposals execution with hash: %s", hash)
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

func (e *Executor) rawTx(proposals []*BtcTransferProposal, resource config.Resource) (*wire.MsgTx, []mempool.Utxo, error) {
	tx := wire.NewMsgTx(wire.TxVersion)
	outputAmount, err := e.outputs(tx, proposals)
	if err != nil {
		return nil, nil, err
	}
	feeEstimate, err := e.fee(uint64(len(proposals)), uint64(len(proposals)))
	if err != nil {
		return nil, nil, err
	}
	inputAmount, utxos, err := e.inputs(tx, resource.Address, outputAmount+feeEstimate)
	if err != nil {
		return nil, nil, err
	}
	if inputAmount < outputAmount {
		return nil, nil, fmt.Errorf("utxo input amount %d less than output amount %d", inputAmount, outputAmount)
	}
	fee, err := e.fee(uint64(len(utxos)), uint64(len(proposals))+1)
	if err != nil {
		return nil, nil, err
	}

	returnAmount := inputAmount - fee - outputAmount
	if returnAmount > 0 {
		// return extra funds
		returnScript, err := txscript.PayToAddrScript(resource.Address)
		if err != nil {
			return nil, nil, err
		}
		txOut := wire.NewTxOut(int64(returnAmount), returnScript)
		tx.AddTxOut(txOut)
	}
	return tx, utxos, err
}

func (e *Executor) outputs(tx *wire.MsgTx, proposals []*BtcTransferProposal) (uint64, error) {
	outputAmount := uint64(0)
	var dataToUpload []map[string]interface{}

	for _, prop := range proposals {
		addr, err := btcutil.DecodeAddress(prop.Data.Recipient, &e.chainCfg)
		if err != nil {
			return 0, err
		}
		destinationAddrByte, err := txscript.PayToAddrScript(addr)
		if err != nil {
			return 0, err
		}

		dataToUpload = append(dataToUpload, map[string]interface{}{
			"sourceDomain": prop.Source,
			"depositNonce": prop.Data.DepositNonce,
		})

		txOut := wire.NewTxOut(int64(prop.Data.Amount), destinationAddrByte)
		tx.AddTxOut(txOut)

		outputAmount += prop.Data.Amount
	}

	// Upload to IPFS
	cid, err := e.uploader.Upload(dataToUpload)
	if err != nil {
		log.Error().Err(err).Msg("Error occured while uploading metadata to ipfs")
		return 0, err
	}

	// Store the CID in OP_RETURN
	opReturnData := []byte("syg_" + cid)
	opReturnScript, err := txscript.NullDataScript(opReturnData)
	if err != nil {
		log.Error().Err(err).Msg("Error occured while constructiong OP_RETURN data")
		return 0, err
	}

	opReturnOut := wire.NewTxOut(0, opReturnScript)
	tx.AddTxOut(opReturnOut)
	return outputAmount, nil
}

func (e *Executor) inputs(tx *wire.MsgTx, address btcutil.Address, outputAmount uint64) (uint64, []mempool.Utxo, error) {
	usedUtxos := make([]mempool.Utxo, 0)
	inputAmount := uint64(0)
	utxos, err := e.mempool.Utxos(address.String())
	if err != nil {
		return 0, nil, err
	}
	for _, utxo := range utxos {
		previousTxHash, err := chainhash.NewHashFromStr(utxo.TxID)
		if err != nil {
			return 0, nil, err
		}
		outPoint := wire.NewOutPoint(previousTxHash, utxo.Vout)
		txIn := wire.NewTxIn(outPoint, nil, nil)
		tx.AddTxIn(txIn)

		usedUtxos = append(usedUtxos, utxo)
		inputAmount += uint64(utxo.Value)
		if inputAmount > outputAmount {
			break
		}
	}
	return inputAmount, usedUtxos, nil
}

func (e *Executor) fee(numOfInputs, numOfOutputs uint64) (uint64, error) {
	recommendedFee, err := e.mempool.RecommendedFee()
	if err != nil {
		return 0, err
	}

	return (numOfInputs*INPUT_SIZE + numOfOutputs*OUTPUT_SIZE) * ((recommendedFee.EconomyFee/FEE_ROUNDING_FACTOR)*FEE_ROUNDING_FACTOR + FEE_ROUNDING_FACTOR), nil
}

func (e *Executor) sendTx(tx *wire.MsgTx, signatures []taproot.Signature, messageID string) (*chainhash.Hash, error) {
	for i, sig := range signatures {
		tx.TxIn[i].Witness = wire.TxWitness{sig}
	}

	var buf buffer.Buffer
	err := tx.Serialize(&buf)
	if err != nil {
		return nil, err
	}
	bytes := buf.Bytes()
	log.Debug().Str("messageID", messageID).Msgf("Assembled raw transaction %s", hex.EncodeToString(bytes))
	return e.conn.SendRawTransaction(tx, true)
}

func (e *Executor) signaturesFilled(signatures []taproot.Signature) bool {
	for _, signature := range signatures {
		if len([]byte(signature)) == 0 {
			return false
		}
	}

	return true
}

func (e *Executor) proposalsForExecution(proposals []*proposal.Proposal, messageID string) ([]*BtcTransferProposal, error) {
	e.propMutex.Lock()
	props := make([]*BtcTransferProposal, 0)
	for _, prop := range proposals {
		executed, err := e.isExecuted(prop)
		if err != nil {
			return props, err
		}

		if executed {
			log.Warn().Str("messageID", messageID).Msgf("Proposal %s already executed", fmt.Sprintf("%d-%d-%d", prop.Source, prop.Destination, prop.Data.(BtcTransferProposalData).DepositNonce))
			continue
		}

		err = e.propStorer.StorePropStatus(prop.Source, prop.Destination, prop.Data.(BtcTransferProposalData).DepositNonce, store.PendingProp)
		if err != nil {
			return props, err
		}
		props = append(props, &BtcTransferProposal{
			Source:      prop.Source,
			Destination: prop.Destination,
			Data:        prop.Data.(BtcTransferProposalData),
		})
	}
	e.propMutex.Unlock()
	return props, nil
}

func (e *Executor) isExecuted(prop *proposal.Proposal) (bool, error) {
	status, err := e.propStorer.PropStatus(prop.Source, prop.Destination, prop.Data.(BtcTransferProposalData).DepositNonce)
	if err != nil {
		return true, err
	}

	if status == store.MissingProp || status == store.FailedProp {
		return false, nil
	}
	return true, err
}

func (e *Executor) storeProposalsStatus(props []*BtcTransferProposal, status store.PropStatus) {
	e.propMutex.Lock()
	for _, prop := range props {
		err := e.propStorer.StorePropStatus(
			prop.Source,
			prop.Destination,
			prop.Data.DepositNonce,
			status)
		if err != nil {
			log.Err(err).Msgf("Failed storing proposal %+v status %s", prop, status)
		}
	}
	e.propMutex.Unlock()
}
