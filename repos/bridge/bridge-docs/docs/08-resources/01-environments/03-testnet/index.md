---
title: Testnet
id: testnet-index
description: The following details a list of resources that support the Testnet environment.
---

import SupportedDomains from '../../../../src/components/SupportedDomains';
import { Environment } from '@buildwithsygma/sygma-sdk-core';

:::tip Status
| Verification Mechanism | Status                |
|------------------------|-----------------------|
| MPC Relay              | **🟢 Active**         |
| Spectre                | **🟢 Active**         |
| Zipline                | **🟠 In Development** |
:::

The following section details Sygma's testnet deployments, including both MPC and Spectre (zk) verification systems:
- [Testnet Transfer UI](#testnet-transfer-ui)
- [Testnet Faucet](#testnet-faucet)
- [Supported Networks](#supported-networks)
- [EVM Contract Addresses](#evm-contract-addresses)
    - [Sepolia](#sepolia)
    - [Cronos Testnet](#cronos-testnet)
    - [Holesky](#holesky)
    - [Arbitrum Sepolia](#arbitrum-sepolia)
    - [Gnosis Chiado](#gnosis-chiado)
    - [Base Sepolia](#base-sepolia)
    - [Amoy](#amoy)
    - [B3 Sepolia](#b3-sepolia)
- [Registered Resources](#registered-resources)
- [Fee Schemes](#fee-schemes)
- [Sygma Explorer](#sygma-explorer)

You will need to work with these environment values if you are working with the [Sygma SDK](../../../03-sygma-sdk/02-Quick-Start/01-installing-the-sdk.md).

## Testnet Transfer UI

The [transfer UI](https://sygma-react-widget.pages.dev/) provides users with a visual interface to connect their wallets and bridge tokens.

## Testnet Faucet

The [faucet UI](./01-obtain-testnet-tokens.md "mention") provides users with a visual interface to mint testnet tokens.

## Supported Networks

<SupportedDomains environment={Environment.TESTNET} />


## EVM Contract Addresses

#### Sepolia

| Contract                                       | MPC Address                                                                                                                        | Spectre Address                                                                                                               |
|------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| Bridge                                         | [0x4CF326d3817558038D1DEF9e76b727202c3E8492](https://sepolia.etherscan.io/address/0x4CF326d3817558038D1DEF9e76b727202c3E8492)      | [0x60e3fd2148f3D6934521a00a0Cf318cB1194e072](https://sepolia.etherscan.io/address/0x60e3fd2148f3D6934521a00a0Cf318cB1194e072) |
| Fee Router                                     | [0xD277478b4684Ed8594d5eb5B228AA7aDbA59df43](https://sepolia.etherscan.io/address/0xD277478b4684Ed8594d5eb5B228AA7aDbA59df43)      | [0xC2C26789671ec5e1542308Ee38faE4A640bCc03e](https://sepolia.etherscan.io/address/0xC2C26789671ec5e1542308Ee38faE4A640bCc03e) |
| Fixed Fee Handler                              | [0x356B7B3C25355325CcBFBCF00a82895F93f086b7](https://sepolia.etherscan.io/address/0x356B7B3C25355325CcBFBCF00a82895F93f086b7)      | [0xEA3a9Bcc6de765aC6E01e327D22DB53b8d3DFa0C](https://sepolia.etherscan.io/address/0xEA3a9Bcc6de765aC6E01e327D22DB53b8d3DFa0C) |
| Percentage Fee Handler                         | [0xDDa14370765696cD64bA93b974a934FF18A278e2](https://sepolia.etherscan.io/address/0xDDa14370765696cD64bA93b974a934FF18A278e2)      | [0xA048Ddc2C953052835e7847d8c3C4b19183AE1C0](https://sepolia.etherscan.io/address/0xA048Ddc2C953052835e7847d8c3C4b19183AE1C0) |
| Dynamic Native Token Fee Handler               | [0x6a5c39db255D55F77ccA590418EB4175Ec02Ea37](https://sepolia.etherscan.io/address/0x6a5c39db255D55F77ccA590418EB4175Ec02Ea37)      |
| Dynamic GMP Fee Handler                        | [0x52C09318436d729c212B582A0b2Aba2952da0a05](https://sepolia.etherscan.io/address/0x52C09318436d729c212B582A0b2Aba2952da0a05)      |
| ERC-20 Handler                                 | [0xa65387feCb172ffF8A0aabA323A10c63757BBFA6](https://sepolia.etherscan.io/address/0xa65387feCb172ffF8A0aabA323A10c63757BBFA6)      | [0x65Ce12864941F56D3665bb0d97D803E81a1d09a0](https://sepolia.etherscan.io/address/0x65Ce12864941F56D3665bb0d97D803E81a1d09a0) |
| ERC-721 Handler                                | [0x669F52487ffA6f9aBf722082f735537A98Ec0E4b](https://sepolia.etherscan.io/address/0x669F52487ffA6f9aBf722082f735537A98Ec0E4b)      |                                                                                                                               |
| ERC-1155 Handler                               | [0x65903772866e538e6ffc001dd0c7665e356eb6d8](https://sepolia.etherscan.io/address/0x65903772866e538e6ffc001dd0c7665e356eb6d8)      |                                                                                                                               |
| Permissionless Generic Handler                 | [0x7dCBdb9cBA0Bb1871EECafAB290E5a2e45077479](https://sepolia.etherscan.io/address/0x7dCBdb9cBA0Bb1871EECafAB290E5a2e45077479)      | [0x57d66F8A5A57942b25368C837005141e8A586155](https://sepolia.etherscan.io/address/0x57d66F8A5A57942b25368C837005141e8A586155) |
| Storage (GMP testing contract)                 | [0x10791B617D2Dad4978Cc18E3A88e422310428430](https://sepolia.etherscan.io/address/0x10791b617d2dad4978cc18e3a88e422310428430)      |                                                                                                                               |
| Router                                         |                                                                                                                                    | [0x57a3aC22bb0779A91B334027982cbC58a71aaD32](https://sepolia.etherscan.io/address/0x57a3aC22bb0779A91B334027982cbC58a71aaD32) |
| Executor                                       |                                                                                                                                    | [0x108E42c97a57A7FFC3fbB80A8A14129a94cD29C7](https://sepolia.etherscan.io/address/0x108E42c97a57A7FFC3fbB80A8A14129a94cD29C7) |
| Control Segregator                             |                                                                                                                                    | [0x2Bdfa87E8caaaB11b8b510C83899Cb18F1D2d4Fd](https://sepolia.etherscan.io/address/0x2Bdfa87E8caaaB11b8b510C83899Cb18F1D2d4Fd) |
| Spectre Proxy                                  |                                                                                                                                    | [0xC4c2722e8E35fe95C49036eb0d2Ed15e48341061](https://sepolia.etherscan.io/address/0xC4c2722e8E35fe95C49036eb0d2Ed15e48341061) |
| Spectre                                        |                                                                                                                                    | [0xEf7d892E8F5177ED9C8eF140E63948685B15E380](https://sepolia.etherscan.io/address/0xEf7d892E8F5177ED9C8eF140E63948685B15E380) |
| ERC-20* Handler (with contract call)           | [0x0d4fB069753bdf1C5aB48302e9744BF222A9F4e8](https://sepolia.etherscan.io/address/0x0d4fB069753bdf1C5aB48302e9744BF222A9F4e8#code) |                                                                                                                               |
| Dynamic ERC-20* Native Fee Handler             | [0x0e5F28d6874938CF1504Fe65Fa491E1BA7891B6A](https://sepolia.etherscan.io/address/0x0e5F28d6874938CF1504Fe65Fa491E1BA7891B6A)      |                                                                                                                               |
| DefaultMessageReceiver                         | [0xe9cbf1De10498b8E5F5e3FD4dd5269a289c65718](https://sepolia.etherscan.io/address/0xe9cbf1De10498b8E5F5e3FD4dd5269a289c65718)      |                                                                                                                               |
| SprinterNameService (ERC-20* testing contract) | [0xf70fb86F700E8Bb7cDf1c20197633518235c3425](https://sepolia.etherscan.io/address/0xf70fb86F700E8Bb7cDf1c20197633518235c3425#code) |                                                                                                                               |

#### Cronos Testnet

| Contract                       | Address                                                                                                                              |
|--------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| Bridge                         | [0x816bb9E810b6b97840F6818bF21Fa25DD7364132](https://explorer.cronos.org/testnet/address/0x816bb9E810b6b97840F6818bF21Fa25DD7364132) |
| Fee Router                     | [0xFaa9214b2bb90222c4f5A2DAbBB61fC945CDFD42](https://explorer.cronos.org/testnet/address/0xFaa9214b2bb90222c4f5A2DAbBB61fC945CDFD42) |
| Fixed Fee Handler              | [0x1afc317C3C8578274B557a022aD79e073ef6EBF7](https://explorer.cronos.org/testnet/address/0x1afc317C3C8578274B557a022aD79e073ef6EBF7) |
| Percentage Fee Handler         | [0xC5244700904c987E5441F568440ed5c3b7233559](https://explorer.cronos.org/testnet/address/0xC5244700904c987E5441F568440ed5c3b7233559) |
| ERC-20 Handler                 | [0x39D1Aea5F01138940F19A15049E2073D4df1dc9E](https://explorer.cronos.org/testnet/address/0x39D1Aea5F01138940F19A15049E2073D4df1dc9E) |
| ERC-721 Handler                | [0x52757D9c1a8f8b496cf1e688bbB8055f9F3DfC8a](https://explorer.cronos.org/testnet/address/0x52757D9c1a8f8b496cf1e688bbB8055f9F3DfC8a) |
| ERC-1155 Handler               | [0x2d92a0aa3ea19da735514ea542438345ef09cb60](https://explorer.cronos.org/testnet/address/0x2d92a0aa3ea19da735514ea542438345ef09cb60) |
| Permissionless Generic Handler | [0x3CBbC542d10CD037cafb1632B29B5B0F59B08A48](https://explorer.cronos.org/testnet/address/0x3CBbC542d10CD037cafb1632B29B5B0F59B08A48) |
| Storage (GMP testing contract) | [0x4b17531F07e002Ee2A0714F79d84d9bEcF6b243D](https://explorer.cronos.org/testnet/address/0x4b17531F07e002Ee2A0714F79d84d9bEcF6b243D) |

#### Holesky

| Contract                       | MPC Address                                                                                                                   | Spectre Address                                                                                                               |
|--------------------------------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| Bridge                         | [0xE366E0B707FBF59CF9A3068af34dC519D5fa6e78](https://holesky.etherscan.io/address/0xE366E0B707FBF59CF9A3068af34dC519D5fa6e78) | [0x3DA376947B760836905a0407588C606D1020C4f3](https://holesky.etherscan.io/address/0x3DA376947B760836905a0407588C606D1020C4f3) |
| Fee Router                     | [0xfc8fE2bC5C780ef8cA28dda09751ef69BdDA53Cb](https://holesky.etherscan.io/address/0xfc8fE2bC5C780ef8cA28dda09751ef69BdDA53Cb) | [0x48512523Bb2634467292dd7776313916425F4c8a](https://holesky.etherscan.io/address/0x48512523Bb2634467292dd7776313916425F4c8a) |
| Fixed Fee Handler              | [0xd6224d509DDa6c5eB3FFd3204218AD8eEC8F59Ef](https://holesky.etherscan.io/address/0xd6224d509DDa6c5eB3FFd3204218AD8eEC8F59Ef) | [0x51b9DcAcB395B00857C815aDb89D20175cBb4A58](https://holesky.etherscan.io/address/0x51b9DcAcB395B00857C815aDb89D20175cBb4A58) |
| Percentage Fee Handler         | [0xa477963806167acB38830Ea91B081a3de47B6ce9](https://holesky.etherscan.io/address/0xa477963806167acB38830Ea91B081a3de47B6ce9) | [0x8fc170823A8dc9eD6Ce81128f0FED3E0089D52f1](https://holesky.etherscan.io/address/0x8fc170823A8dc9eD6Ce81128f0FED3E0089D52f1) |
| ERC-20 Handler                 | [0xEeFBd08769Ab1e369a04a17180E91E4549938d4c](https://holesky.etherscan.io/address/0xEeFBd08769Ab1e369a04a17180E91E4549938d4c) | [0x96e0cBCb653448a6AE76C2f4041eC6D73127585a](https://holesky.etherscan.io/address/0x96e0cBCb653448a6AE76C2f4041eC6D73127585a) |
| ERC-721 Handler                |                                                                                                                               |                                                                                                                               |
| Permissionless Generic Handler | [0xc1154781Fa12a845aCaf276Bf2030040Ba9DAec8](https://holesky.etherscan.io/address/0xc1154781Fa12a845aCaf276Bf2030040Ba9DAec8) | [0x347BfeA6B3E929d91caE8e0AfB5DE97d3574AB45](https://holesky.etherscan.io/address/0x347BfeA6B3E929d91caE8e0AfB5DE97d3574AB45) |
| Storage (GMP testing contract) | [0x5984CA38b38b43d0A9c94BA5a6D6969E92124a15](https://holesky.etherscan.io/address/0x5984CA38b38b43d0A9c94BA5a6D6969E92124a15) |                                                                                                                               |
| Router                         |                                                                                                                               | [0x6Bac21Aa7627009C79cd4784D94e9c280E36ED79](https://holesky.etherscan.io/address/0x6Bac21Aa7627009C79cd4784D94e9c280E36ED79) |
| Executor                       |                                                                                                                               | [0x8Fa58B46F887aB519bB4d7967E718eCe7a92716d](https://holesky.etherscan.io/address/0x8Fa58B46F887aB519bB4d7967E718eCe7a92716d) |
| Control Segregator             |                                                                                                                               | [0xEE6dCE1979a80b6955FBf939403e956CE649a7b5](https://holesky.etherscan.io/address/0xEE6dCE1979a80b6955FBf939403e956CE649a7b5) |
| Spectre Proxy                  |                                                                                                                               | [0x4D083Cb89698C2CA59cDae9428854073781784A3](https://holesky.etherscan.io/address/0x4D083Cb89698C2CA59cDae9428854073781784A3) |
| Spectre                        |                                                                                                                               | [0x220aD39E44A3765a791B33c925B8e76B8a665657](https://holesky.etherscan.io/address/0x220aD39E44A3765a791B33c925B8e76B8a665657) |

#### Arbitrum Sepolia

| Contract                       | Address                                                                                                                      |
|--------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| Bridge                         | [0xFB2530Fb3f5801aD35ccd6fdA29715D9330b7F9f](https://sepolia.arbiscan.io/address/0xFB2530Fb3f5801aD35ccd6fdA29715D9330b7F9f) |
| Fee Router                     | [0x2CD3f725BF9aa446FBD38e2A1A4787a40eEA2286](https://sepolia.arbiscan.io/address/0x2CD3f725BF9aa446FBD38e2A1A4787a40eEA2286) |
| Fixed Fee Handler              | [0x2D7651aF6FFdb486571067568BFefA6e7fe7Fa8a](https://sepolia.arbiscan.io/address/0x2D7651aF6FFdb486571067568BFefA6e7fe7Fa8a) |
| Percentage Fee Handler         | [0x9D5C332Ebe0DaE36e07a4eD552Ad4d8c5067A61F](https://sepolia.arbiscan.io/address/0x9D5C332Ebe0DaE36e07a4eD552Ad4d8c5067A61F) |
| ERC-20 Handler                 | [0x5AF405550De00b38cAC1ED7276d0A09114831bCB](https://sepolia.arbiscan.io/address/0x5AF405550De00b38cAC1ED7276d0A09114831bCB) |
| ERC-721 Handler                |                                                                                                                              |
| Permissionless Generic Handler | [0x5ffB6Dc54221371CcBDb9850A283488e12aDf97D](https://sepolia.arbiscan.io/address/0x5ffB6Dc54221371CcBDb9850A283488e12aDf97D) |
| Storage (GMP testing contract) | [0xD7d5E7d7eaD31E783Df01760FbFad249704Aab14](https://sepolia.arbiscan.io/address/0xD7d5E7d7eaD31E783Df01760FbFad249704Aab14) |

#### Gnosis Chiado

| Contract                       | Address                                                                                                                               |
|--------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| Bridge                         | [0x668fad90DeAd0F8f04346A735875b62eF9c65f0B](https://gnosis-chiado.blockscout.com/address/0x668fad90DeAd0F8f04346A735875b62eF9c65f0B) |
| Fee Router                     | [0x36930B9117713EF92E2f5Ca4FCa047DAdb92C48E](https://gnosis-chiado.blockscout.com/address/0x36930B9117713EF92E2f5Ca4FCa047DAdb92C48E) |
| Fixed Fee Handler              | [0x27284B791992a40D18197087477304368Ac67Fcb](https://gnosis-chiado.blockscout.com/address/0x27284B791992a40D18197087477304368Ac67Fcb) |
| Percentage Fee Handler         | [0x14d1cA88277E13B5615C2C727415a1B94E90264F](https://gnosis-chiado.blockscout.com/address/0x14d1cA88277E13B5615C2C727415a1B94E90264F) |
| ERC-20 Handler                 | [0xb947F89269F0cF54CC721BcDE298a46930f3418b](https://gnosis-chiado.blockscout.com/address/0xb947F89269F0cF54CC721BcDE298a46930f3418b) |
| Permissionless Generic Handler | [0xe4B86b1B07bBdB0C47940b9B3bD4954A0deAdaBE](https://gnosis-chiado.blockscout.com/address/0xe4B86b1B07bBdB0C47940b9B3bD4954A0deAdaBE) |
| Storage (GMP testing contract) | [0x40e273C40349dCA9062F9a3B80BAdFF000512c1F](https://gnosis-chiado.blockscout.com/address/0x40e273C40349dCA9062F9a3B80BAdFF000512c1F) |

#### Base Sepolia

| Contract                                       | Address                                                                                                                            |
|------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
| Bridge                                         | [0x9D5C332Ebe0DaE36e07a4eD552Ad4d8c5067A61F](https://sepolia.basescan.org/address/0x9D5C332Ebe0DaE36e07a4eD552Ad4d8c5067A61F)      |
| Fee Router                                     | [0x241a1617a0C73373110F868C07e38679B1Ec4950](https://sepolia.basescan.org/address/0x241a1617a0C73373110F868C07e38679B1Ec4950)      |
| Fixed Fee Handler                              | [0x52757D9c1a8f8b496cf1e688bbB8055f9F3DfC8a](https://sepolia.basescan.org/address/0x52757D9c1a8f8b496cf1e688bbB8055f9F3DfC8a)      |
| Percentage Fee Handler                         | [0x8b8F51bB26Eb521ac33dD6DcAA97f5d052977746](https://sepolia.basescan.org/address/0x8b8F51bB26Eb521ac33dD6DcAA97f5d052977746)      |
| ERC-20 Handler                                 | [0x72a588B76025d552B239532C31fB7D5Cc80A3e41](https://sepolia.basescan.org/address/0x72a588B76025d552B239532C31fB7D5Cc80A3e41)      |
| Permissionless Generic Handler                 | [0x7f4e1E62A0Abd4A381254335CeF5770F74b3E22E](https://sepolia.basescan.org/address/0x7f4e1E62A0Abd4A381254335CeF5770F74b3E22E)      |
| Permissionless Generic Handler                 | [0xfd69bbfcCbfc832C56Ca1490df48B4baF3DfD376](https://sepolia.basescan.org/address/0xfd69bbfcCbfc832C56Ca1490df48B4baF3DfD376)      |
| Storage (GMP testing contract)                 | [0xF1bFBbE4174E2E6595E095BDF3ac8b97aF7796aA](https://sepolia.basescan.org/address/0xF1bFBbE4174E2E6595E095BDF3ac8b97aF7796aA)      |
| ERC-20* Handler (with contract call)           | [0x3b0f996C474c91de56617da13a52B22BB659D18E](https://sepolia.basescan.org/address/0x3b0f996C474c91de56617da13a52B22BB659D18E#code) |                                                                                                                               |
| Dynamic ERC-20* Native Fee Handler             | [0x4cd7786e926b1f3d5c8ecb338c6ad2abafc44948](https://sepolia.basescan.org/address/0x4cd7786e926b1f3d5c8ecb338c6ad2abafc44948#code) |                                                                                                                               |
| DefaultMessageReceiver                         | [0x0a55ad2e95BFfd8216774c1489aeDef2F92Ae43a](https://sepolia.basescan.org/address/0x0a55ad2e95BFfd8216774c1489aeDef2F92Ae43a)      |                                                                                                                               |
| SprinterNameService (ERC-20* testing contract) | [0x3F9A68fF29B3d86a6928C44dF171A984F6180009](https://sepolia.basescan.org/address/0x3F9A68fF29B3d86a6928C44dF171A984F6180009#code) |                                                                                                                               |

#### Amoy

| Contract                       | Address                                                                                                                               |
|--------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| Bridge                         | [0x09379198A26E98b42E2619D2fCa1f3170c6FBB6e](https://www.oklink.com/amoy/address/0x09379198A26E98b42E2619D2fCa1f3170c6FBB6e/contract) |
| Fee Router                     | [0x04C4B75084B557ab0E79dE3DEd42b6c1dD4F48E6](https://www.oklink.com/amoy/address/0x04C4B75084B557ab0E79dE3DEd42b6c1dD4F48E6/contract) |
| Fixed Fee Handler              | [0x5842071a7D59e56c73874194A1Ea25FcFB6e00D7](https://www.oklink.com/amoy/address/0x5842071a7D59e56c73874194A1Ea25FcFB6e00D7/contract) |
| Percentage Fee Handler         | [0x2E3d3b2535785341584D17840F59dc0370b3504D](https://www.oklink.com/amoy/address/0x2E3d3b2535785341584D17840F59dc0370b3504D/contract) |
| ERC-20 Handler                 | [0x7E462Fc0E0b6f5EC18C8935f2676E7E40e06a7af](https://www.oklink.com/amoy/address/0x7E462Fc0E0b6f5EC18C8935f2676E7E40e06a7af/contract) |
| Permissionless Generic Handler | [0x3080Ecd6dEc3fe8a3A07d40e656f7D5fc6b3c3A7](https://www.oklink.com/amoy/address/0x3080Ecd6dEc3fe8a3A07d40e656f7D5fc6b3c3A7/contract) |
| Permissionless Generic Handler | [0x1F932B2582c52f939DCfb367A3A42f8A95f66782](https://www.oklink.com/amoy/address/0x1F932B2582c52f939DCfb367A3A42f8A95f66782/contract) |
| Storage (GMP testing contract) | [0x2d5395aa622DBC7688B2eEeD3E2dC089aE0fd356](https://www.oklink.com/amoy/address/0x2d5395aa622DBC7688B2eEeD3E2dC089aE0fd356/contract) |

#### B3 Sepolia
| Contract                                       | Address                                                                                                                                       |
|------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| Bridge                                         | [0xFF92C3C393B22F9d26e5732F2601EaC04094880F](https://sepolia.explorer.b3.fun/address/0xFF92C3C393B22F9d26e5732F2601EaC04094880F)              |
| Fee Router                                     | [0xA56419ECdb71acE442a6FbfC8E50c5F993667938](https://sepolia.explorer.b3.fun/address/0xA56419ECdb71acE442a6FbfC8E50c5F993667938)              |
| Fixed Fee Handler                              | [0x8651f04eFA1FccfB9Cd27c6FE3FA0F2E7CB7A75a](https://sepolia.explorer.b3.fun/address/0x8651f04eFA1FccfB9Cd27c6FE3FA0F2E7CB7A75a)              |
| Percentage Fee Handler                         | [0xA6Cd2A4c41946fAADACf88a87A50997B04abE557](https://sepolia.explorer.b3.fun/address/0xA6Cd2A4c41946fAADACf88a87A50997B04abE557)              |
| ERC-20 Handler                                 | [0x36930B9117713EF92E2f5Ca4FCa047DAdb92C48E](https://sepolia.explorer.b3.fun/address/0x36930B9117713EF92E2f5Ca4FCa047DAdb92C48E)              |
| Permissionless Generic Handler                 | [0x41F7869E4E0dd06c0c02804e4afcFA76BEE92CC7](https://sepolia.explorer.b3.fun/address/0x41F7869E4E0dd06c0c02804e4afcFA76BEE92CC7)              |
| Storage (GMP testing contract)                 | [0xF5Ac994A5C402F4f426c2D7319C27912d5DBD7a8](https://sepolia.explorer.b3.fun/address/0xF5Ac994A5C402F4f426c2D7319C27912d5DBD7a8)              |
| ERC-20* Handler (with contract call)           | [0x46b535f52237a9621dce4d506744056b10d2AD7D](https://sepolia.explorer.b3.fun/address/0x46b535f52237a9621dce4d506744056b10d2AD7D?tab=contract) |                                                                                                                               |
| Dynamic ERC-20* Native Fee Handler             | [0xfA520bE56053AFAbd82cE48BeCc5fa277DBda982](https://sepolia.explorer.b3.fun/address/0xfA520bE56053AFAbd82cE48BeCc5fa277DBda982?tab=contract) |                                                                                                                               |
| DefaultMessageReceiver                         | [0x28a033aB855dc4A8459515706fD511689Aa40ce2](https://sepolia.explorer.b3.fun/address/0x28a033aB855dc4A8459515706fD511689Aa40ce2?tab=contract) |                                                                                                                               |
| SprinterNameService (ERC-20* testing contract) | [0x17e4C404aD634E429ebCdF9a10F38A96Ce8eEF27](https://sepolia.explorer.b3.fun/address/0x17e4C404aD634E429ebCdF9a10F38A96Ce8eEF27?tab=contract) |                                                                                                                               |


## Registered Resources

**ERC20LRTest**

| Property                 | Value                                                                                                                                |
|--------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| Symbol                   | ERC20LRTST                                                                                                                           |
| Type                     | ERC-20                                                                                                                               |
| Registered Fee Handler   | BasicFeeHandler                                                                                                                      |
| Registered Handler       | ERC20Handler                                                                                                                         |
| Bridging Strategy        | Lock/Release                                                                                                                         |
| Resource ID              | 0x0000000000000000000000000000000000000000000000000000000000000300                                                                   |
| Sepolia Contract Address | [0x7d58589b6C1Ba455c4060a3563b9a0d447Bef9af](https://sepolia.etherscan.io/address/0x7d58589b6C1Ba455c4060a3563b9a0d447Bef9af)        |
| Cronos Contract Address  | [0x2938ed97ef9d897dac7b21c48e045f34a3a02846](https://explorer.cronos.org/testnet/address/0x2938ed97ef9d897dac7b21c48e045f34a3a02846) |
| Amoy Contract Address    | [0x245466D2175bcED0A1ad1ce804C8F724D7050e85](https://www.oklink.com/amoy/address/0x245466d2175bced0a1ad1ce804c8f724d7050e85)         |


**Phala**

| Details                  | Information                                                                                                                   |
|--------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| Symbol                   | gPHA                                                                                                                          |
| Type                     | ERC-20                                                                                                                        |
| Registered Fee Handler   | BasicFeeHandler                                                                                                               |
| Registered Handler       | ERC20Handler                                                                                                                  |
| Bridging Strategy        | Lock/Release                                                                                                                  |
| Resource ID              | 0x0000000000000000000000000000000000000000000000000000000000001000                                                            |
| Sepolia Contract Address | [0xAE3283fb214cDb14884039Df09B7895773e68eFA](https://sepolia.etherscan.io/address/0xAE3283fb214cDb14884039Df09B7895773e68eFA) |

**Permissionless generic message (MAX_FEE = 15 million)**

| Details                           | Information                                                        |
|-----------------------------------|--------------------------------------------------------------------|
| Type                              | Generic                                                            |
| Registered Fee Handler            | BasicFeeHandler                                                    |
| Registered Handler                | PermissionlessGenericHandler                                       |
| Bridging Strategy                 | GMP                                                                |
| Resource ID                       | 0x0000000000000000000000000000000000000000000000000000000000000600 |
| Sepolia Contract Address          | N/A                                                                |
| Cronos-Testnet Contract Address   | N/A                                                                |
| Holesky Contract Address          | N/A                                                                |
| Arbitrum-Sepolia Contract Address | N/A                                                                |
| Base-sepolia Contract Address     | N/A                                                                |
| Amoy Contract Address             | N/A                                                                |

**SygmaUSD**

| Details                       | Information                                                                                                                   |
|-------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| Symbol                        | sygUSD                                                                                                                        |
| Type                          | ERC-20                                                                                                                        |
| Registered Fee Handler        | PercentageFeeHandler                                                                                                          |
| Registered Handler            | ERC20Handler                                                                                                                  |
| Bridging Strategy             | Lock/Release                                                                                                                  |
| Resource ID                   | 0x0000000000000000000000000000000000000000000000000000000000001100                                                            |
| Sepolia Contract Address      | [0xA9F30c6B5E7996D1bAd51D213277c30750bcBB36](https://sepolia.etherscan.io/address/0xA9F30c6B5E7996D1bAd51D213277c30750bcBB36) |
| Base sepolia Contract Address | [0xb947F89269F0cF54CC721BcDE298a46930f3418b](https://sepolia.basescan.org/address/0xb947f89269f0cf54cc721bcde298a46930f3418b) |

**ERC1155TST**

| Details                  | Information                                                                                                                          |
|--------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| Symbol                   | ERC1155TST                                                                                                                           |
| Type                     | ERC-1155                                                                                                                             |
| Registered Fee Handler   | BasicFeeHandler                                                                                                                      |
| Registered Handler       | ERC1155Handler                                                                                                                       |
| Bridging Strategy        | Mint/Burn                                                                                                                            |
| Resource ID              | 0x0000000000000000000000000000000000000000000000000000000000000400                                                                   |
| Sepolia Contract Address | [0xc6DE9aa04eF369540A6A4Fa2864342732bC99d06](https://sepolia.etherscan.io/address/0xc6DE9aa04eF369540A6A4Fa2864342732bC99d06)        |
| Cronos Contract Address  | [0x0d3ce33038a3e9bf940eca6f5eadf355d47d36b3](https://explorer.cronos.org/testnet/address/0x0d3ce33038a3e9bf940eca6f5eadf355d47d36b3) |


**SygmaBTC**

| Details                  | Information                                                                                                                   |
|--------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| Symbol                   | sygBTC                                                                                                                        |
| Type                     | Fungible                                                                                                                      |
| Registered Fee Handler   | BasicFeeHandler                                                                                                               |
| Registered Handler       | ERC20Handler                                                                                                                  |
| Bridging Strategy        | Lock/Release                                                                                                                  |
| Resource ID              | 0x0000000000000000000000000000000000000000000000000000000000000700                                                            |
| Sepolia Contract Address | [0xc3cb14a020319f479ff164485008896a853dc8ca](https://sepolia.etherscan.io/address/0xc3cb14a020319f479ff164485008896a853dc8ca) |
| Holesky Contract Address | [0x34d4fb8c45060143d39b7526c2b645d351af85a5](https://holesky.etherscan.io/address/0x34d4fb8c45060143d39b7526c2b645d351af85a5) |


## Fee Schemes

| Network Name                    | Handler Address                                                                                                                       | Fee Type   | Fee Percent/Amount | Gas Amount |
|---------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|------------|--------------------|------------|
| Sepolia Fixed                   | [0x356B7B3C25355325CcBFBCF00a82895F93f086b7](https://sepolia.etherscan.io/address/0x356B7B3C25355325CcBFBCF00a82895F93f086b7)         | Fixed fee  | 0.001 ETH          |            |
| Sepolia Percentage-based        | [0xDDa14370765696cD64bA93b974a934FF18A278e2](https://sepolia.etherscan.io/address/0xDDa14370765696cD64bA93b974a934FF18A278e2)         | Percentage | 10 BPS (or 0.1%)   |            |
| Cronos-Testnet Fixed            | [0x1afc317C3C8578274B557a022aD79e073ef6EBF7](https://explorer.cronos.org/testnet/address/0x1afc317C3C8578274B557a022aD79e073ef6EBF7)  | Fixed fee  | 0.001 ETH          |            |
| Cronos-Testnet Percentage-based | [0xC5244700904c987E5441F568440ed5c3b7233559](https://explorer.cronos.org/testnet/address/0xC5244700904c987E5441F568440ed5c3b7233559)  | Percentage | 1 BPS (or 0.01%)   |            |
| Holesky Fixed                   | [0xd6224d509DDa6c5eB3FFd3204218AD8eEC8F59Ef](https://holesky.etherscan.io/address/0xd6224d509DDa6c5eB3FFd3204218AD8eEC8F59Ef)         | Fixed fee  | 0.001 ETH          |            |
| Holesky Percentage-based        | [0xa477963806167acB38830Ea91B081a3de47B6ce9](https://holesky.etherscan.io/address/0xa477963806167acB38830Ea91B081a3de47B6ce9)         | Percentage | 1 BPS (or 0.01%)   |            |
| Arbitrum-Sepolia Fixed          | [0x2D7651aF6FFdb486571067568BFefA6e7fe7Fa8a](https://sepolia.arbiscan.io/address/0x2D7651aF6FFdb486571067568BFefA6e7fe7Fa8a)          | Fixed fee  | 0.001 ETH          |            |
| Gnosis-Chiado Fixed             | [0x27284B791992a40D18197087477304368Ac67Fcb](https://gnosis-chiado.blockscout.com/address/0x27284B791992a40D18197087477304368Ac67Fcb) | Fixed fee  | 0.001 xDAI         |            |
| Base sepolia Fixed              | [0x52757D9c1a8f8b496cf1e688bbB8055f9F3DfC8a](https://sepolia.basescan.org/address/0x52757D9c1a8f8b496cf1e688bbB8055f9F3DfC8a)         | Fixed fee  | 0.001 ETH          |            |
| Base sepolia Percentage-based   | [0xa477963806167acB38830Ea91B081a3de47B6ce9](https://sepolia.basescan.org/address/0x8b8F51bB26Eb521ac33dD6DcAA97f5d052977746)         | Percentage | 1 BPS (or 0.01%)   |            |
| Amoy Fixed                      | [0x5842071a7D59e56c73874194A1Ea25FcFB6e00D7](https://www.oklink.com/amoy/address/0x5842071a7D59e56c73874194A1Ea25FcFB6e00D7)          | Fixed fee  | 0.001 ETH          |            |
| Amoy Percentage-based           | [0x2E3d3b2535785341584D17840F59dc0370b3504D](https://www.oklink.com/amoy/address/0x2E3d3b2535785341584D17840F59dc0370b3504D)          | Percentage | 1 BPS (or 0.01%)   |            |
| B3 Sepolia Fixed                | [0x8651f04eFA1FccfB9Cd27c6FE3FA0F2E7CB7A75a](https://sepolia.explorer.b3.fun/address/0x8651f04eFA1FccfB9Cd27c6FE3FA0F2E7CB7A75a)      | Fixed fee  | 0.001 ETH          |            |
| B3 Sepolia Percentage-based     | [0xA6Cd2A4c41946fAADACf88a87A50997B04abE557](https://sepolia.explorer.b3.fun/address/0xA6Cd2A4c41946fAADACf88a87A50997B04abE557)      | Percentage | 1 BPS (or 0.01%)   |            |


## Sygma Explorer

The [explorer UI](https://scan.test.buildwithsygma.com/) provides users with a cross-chain block explorer that scans for testnet transactions through the Sygma protocol.
