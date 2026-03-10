# Sygma bridge

<a href="https://golang.org">
<img alt="go" src="https://img.shields.io/badge/Go-00ADD8?style=for-the-badge&logo=go&logoColor=white" />
</a>

&nbsp;

Sygma bridge uses [sygma-core](https://github.com/sygmaprotocol/sygma-core) framework 

&nbsp;

### Table of Contents

1. [Local environment](#local-environment)
2. [Configuration](#configuration)
3. [Technical documentation](#technical-documentation)

&nbsp;

## Local environment
Run `make example` to start the local environment.

_This will start two EVM networks with configured smart contracts, and start three preconfigured relayers._

&nbsp;

## Configuration

Configuration can be provided either as a configuration file or as ENV variables, depending on `--config` flag value.
If it is set to `env`, configuration properties are expected as ENV variables. Otherwise, you need to provide a path to the configuration file.

Configuration consists of two distinct parts. The first one is `RelayerConfig`, defining various parameters of the relayer.
The second one is a list of `ChainConfig` that defines parameters for each chain/domain that the relayer is processing.

### Configuration file

An example of a JSON configuration file can be found inside `/example/cfg` folder.

### ENV variables

_Each ENV variable needs to be prefixed with SYG._

Properties of `RelayerConfig` are expected to be defined as separate ENV variables
where ENV variable name reflects properties position in the structure.

For example, if you want to set `Config.RelayerConfig.MpcConfig.Port` this would
translate to ENV variable named `SYG_RELAYER_MPCCONFIG_PORT`.

`ChainConfig` is defined as one ENV variable `SYG_CHAINS`, where its content is JSON configuration for all supported chains and should match
ordering with shared configuration.

## Technical documentation
Each service has a technical documentation inside its repository under `/docs` directory. [Here](/docs/Home.md) you can find technical documentation for relayers.

Additionally, the [general high-level documentation for the entire Sygma system](/docs/general/Arhitecture.md) can be found in this same repository under the `/docs/general` directory.
