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

import type { ParachainId } from '@buildwithsygma/core';
import { Network } from '@buildwithsygma/core';
import type { NativeTokenAdapter } from '@buildwithsygma/sygma-contracts';
import { hexlify } from '@ethersproject/bytes';
import type { ethers } from 'ethers';

import type {
  FungibleTransferOptionalMessage,
  NativeTokenDepositArgsWithEVMMessage,
  NativeTokenDepositArgsWithGeneralMessage,
  NativeTokenDepositArgsWithoutMessage,
  NativeTokenDepositMethods,
  TransactionRequest,
} from '../types.js';

import { serializeDestinationAddress, encodeOptionalMessage } from './assetTransferHelpers.js';
import { createTransactionRequest } from './transaction.js';

export function getNativeTokenDepositMethod(
  destinationNetworkType: Network,
  optionalMessage?: FungibleTransferOptionalMessage,
): NativeTokenDepositMethods {
  if (!optionalMessage) {
    if (destinationNetworkType === Network.EVM) {
      return 'depositToEVM';
    }
    return 'deposit';
  } else {
    if (destinationNetworkType === Network.EVM) {
      return 'depositToEVMWithMessage';
    }

    return 'depositGeneral';
  }
}

interface NativeDepositParams {
  method: NativeTokenDepositMethods;
  recipientAddress: string;
  destinationNetworkType: Network;
  destinationNetworkId: string;
  parachainId?: ParachainId;
  optionalMessage?: FungibleTransferOptionalMessage;
  optionalGas?: bigint;
  depositData: string;
}

export function getNativeTokenDepositContractArgs(
  args: NativeDepositParams,
):
  | NativeTokenDepositArgsWithoutMessage
  | NativeTokenDepositArgsWithEVMMessage
  | NativeTokenDepositArgsWithGeneralMessage {
  const {
    method,
    destinationNetworkId,
    recipientAddress,
    destinationNetworkType,
    parachainId,
    optionalMessage,
    optionalGas,
    depositData,
  } = args;

  switch (method) {
    case 'deposit':
    case 'depositToEVM':
      return [
        destinationNetworkId,
        hexlify(serializeDestinationAddress(recipientAddress, destinationNetworkType, parachainId)),
      ];
    case 'depositToEVMWithMessage':
      return [
        destinationNetworkId,
        recipientAddress,
        optionalGas!,
        encodeOptionalMessage(optionalMessage!),
      ];
    case 'depositGeneral':
      return [destinationNetworkId, `0x${depositData.substring(66)}`];
  }
}

export async function getNativeTokenDepositTransaction(
  depositParams: Omit<NativeDepositParams, 'method'>,
  nativeTokenAdapter: NativeTokenAdapter,
  overrides?: ethers.PayableOverrides,
): Promise<TransactionRequest> {
  const method = getNativeTokenDepositMethod(
    depositParams.destinationNetworkType,
    depositParams.optionalMessage,
  );

  const args = getNativeTokenDepositContractArgs({
    method,
    ...depositParams,
  });

  switch (method) {
    case 'deposit':
    case 'depositToEVM':
      return createTransactionRequest(
        await nativeTokenAdapter.populateTransaction[method](
          ...(args as NativeTokenDepositArgsWithoutMessage),
          overrides,
        ),
      );
    case 'depositToEVMWithMessage':
      return createTransactionRequest(
        await nativeTokenAdapter.populateTransaction[method](
          ...(args as NativeTokenDepositArgsWithEVMMessage),
          overrides,
        ),
      );
    case 'depositGeneral':
      return createTransactionRequest(
        await nativeTokenAdapter.populateTransaction[method](
          ...(args as NativeTokenDepositArgsWithGeneralMessage),
          overrides,
        ),
      );
    default:
      throw new Error('Unsupported method');
  }
}
