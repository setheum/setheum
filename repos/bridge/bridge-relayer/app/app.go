// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

package app

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/ChainSafe/sygma-relayer/chains"
	"github.com/ChainSafe/sygma-relayer/chains/btc"
	"github.com/ChainSafe/sygma-relayer/chains/btc/mempool"
	"github.com/ChainSafe/sygma-relayer/chains/btc/uploader"
	"github.com/ChainSafe/sygma-relayer/chains/evm"
	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/contracts/bridge"
	"github.com/ChainSafe/sygma-relayer/chains/evm/calls/events"
	"github.com/ChainSafe/sygma-relayer/chains/evm/executor"
	"github.com/ChainSafe/sygma-relayer/chains/evm/listener/depositHandlers"
	evmEventHandlers "github.com/ChainSafe/sygma-relayer/chains/evm/listener/eventHandlers"
	"github.com/ChainSafe/sygma-relayer/chains/substrate"
	"github.com/ChainSafe/sygma-relayer/relayer/retry"
	"github.com/ChainSafe/sygma-relayer/relayer/transfer"
	propStore "github.com/ChainSafe/sygma-relayer/store"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/gas"
	coreSubstrate "github.com/sygmaprotocol/sygma-core/chains/substrate"
	"github.com/sygmaprotocol/sygma-core/crypto/secp256k1"
	"github.com/sygmaprotocol/sygma-core/observability"
	"github.com/sygmaprotocol/sygma-core/relayer"
	"github.com/sygmaprotocol/sygma-core/relayer/message"
	"github.com/sygmaprotocol/sygma-core/store"
	"github.com/sygmaprotocol/sygma-core/store/lvldb"

	btcConfig "github.com/ChainSafe/sygma-relayer/chains/btc/config"
	btcConnection "github.com/ChainSafe/sygma-relayer/chains/btc/connection"
	btcExecutor "github.com/ChainSafe/sygma-relayer/chains/btc/executor"
	btcListener "github.com/ChainSafe/sygma-relayer/chains/btc/listener"
	substrateExecutor "github.com/ChainSafe/sygma-relayer/chains/substrate/executor"
	substrateListener "github.com/ChainSafe/sygma-relayer/chains/substrate/listener"
	substratePallet "github.com/ChainSafe/sygma-relayer/chains/substrate/pallet"
	"github.com/ChainSafe/sygma-relayer/metrics"
	"github.com/centrifuge/go-substrate-rpc-client/v4/signature"
	coreEvm "github.com/sygmaprotocol/sygma-core/chains/evm"
	evmClient "github.com/sygmaprotocol/sygma-core/chains/evm/client"
	"github.com/sygmaprotocol/sygma-core/chains/evm/listener"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/monitored"
	"github.com/sygmaprotocol/sygma-core/chains/evm/transactor/transaction"
	substrateClient "github.com/sygmaprotocol/sygma-core/chains/substrate/client"
	"github.com/sygmaprotocol/sygma-core/chains/substrate/connection"
	coreSubstrateListener "github.com/sygmaprotocol/sygma-core/chains/substrate/listener"

	"github.com/ChainSafe/sygma-relayer/comm/elector"
	"github.com/ChainSafe/sygma-relayer/comm/p2p"
	"github.com/ChainSafe/sygma-relayer/config"
	"github.com/ChainSafe/sygma-relayer/health"
	"github.com/ChainSafe/sygma-relayer/jobs"
	"github.com/ChainSafe/sygma-relayer/keyshare"
	"github.com/ChainSafe/sygma-relayer/topology"
	"github.com/ChainSafe/sygma-relayer/tss"
	"github.com/ethereum/go-ethereum/common"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/rs/zerolog/log"
	"github.com/spf13/viper"
)

var Version string

