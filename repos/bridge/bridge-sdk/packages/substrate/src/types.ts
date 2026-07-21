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

import type { Domain, FeeHandlerType, ParachainId, SubstrateResource } from '@buildwithsygma/core';
import type { ExtrinsicStatus } from '@polkadot/types/interfaces';
import type { BN } from '@polkadot/util';

export type SubstrateFee = {
  fee: BN;
  type: FeeHandlerType;
};

type AssetTransfer = {
  recipient: string;
  parachainId?: ParachainId;
};

export type Fungible = AssetTransfer & {
  amount: string;
};
export type Transfer<TransferType> = {
  details: TransferType;
  to: Domain;
  from: Domain;
  resource: SubstrateResource;
  sender: string;
};

export type DepositEventDataType = {
  depositData: string;
  depositNonce: string;
  destDomainId: string;
  handlerResponse: string;
  resourceId: string;
  sender: string;
  transferType: string;
};

export type DepositCallbacksType = {
  /**
   * Callback for when the transaction is included in a block.
   */
  onInBlock?: (status: ExtrinsicStatus) => void;
  /**
   * Callback for when the transaction is finalized.
   */
  onFinalized?: (status: ExtrinsicStatus) => void;
  /**
   * Callback for when an error occurs.
   */
  onError?: (error: unknown) => void;
  /**
   * Callback for sygmaBridge.Deposit event on finalize stage
   */
  onDepositEvent?: (data: DepositEventDataType) => void;
};
