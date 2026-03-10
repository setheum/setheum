# Fees

Sygma allows granular control of handling fees for each resource. Although specific implementations differ based on chain architecture, the general functionality is the same. Each resource is assigned a fee strategy for every potential destination domain, and this mapping also outlines all potential bridging routes for a given resource.

### Deposit flow

1. **Calculate the final fee**
   - Call the contract to calculate the fee. This call, under the hood, fetches information on price from the on-chain oracle (for dynamic fee strategy) or uses predefined rules for fixed and percentage-based strategies.
2. **Execute deposit**
   - Send the appropriate amount based on the calculated final fee when executing the deposit.

## Fee strategies

Fee strategy defines a set of rules on how fees should be charged when executing deposits on the source chain.

### Dynamic fee strategy (EVM only)
_Supported on: **EVM**_ | _Fee can be paid in:_ **Native asset**

This strategy utilizes the on-chain `TwapOracle` contract to pull price information and calculate fees. The final fee is calculated based on the execution cost for the destination chain and can be paid in the native currency of the source chain.

Calculated fee consists out of two parts:
`fee = execution cost + protocol fee`
- **execution cost**: price of executing destination transaction
   - `destination_network_gasprice * gas_used * destination_coin_price_from_twap`
  

- **protocol fee**: fee that the protocol takes for maintenance costs
   - _fixed:_ constant amount of native tokens added on top of the execution cost
   - _percentage:_ percentage of the execution cost added on top of the execution cost
   - _** it is possible for some routes for protocol fee to be 0_

Based on route type, there are two concrete implementations of dynamic fees fee handlers:

- **For asset transfers**: `TwapNativeTokenFeeHandler.sol`
  - Parameter `gas_used` is fixed and configured in the contract (generally, transactions transferring specific type of asset use a fixed amount of gas)


- **For GMP (generic message) transfers**: `TwapGenericFeeHandler.sol`
  - Parameter `gas_used` is provided by the user with deposit data. This value will be set by the relayer executing this transaction, so the user must understand what they are executing.

#### TwapOracle
The TwapOracle is a wrapper contract around the Uniswap TWAP oracle system. Based on the configuration, this contract will either pull the price of the destination gas token periodically from Uniswap or use the configured fixed price of the destination gas token.

### Fixed fee strategy
_Supported on: **EVM**, **Substrate**_ | _Fee can be paid in:_ **Native asset**

This strategy always requires a predefined fixed fee amount per deposit. **EVM implementation can only collect fees in the native currency of the source chain, while Substrate implementation allows for fees to be collected in any configured asset**.

### Percentage-based fee strategy
_Supported on: **EVM**, **Substrate**_ | _Fee can be paid in:_ **Transferred asset**

This strategy calculates the fee amount based on the amount of token being transferred. It always collects fees in the token that is being transferred, so it only makes sense for fungible token routes.

<img src="/docs/resources/percentage-formula-general.png" data-canonical-src="/docs/resources/percentage-formula-general.png" width="386" height="267" />

## Architecture overview
*On the diagram below, we use [Sygma SDK](https://github.com/sygmaprotocol/sygma-sdk) for interaction with all services.*

![](/docs/resources/fees-diagram.png)

### EVM

Check out the [solidity documentation](https://github.com/sygmaprotocol/sygma-solidity/blob/master/docs/Home.md) for details on EVM implementation.

### Substrate

Check out the [substrate pallet documentation](https://github.com/sygmaprotocol/sygma-substrate-pallets/blob/master/docs/Home.md) for details on Substrate implementation.
