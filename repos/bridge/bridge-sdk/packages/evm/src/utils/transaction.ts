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

import type { PopulatedTransaction } from 'ethers';

import type { TransactionRequest } from '../types.js';
/**
 * Create a library agnostic transaction type
 * that can be sent to an EVM node
 * @param {PopulatedTransaction} transaction Ethers `PopulatedTransaction` object
 * @returns {TransactionRequest}
 */
export function createTransactionRequest(transaction: PopulatedTransaction): TransactionRequest {
  return {
    to: transaction.to,
    value: transaction.value ? transaction.value.toBigInt() : undefined,
    data: transaction.data,
    gasLimit: transaction.gasLimit ? transaction.gasLimit.toBigInt() : undefined,
    gasPrice: transaction.gasPrice ? transaction.gasPrice.toBigInt() : undefined,
    nonce: transaction.nonce ?? undefined,
    chainId: transaction.chainId ?? undefined,
    type: transaction.type ?? undefined,
    maxFeePerGas: transaction.maxFeePerGas ?? undefined,
    maxPriorityFeePerGas: transaction.maxPriorityFeePerGas ?? undefined,
  } as TransactionRequest;
}
