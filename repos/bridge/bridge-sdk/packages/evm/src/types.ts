// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

import type {
  BaseTransferParams,
  Eip1193Provider,
  EvmResource,
  FeeHandlerType,
  SecurityModel,
} from '@buildwithsygma/core';
import type {
  Abi,
  AbiParametersToPrimitiveTypes,
  ExtractAbiFunction,
  ExtractAbiFunctionNames,
} from 'abitype';

export interface TransactionRequest {
  to: string;
  value: bigint;
  data: string;
  gasLimit: bigint;
  gasPrice?: bigint;
  nonce?: number;
  chainId?: number;
  type?: number;
  maxFeePerGas?: bigint;
  maxPriorityFeePerGas?: bigint;
}

export type EvmFee = {
  /** amount that will be deducated */
  fee: bigint;
  /** type of fee calculation that will be used */
  type: FeeHandlerType;
  /** fee handler contract address */
  handlerAddress: string;
  /** fungible token ERC20 address */
  tokenAddress?: string;
  /** Percentage - applicable when
   *  percentage calculation is used
   */
  percentage?: number;
  /** minimum Fee - applicable when
   * percentage calculation is used
   */
  minFee?: bigint;
  /** maximum Fee - applicable when
   * percentage calculation is used
   */
  maxFee?: bigint;
};

/** An EVM resource is accepted as either the resource object or it's Sygma ID */
export type EvmResourceish = string | EvmResource;

interface FungibleDepositAction {
  nativeValue: bigint;
  callTo: string;
  approveTo: string;
  tokenSend: string;
  tokenReceive: string;
  data: string;
}

export interface FungibleTransferOptionalMessage {
  transactionId: string;
  actions: FungibleDepositAction[];
  receiver: string;
}

export interface EvmTransferParams extends BaseTransferParams {
  sourceAddress: string;
  sourceNetworkProvider: Eip1193Provider;
}

export interface EvmAssetTransferParams extends EvmTransferParams {
  recipientAddress: string;
  amount?: bigint;
  tokenId?: string;
}

export interface SemiFungibleTransferParams extends EvmTransferParams {
  recipientAddress: string;
  tokenIds: string[];
  amounts: bigint[];
}

export interface FungibleTransferParams extends EvmAssetTransferParams {
  amount: bigint;
  securityModel?: SecurityModel;
  optionalGas?: bigint;
  optionalMessage?: FungibleTransferOptionalMessage;
}

export interface NonFungibleTransferParams extends EvmAssetTransferParams {
  tokenId: string;
}

export interface GenericMessageTransferParams<
  ContractAbi extends Abi,
  FunctionName extends ExtractAbiFunctionNames<ContractAbi, 'nonpayable' | 'payable'>,
> extends EvmTransferParams {
  gasLimit: bigint;
  functionParameters: AbiParametersToPrimitiveTypes<
    ExtractAbiFunction<ContractAbi, FunctionName>['inputs'],
    'inputs'
  >;
  functionName: FunctionName;
  destinationContractAbi: ContractAbi;
  destinationContractAddress: string;
  maxFee: bigint;
}

export type NativeTokenDepositArgsWithoutMessage = [string, string];
export type NativeTokenDepositArgsWithGeneralMessage = [string, string];
export type NativeTokenDepositArgsWithEVMMessage = [string, string, bigint, string];
export type NativeTokenDepositMethods =
  | 'deposit'
  | 'depositToEVM'
  | 'depositGeneral'
  | 'depositToEVMWithMessage';
