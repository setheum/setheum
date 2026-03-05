---
slug: /readme/governance/evmmultisig
id: governance-evm-multisig
title: EVM Governance & Multisigs
description: The following section describes the EVM multisigs administrating the Sygma protocol.
sidebar_position: 2
draft: false
---

## EVM Protocol Administration

Administration of the Sygma protocol on EVM is currently handled by a consortium of early contributors, and will be extended to other network operators over the coming weeks. The Sygma protocol on EVM is currently administered by the following set of multi-signature accounts (Multisigs):

### Administrative Multisig 

The admin multisig is a 3-of-5 and includes actions such as:
- *adminSetResource* (registering resources and routes)
- *transferFee* (withdrawing fees from the handler)
- *adminChangeFeeHandler* (changing the fee logic for a bridge)
- *changeFee* (changing fee percentage on a static fee handler)
- *setFeeOracle* (set the address of the fee oracle which provides gas price and effective rates)
- *adminPauseTransfers, adminUnpauseTransfers* (pause and unpause the bridge)

The admin multisig covers a set of super administrative privileges, such as pausing the bridge, that may be required in order to be able to reduce the impact of security incidents. As these actions may have significant financial impact, the admin governance process follows a strict off-chain preparation/review and onchain review/signing. 

The admin multisig is listed at the following Gnosis Safe Proxy contract address [0xde79695d5cefF7c324552B3ecbe6165f77FCdF53](https://etherscan.io/address/0xde79695d5cefF7c324552B3ecbe6165f77FCdF53), and is currently made up of members from **ChainSafe Systems**.

| Name            | Organization      | Address                                                                                                      |
| --------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------- |
| David Ansermino | ChainSafe Systems | [0x86a73a594f74C76a6eB8F9E728d992D03252f60f](https://etherscan.io/address/0x86a73a594f74C76a6eB8F9E728d992D03252f60f) |
| Peter Kalambet  | ChainSafe Systems | [0x5a288b42dC222190D8cF5014A330c978ee42A5df](https://etherscan.io/address/0x5a288b42dC222190D8cF5014A330c978ee42A5df) |
| Greg Markou     | ChainSafe Systems | [0xe845B1d31CaA16Bf6c6Bf5E97a28D086bd46FD49](https://etherscan.io/address/0xe845B1d31CaA16Bf6c6Bf5E97a28D086bd46FD49) |
| Mak Muftic      | ChainSafe Systems | [0xacc0268a75280192897a78C706C9FBA2d2b851C4](https://etherscan.io/address/0xacc0268a75280192897a78C706C9FBA2d2b851C4) |
| Alex Müller     | ChainSafe Systems | [0x197C57440A30cB28103ab27CB1b0dC86E5907ADA](https://etherscan.io/address/0x197C57440A30cB28103ab27CB1b0dC86E5907ADA) |

### Community Multisig

The community multisig is a 4-of-6 and includes one action:
- *adminWithdraw* (withdraw liquidity from the handler)

As described by its sole action, the community multisig is used to manage the fee handler's liquidity. This action would be required in case of a security breach, or where a migration of fee handler is required. 
 
The community multisig is listed at the following Gnosis Safe Proxy contract address [0xc4d8b2F5501C765dE0C5E12550118F397B197D05](https://etherscan.io/address/0xc4d8b2F5501C765dE0C5E12550118F397B197D05), and is currently made up of members from **ChainSafe Systems** and **Phala Network**:

| Name            | Organization      |  Address 
|-----------------|-------------------| --------------------------------------------------------------------------------------------------------------------- |
| David Ansermino | ChainSafe Systems | [0x86a73a594f74C76a6eB8F9E728d992D03252f60f](https://etherscan.io/address/0x86a73a594f74C76a6eB8F9E728d992D03252f60f) |
| Hatcher Lipton  | ChainSafe Systems | [0xC6458dedf35231F524ED9d7E0DF77A60b9E08676](https://etherscan.io/address/0xC6458dedf35231F524ED9d7E0DF77A60b9E08676) |
| Greg Markou     | ChainSafe Systems | [0xe845B1d31CaA16Bf6c6Bf5E97a28D086bd46FD49](https://etherscan.io/address/0xe845B1d31CaA16Bf6c6Bf5E97a28D086bd46FD49) |
| Marvin Tong     | Phala Network     | [0xa399460Ce767b06297457178D2F9F8f144017E77](https://etherscan.io/address/0xa399460Ce767b06297457178D2F9F8f144017E77) | 
| Wengfeng Wang   | Phala Network     | [0x0c1db86328E6CFCD4f530401131Dc9a26DefA12a](https://etherscan.io/address/0x0c1db86328E6CFCD4f530401131Dc9a26DefA12a) |
| Hang Yin        | Phala Network     | [0xd85b34B2Fe1eC7815B6dF659372382A8FA229677](https://etherscan.io/address/0xd85b34B2Fe1eC7815B6dF659372382A8FA229677) |

To maintain transparency of the community multisig governance process, all proposals are expected to be defined as a PR inside the Sygma [community multisig execution](https://github.com/sygmaprotocol/community-executions) repo. These proposals, consisting of one (or more) transactions/extrinsics, are expected to be prepared in a predefined format and created as separate files inside the repo.