func Run() error {
	var err error

	configFlag := viper.GetString(config.ConfigFlagName)
	configURL := viper.GetString("config-url")

	var configuration *config.Config
	if configURL != "" {
		configuration, err = config.GetSharedConfigFromNetwork(configURL)
		panicOnError(err)
	}

	if strings.ToLower(configFlag) == "env" {
		configuration, err = config.GetConfigFromENV(configuration)
		panicOnError(err)
	} else {
		configuration, err = config.GetConfigFromFile(configFlag, configuration)
		panicOnError(err)
	}

	observability.ConfigureLogger(configuration.RelayerConfig.LogLevel, os.Stdout)

	log.Info().Msg("Successfully loaded configuration")

	topologyProvider, err := topology.NewNetworkTopologyProvider(configuration.RelayerConfig.MpcConfig.TopologyConfiguration, http.DefaultClient)
	panicOnError(err)
	topologyStore := topology.NewTopologyStore(configuration.RelayerConfig.MpcConfig.TopologyConfiguration.Path)
	networkTopology, err := topologyStore.Topology()
	// if topology is not already in file, read from provider
	if err != nil {
		networkTopology, err = topologyProvider.NetworkTopology("")
		panicOnError(err)

		err = topologyStore.StoreTopology(networkTopology)
		panicOnError(err)
	}
	log.Info().Msgf("Successfully loaded topology")

	privBytes, err := crypto.ConfigDecodeKey(configuration.RelayerConfig.MpcConfig.Key)
	panicOnError(err)

	priv, err := crypto.UnmarshalPrivateKey(privBytes)
	panicOnError(err)

	connectionGate := p2p.NewConnectionGate(networkTopology)
	host, err := p2p.NewHost(priv, networkTopology, connectionGate, configuration.RelayerConfig.MpcConfig.Port)
	panicOnError(err)
	log.Info().Str("peerID", host.ID().String()).Msg("Successfully created libp2p host")

	go health.StartHealthEndpoint(configuration.RelayerConfig.HealthPort)

	communication := p2p.NewCommunication(host, "p2p/sygma")
	electorFactory := elector.NewCoordinatorElectorFactory(host, configuration.RelayerConfig.BullyConfig)
	coordinator := tss.NewCoordinator(host, communication, electorFactory)

	// this is temporary solution related to specifics of aws deployment
	// effectively it waits until old instance is killed
	var db *lvldb.LVLDB
	for {
		db, err = lvldb.NewLvlDB(viper.GetString(config.BlockstoreFlagName))
		if err != nil {
			log.Error().Err(err).Msg("Unable to connect to blockstore file, retry in 10 seconds")
			time.Sleep(10 * time.Second)
		} else {
			log.Info().Msg("Successfully connected to blockstore file")
			break
		}
	}
	blockstore := store.NewBlockStore(db)
	keyshareStore := keyshare.NewECDSAKeyshareStore(configuration.RelayerConfig.MpcConfig.KeysharePath)
	frostKeyshareStore := keyshare.NewFrostKeyshareStore(configuration.RelayerConfig.MpcConfig.FrostKeysharePath)
	propStore := propStore.NewPropStore(db)

	// wait until executions are done and then stop further executions before exiting
	exitLock := &sync.RWMutex{}
	defer exitLock.Lock()

	mp, err := observability.InitMetricProvider(context.Background(), configuration.RelayerConfig.OpenTelemetryCollectorURL)
	if err != nil {
		panic(err)
	}
	defer func() {
		if err := mp.Shutdown(context.Background()); err != nil {
			log.Error().Msgf("Error shutting down meter provider: %v", err)
		}
	}()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	sygmaMetrics, err := metrics.NewSygmaMetrics(ctx, mp.Meter("relayer-metric-provider"), configuration.RelayerConfig.Env, configuration.RelayerConfig.Id, Version)
	if err != nil {
		panic(err)
	}
	msgChan := make(chan []*message.Message)

	domains := make(map[uint8]relayer.RelayedChain)
	for _, chainConfig := range configuration.ChainConfigs {
		switch chainConfig["type"] {
		case "evm":
			{
				config, err := evm.NewEVMConfig(chainConfig)
				panicOnError(err)
				kp, err := secp256k1.NewKeypairFromString(config.GeneralChainConfig.Key)
				panicOnError(err)

				client, err := evmClient.NewEVMClient(config.GeneralChainConfig.Endpoint, kp)
				panicOnError(err)

				log.Info().Str("domain", config.String()).Msgf("Registering EVM domain")

				bridgeAddress := common.HexToAddress(config.Bridge)
				frostAddress := common.HexToAddress(config.FrostKeygen)
				gasPricer := gas.NewLondonGasPriceClient(client, &gas.GasPricerOpts{
					UpperLimitFeePerGas: config.MaxGasPrice,
					GasPriceFactor:      config.GasMultiplier,
				})
				t := monitored.NewMonitoredTransactor(*config.GeneralChainConfig.Id, transaction.NewTransaction, gasPricer, sygmaMetrics, client, config.MaxGasPrice, config.GasIncreasePercentage)
				go t.Monitor(ctx, time.Minute*3, time.Minute*10, time.Minute)
				bridgeContract := bridge.NewBridgeContract(client, bridgeAddress, t)

				depositHandler := depositHandlers.NewETHDepositHandler(bridgeContract)
				for _, handler := range config.Handlers {
					switch handler.Type {
					case "erc20", "native":
						{
							depositHandler.RegisterDepositHandler(handler.Address, &depositHandlers.Erc20DepositHandler{})
						}
					case "permissionlessGeneric":
						{
							depositHandler.RegisterDepositHandler(handler.Address, &depositHandlers.PermissionlessGenericDepositHandler{})
						}
					case "erc721":
						{
							depositHandler.RegisterDepositHandler(handler.Address, &depositHandlers.Erc721DepositHandler{})
						}
					case "erc1155":
						{
							depositHandler.RegisterDepositHandler(handler.Address, &depositHandlers.Erc1155DepositHandler{})
						}
					}
				}
				depositListener := events.NewListener(client)
				tssListener := events.NewListener(client)
				eventHandlers := make([]listener.EventHandler, 0)
				l := log.With().Str("chain", fmt.Sprintf("%v", config.GeneralChainConfig.Name)).Uint8("domainID", *config.GeneralChainConfig.Id)

				depositEventHandler := evmEventHandlers.NewDepositEventHandler(depositListener, depositHandler, bridgeAddress, *config.GeneralChainConfig.Id, msgChan)
				eventHandlers = append(eventHandlers, depositEventHandler)
				eventHandlers = append(eventHandlers, evmEventHandlers.NewKeygenEventHandler(l, tssListener, coordinator, host, communication, keyshareStore, bridgeAddress, networkTopology.Threshold))
				eventHandlers = append(eventHandlers, evmEventHandlers.NewFrostKeygenEventHandler(l, tssListener, coordinator, host, communication, frostKeyshareStore, frostAddress, networkTopology.Threshold))
				eventHandlers = append(eventHandlers, evmEventHandlers.NewRefreshEventHandler(l, topologyProvider, topologyStore, tssListener, coordinator, host, communication, connectionGate, keyshareStore, frostKeyshareStore, bridgeAddress))
				eventHandlers = append(eventHandlers, evmEventHandlers.NewRetryV1EventHandler(l, tssListener, depositHandler, propStore, bridgeAddress, *config.GeneralChainConfig.Id, config.BlockConfirmations, msgChan))
				if config.Retry != "" {
					eventHandlers = append(eventHandlers, evmEventHandlers.NewRetryV2EventHandler(l, tssListener, common.HexToAddress(config.Retry), *config.GeneralChainConfig.Id, msgChan))
				}
				evmListener := listener.NewEVMListener(client, eventHandlers, blockstore, sygmaMetrics, *config.GeneralChainConfig.Id, config.BlockRetryInterval, config.BlockConfirmations, config.BlockInterval)

				mh := message.NewMessageHandler()
				mh.RegisterMessageHandler(retry.RetryMessageType, executor.NewRetryMessageHandler(depositEventHandler, client, propStore, config.BlockConfirmations, msgChan))
				mh.RegisterMessageHandler(transfer.TransferMessageType, &executor.TransferMessageHandler{})
				executor := executor.NewExecutor(host, communication, coordinator, bridgeContract, keyshareStore, exitLock, config.GasLimit.Uint64(), config.TransferGas)

				startBlock, err := blockstore.GetStartBlock(*config.GeneralChainConfig.Id, config.StartBlock, config.GeneralChainConfig.LatestBlock, config.GeneralChainConfig.FreshStart)
				if err != nil {
					panic(err)
				}
				if startBlock == nil {
					head, err := client.LatestBlock()
					if err != nil {
						panic(err)
					}
					startBlock = head
				}
				startBlock, err = chains.CalculateStartingBlock(startBlock, config.BlockInterval)
				if err != nil {
					panic(err)
				}
				chain := coreEvm.NewEVMChain(evmListener, mh, executor, *config.GeneralChainConfig.Id, startBlock)

				domains[*config.GeneralChainConfig.Id] = chain
			}
		case "substrate":
			{
				config, err := substrate.NewSubstrateConfig(chainConfig)
				if err != nil {
					panic(err)
				}

				conn, err := connection.NewSubstrateConnection(config.GeneralChainConfig.Endpoint)
				if err != nil {
					panic(err)
				}
				keyPair, err := signature.KeyringPairFromSecret(config.GeneralChainConfig.Key, config.SubstrateNetwork)
				if err != nil {
					panic(err)
				}

				substrateClient := substrateClient.NewSubstrateClient(conn, &keyPair, config.ChainID, config.Tip)
				bridgePallet := substratePallet.NewPallet(substrateClient)

				log.Info().Str("domain", config.String()).Msgf("Registering substrate domain")

				l := log.With().Str("chain", fmt.Sprintf("%v", config.GeneralChainConfig.Name)).Uint8("domainID", *config.GeneralChainConfig.Id)
				depositHandler := substrateListener.NewSubstrateDepositHandler()
				depositHandler.RegisterDepositHandler(transfer.FungibleTransfer, substrateListener.FungibleTransferHandler)
				eventHandlers := make([]coreSubstrateListener.EventHandler, 0)
				depositEventHandler := substrateListener.NewFungibleTransferEventHandler(l, *config.GeneralChainConfig.Id, depositHandler, msgChan, conn)
				eventHandlers = append(eventHandlers, substrateListener.NewRetryEventHandler(l, conn, depositHandler, *config.GeneralChainConfig.Id, msgChan))
				eventHandlers = append(eventHandlers, depositEventHandler)
				substrateListener := coreSubstrateListener.NewSubstrateListener(conn, eventHandlers, blockstore, sygmaMetrics, *config.GeneralChainConfig.Id, config.BlockRetryInterval, config.BlockInterval)

				mh := message.NewMessageHandler()
				mh.RegisterMessageHandler(transfer.TransferMessageType, &substrateExecutor.SubstrateMessageHandler{})
				mh.RegisterMessageHandler(retry.RetryMessageType, substrateExecutor.NewRetryMessageHandler(depositEventHandler, conn, propStore, msgChan))

				sExecutor := substrateExecutor.NewExecutor(host, communication, coordinator, bridgePallet, keyshareStore, conn, exitLock)

				startBlock, err := blockstore.GetStartBlock(*config.GeneralChainConfig.Id, config.StartBlock, config.GeneralChainConfig.LatestBlock, config.GeneralChainConfig.FreshStart)
				if err != nil {
					panic(err)
				}
				if startBlock == nil {
					head, err := substrateClient.LatestBlock()
					if err != nil {
						panic(err)
					}
					startBlock = head
				}
				startBlock, err = chains.CalculateStartingBlock(startBlock, config.BlockInterval)
				if err != nil {
					panic(err)
				}
				substrateChain := coreSubstrate.NewSubstrateChain(substrateListener, mh, sExecutor, *config.GeneralChainConfig.Id, startBlock)

				domains[*config.GeneralChainConfig.Id] = substrateChain
			}
		case "btc":
			{
				log.Info().Msgf("Registering btc domain")
				config, err := btcConfig.NewBtcConfig(chainConfig)
				if err != nil {
					panic(err)
				}

				conn, err := btcConnection.NewBtcConnection(
					config.GeneralChainConfig.Endpoint,
					config.Username,
					config.Password,
					false)
				if err != nil {
					panic(err)
				}

				l := log.With().Str("chain", fmt.Sprintf("%v", config.GeneralChainConfig.Name)).Uint8("domainID", *config.GeneralChainConfig.Id)
				resources := make(map[[32]byte]btcConfig.Resource)
				for _, resource := range config.Resources {
					resources[resource.ResourceID] = resource
				}
				depositHandler := &btcListener.BtcDepositHandler{}
				depositEventHandler := btcListener.NewFungibleTransferEventHandler(l, *config.GeneralChainConfig.Id, depositHandler, msgChan, conn, resources, config.FeeAddress)
				eventHandlers := make([]btcListener.EventHandler, 0)
				eventHandlers = append(eventHandlers, depositEventHandler)
				listener := btcListener.NewBtcListener(conn, eventHandlers, config, blockstore)

				mempool := mempool.NewMempoolAPI(config.MempoolUrl)
				mh := message.NewMessageHandler()
				mh.RegisterMessageHandler(transfer.TransferMessageType, &btcExecutor.FungibleMessageHandler{})
				mh.RegisterMessageHandler(retry.RetryMessageType, btcExecutor.NewRetryMessageHandler(depositEventHandler, conn, config.BlockConfirmations, propStore, msgChan))
				uploader := uploader.NewIPFSUploader(configuration.RelayerConfig.UploaderConfig)

				executor := btcExecutor.NewExecutor(
					propStore,
					host,
					communication,
					coordinator,
					frostKeyshareStore,
					conn,
					mempool,
					resources,
					config.Network,
					exitLock,
					uploader)

				btcChain := btc.NewBtcChain(listener, executor, mh, *config.GeneralChainConfig.Id)
				domains[*config.GeneralChainConfig.Id] = btcChain

			}
		default:
			panic(fmt.Errorf("type '%s' not recognized", chainConfig["type"]))
		}
	}

	go jobs.StartCommunicationHealthCheckJob(host, configuration.RelayerConfig.MpcConfig.CommHealthCheckInterval, sygmaMetrics)

	r := relayer.NewRelayer(domains, sygmaMetrics)
	go r.Start(ctx, msgChan)

	sysErr := make(chan os.Signal, 1)
	signal.Notify(sysErr,
		syscall.SIGTERM,
		syscall.SIGINT,
		syscall.SIGHUP,
		syscall.SIGQUIT)

	relayerName := viper.GetString("name")
	log.Info().Msgf("Started relayer: %s with PID: %s. Version: v%s", relayerName, host.ID().Pretty(), Version)

	_, err = keyshareStore.GetKeyshare()
	if err != nil {
		log.Info().Msg("Relayer not part of MPC. Waiting for refresh event...")
	}

	sig := <-sysErr
	log.Info().Msgf("terminating got ` [%v] signal", sig)
	return nil

}

func panicOnError(err error) {
	if err != nil {
		panic(err)
	}
}
