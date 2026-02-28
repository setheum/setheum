---
title: Mainnet
description: The following details a list of resources to support the Mainnet environment.
---

import SupportedDomains from '../../../src/components/SupportedDomains';
import { Environment } from '@buildwithsygma/sygma-sdk-core';

:::tip Status
| Verification Mechanism | Status                |
|------------------------|-----------------------|
| MPC Relay              | **🟢 Active**         |
| Spectre                | **🔴 Inactive** |
| Zipline                | **🟠 In Development** |
:::

The following section details Sygma's mainnet deployment, including the protocol's:
- [Supported networks](#supported-networks)
- [EVM contract addresses](#evm-contract-addresses)
    - [Ethereum Mainnet](#ethereum-mainnet)
    - [Khala](#khala)
    - [Phala](#phala)
    - [Cronos](#cronos)
    - [Base](#base)
    - [Gnosis](#gnosis)
    - [Polygon](#polygon)
- [Registered Resources](#registered-resources)
- [Registered Routes And Associated Fee Schemes](#registered-routes-and-associated-fee-schemes)
- [Sygma Explorer](#sygma-explorer)

You will need to work with these environment values if you are working with the [Sygma SDK](../../03-sygma-sdk/02-Quick-Start/01-installing-the-sdk.md).

## Supported networks

<SupportedDomains environment={Environment.MAINNET} />

## EVM contract addresses

#### Ethereum Mainnet

| Contract                           | Address                                    |
| ---------------------------------- | ------------------------------------------ |
| Bridge                             | [0x4D878E8Fb90178588Cda4cf1DCcdC9a6d2757089](https://etherscan.io/address/0x4D878E8Fb90178588Cda4cf1DCcdC9a6d2757089) |
| Fee Router                         | [0x1d34808907607FA82Fa1b51F5fBA5Ff5a3Fa90cF](https://etherscan.io/address/0x1d34808907607FA82Fa1b51F5fBA5Ff5a3Fa90cF) |
| Fixed Fee Handler (All routes)     | [0x9f9778DA7c1D0AbE148314d6C1EA6E0A93C151C7](https://etherscan.io/address/0x9f9778DA7c1D0AbE148314d6C1EA6E0A93C151C7) |
| ERC-20 Handler                     | [0xC832588193cd5ED2185daDA4A531e0B26eC5B830](https://etherscan.io/address/0xC832588193cd5ED2185daDA4A531e0B26eC5B830) |
| Permissionless Generic Handler     | [0x31282123E7bcd947e2c1Bc364d564839574fAdCD](https://etherscan.io/address/0x31282123E7bcd947e2c1Bc364d564839574fAdCD) |

#### Khala

| Pallet                 | Address                                          |
|------------------------|--------------------------------------------------|
| Token Reserved Account | 436H4jatj6ntHTVm3wh9zs1Mqa8p1ykfcdkNH7txmjmohTu3 |
| ERC-20 Handler         | 5EYCAe5jLbHcAAMKvLFSXgCTbPrLgBJusvPwfKcaKzuf5X5e |

#### Phala

| Pallet                 | Address                                          |
|------------------------|--------------------------------------------------|
| Token Reserved Account | 436H4jatj6ntHTVm3wh9zs1Mqa8p1ykfcdkNH7txmjmohTu3 |
| ERC-20 Handler         | 5EYCAe5jLbHcAAMKvLFSXgCTbPrLgBJusvPwfKcaKzuf5X5e |

#### Cronos

| Contract                           | Address                                    |
| ---------------------------------- | ------------------------------------------ |
| Bridge                             | [0x44d1Ae962945c5B168282D5002705dE7A9B84657](https://cronoscan.com/address/0x44d1Ae962945c5B168282D5002705dE7A9B84657) |
| Fee Router                         | [0xb18fEa28C8C9557aB65b2808c7b323A586687740](https://cronoscan.com/address/0xb18fEa28C8C9557aB65b2808c7b323A586687740) |
| Fixed Fee Handler (Cronos->ETH)    | [0x8cb45Fa8B8eAC078f901498886a7655605a3d15a](https://cronoscan.com/address/0x8cb45Fa8B8eAC078f901498886a7655605a3d15a) |
| Fixed Fee Handler (Cronos->Base)   | [0x1dac945C9E418D9D07278F2d36e310194BaaB24c](https://cronoscan.com/address/0x1dac945C9E418D9D07278F2d36e310194BaaB24c) |
| ERC-20 Handler                     | [0x13572649779c8e88bcbbF46E38d6AddaFa6Ba4f1](https://cronoscan.com/address/0x13572649779c8e88bcbbF46E38d6AddaFa6Ba4f1) |
| Permissionless Generic Handler     | [0xB86bAe6A570a52cBc38Cf6Ac6557F169422cDf30](https://cronoscan.com/address/0xB86bAe6A570a52cBc38Cf6Ac6557F169422cDf30) |
 
#### Base

| Contract                           | Address                                    |
| ---------------------------------- | ------------------------------------------ |
| Bridge                             | [0xC47468aeae431f5D0B7DA50F9f5D8a6c0eca4789](https://basescan.org/address/0xC47468aeae431f5D0B7DA50F9f5D8a6c0eca4789) |
| Fee Router                         | [0x5573Ae978A10B724705624C620E6a7977935c721](https://basescan.org/address/0x5573Ae978A10B724705624C620E6a7977935c721) |
| Fixed Fee Handler (Base->ETH)      | [0x4EE82A64Aa9535AE4aABe3B35a12c29a0430A951](https://basescan.org/address/0x4EE82A64Aa9535AE4aABe3B35a12c29a0430A951) |
| Fixed Fee Handler (Base->Cronos)   | [0x73aB792E6AdF236879a0A2AB1597fa221E736a0d](https://basescan.org/address/0x73aB792E6AdF236879a0A2AB1597fa221E736a0d) |
| ERC-20 Handler                     | [0xe43F8245249d7fAF46408723Ab36D071dD85D7BB](https://basescan.org/address/0xe43F8245249d7fAF46408723Ab36D071dD85D7BB) |
| Permissionless Generic Handler     | [0x2e1eE4153ad2F763ab8C612415AcF0DEe02Bc79B](https://basescan.org/address/0x2e1eE4153ad2F763ab8C612415AcF0DEe02Bc79B) |

#### Gnosis

| Contract                           | Address                                    |
| ---------------------------------- | ------------------------------------------ |
| Bridge                             | [0x5FdC38c9b32909CcfF4629ffBFe5394852584C59](https://gnosisscan.io/address/0x5FdC38c9b32909CcfF4629ffBFe5394852584C59) |
| Fee Router                         | [0xbb2bf03eD554571EFe9BEE2b46dFdc7e44c157e4](https://gnosisscan.io/address/0xbb2bf03eD554571EFe9BEE2b46dFdc7e44c157e4) |
| Fixed Fee Handler (Gnosis->ETH)    | [0xDAC861f54368cD917f6dbEb93FF90727B2c37cB2](https://gnosisscan.io/address/0xDAC861f54368cD917f6dbEb93FF90727B2c37cB2) |
| ERC-20 Handler                     | [0x89b835B4b01E29C9464860189a394297913fD65B](https://gnosisscan.io/address/0x89b835B4b01E29C9464860189a394297913fD65B) |
| Permissionless Generic Handler     | [0xde57DEfEe28F0F59C5Ad3B7116B3E98d257f6f27](https://gnosisscan.io/address/0xde57DEfEe28F0F59C5Ad3B7116B3E98d257f6f27) |

#### Polygon

| Contract                           | Address                                    |
| ---------------------------------- | ------------------------------------------ |
| Bridge                             | [0x73aB792E6AdF236879a0A2AB1597fa221E736a0d](https://polygonscan.com/address/0x73aB792E6AdF236879a0A2AB1597fa221E736a0d) |
| Fee Router                         | [0x11947a868b304898e51E50371b84a34D278026e5](https://polygonscan.com/address/0x11947a868b304898e51E50371b84a34D278026e5) |
| Fixed Fee Handler (All routes)     | [0x823055b65894203e41EA35BaAC0E1862a9C07E6D](https://polygonscan.com/address/0x823055b65894203e41EA35BaAC0E1862a9C07E6D) |
| ERC-20 Handler                     | [0x3eE20f17BC7D07bf3e06a7342C13A29823C22Ad5](https://polygonscan.com/address/0x3eE20f17BC7D07bf3e06a7342C13A29823C22Ad5) |
| Permissionless Generic Handler     | [0x96eb8544Dd96aF28EeBa9b86a1De6357DAb519F6](https://polygonscan.com/address/0x96eb8544Dd96aF28EeBa9b86a1De6357DAb519F6) |

## Registered Resources

**Phala**

| Property                          | Value                                                                                                                 |
|-----------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| Symbol                            | PHA                                                                                                                   |
| Type                              | Fungible                                                                                                              |
| Fee strategy                      | Fixed Fee                                                                                                             |
| Registered Handler                | ERC20Handler                                                                                                          |
| Bridging Strategy                 | Lock/Release                                                                                                          |
| Resource ID                       | 0x0000000000000000000000000000000000000000000000000000000000000001                                                    |
| Ethereum Mainnet Contract Address | [0x6c5bA91642F10282b576d91922Ae6448C9d52f4E](https://etherscan.io/address/0x6c5bA91642F10282b576d91922Ae6448C9d52f4E) |
| Substrate Token Reserved Account  | 436H4jatj6ntHTVm3wh9zs1Mqa8p1ykfcdkNH7txmjmohTu3                                                                      |

**Permissionless generic message**

| Details            | Information                                                        |
| ------------------ | ------------------------------------------------------------------ |
| Type               | Generic                                                            |
| Registered Handler | PermissionlessGenericHandler                                       |
| Bridging Strategy  | GMP                                                                |
| Resource ID        | 0x0000000000000000000000000000000000000000000000000000000000000000 |

## Registered Routes And Associated Fee Schemes

| Source Network Name | Destination Network Name | Resource | Fee Percent/Amount | Resource ID                                                        |
| ------------------- | ------------------------ | -------- | ------------------ | ------------------------------------------------------------------ |
| Ethereum Mainnet    | Phala                    | PHA      | 0.0001 ETH         | 0x0000000000000000000000000000000000000000000000000000000000000001 |
| Ethereum Mainnet    | Khala                    | PHA      | 0.0001 ETH         | 0x0000000000000000000000000000000000000000000000000000000000000001 |
| Khala               | Ethereum Mainnet         | PHA      | 50 PHA             | 0x0000000000000000000000000000000000000000000000000000000000000001 |
| Phala               | Ethereum Mainnet         | PHA      | 50 PHA             | 0x0000000000000000000000000000000000000000000000000000000000000001 |
| Ethereum            | Base                     | GMP      | 0.0008 ETH         | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Ethereum            | Cronos                   | GMP      | 0.0004 ETH         | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Cronos              | Ethereum                 | GMP      | 200 CRO            | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Cronos              | Base                     | GMP      | 25 CRO             | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Base                | Ethereum                 | GMP      | 0.007 ETH          | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Base                | Cronos                   | GMP      | 0.0004 ETH         | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Ethereum            | Gnosis                   | GMP      | 0.0004 ETH         | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Gnosis              | Ethereum                 | GMP      | 15 xDAI            | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Polygon             | Ethereum                 | GMP      | 20 MATIC           | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Polygon             | Base                     | GMP      | 2 MATIC            | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Polygon             | Cronos                   | GMP      | 1 MATIC            | 0x0000000000000000000000000000000000000000000000000000000000000000 |
| Polygon             | Gnosis                   | GMP      | 1 MATIC            | 0x0000000000000000000000000000000000000000000000000000000000000000 |

## Sygma Explorer

The [explorer UI](https://scan.buildwithsygma.com/) provides users with a cross-chain block explorer that scans for mainnet transactions through the Sygma protocol. 
