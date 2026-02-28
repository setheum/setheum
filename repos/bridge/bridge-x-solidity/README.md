<p align="center"><a href="https://buildwithsygma.com"><img width="250" title="Sygma solidity" src='assets/full-logo.png'/></a></p>

# sygma-x-solidity

Sygma uses Solidity smart contracts to enable transfers to and from EVM compatible chains. These contracts consist of a core bridge contract (Bridge.sol) and a set of handler contracts (ERC20Handler.sol, PermissionlessGenericHandler.sol). The bridge contract is responsible for initiating and executing proposed transfers. The handlers are used by the bridge contract to interact with other existing contracts.

## Deployments

To deploy contracts run `npx hardhat run --network NETWORK_NAME scripts/deploy.ts`.

For more details check out the [official hardhat documentation](https://hardhat.org/hardhat-runner/docs/guides/deploying).

To add another network do the following:
 * update `hardhat.config.ts` with the desired configuration

## Dependencies

Requires `yarn` and `@nomicfoundation/hardhat`.

## Commands
  * hardhat commands: <br>
    `check` - Check whatever you need <br>
    `clean` - Clears the cache and deletes all artifacts <br>
    `compile` - Compiles the entire project, building all artifacts <br>
    `console` - Opens a hardhat console <br>
    `coverage` - Generates a code coverage report for tests <br>
    `flatten` - Flattens and prints contracts and their dependencies <br>
    `help` - Prints this message <br>
    `node` - Starts a JSON-RPC server on top of Hardhat Network <br>
    `run` - Runs a user-defined script after compiling the project <br>
    `test` - Runs mocha tests <br>
    `typechain` - Generate Typechain typings for compiled contracts <br>
    `verify` - Verifies contract on Etherscan <br>
  * custom commands: <br>
    `yarn run test`: Runs tests.

# Sygma Security Policy

## Reporting a Security Bug

We take all security issues seriously, if you believe you have found a security issue within a Sygma
project please notify us immediately. If an issue is confirmed, we will take all necessary precautions
to ensure a statement and patch release is made in a timely manner.

Please email us a description of the flaw and any related information (e.g. reproduction steps, version) to
[dev@buildwithsygma.com](mailto:dev@buildwithsygma.com).
