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

import { FeeHandlerType } from '@buildwithsygma/core';
import type { ethers } from 'ethers';
import { BigNumber } from 'ethers';

import type { EvmFee } from '../types.js';

export const ASSET_TRANSFER_GAS_LIMIT: BigNumber = BigNumber.from(300000);

export function getTransactionOverrides(
  fee: EvmFee,
  overrides?: ethers.Overrides,
): ethers.PayableOverrides {
  const sygmaOverrides = {
    gasLimit: ASSET_TRANSFER_GAS_LIMIT,
    value: fee.type === FeeHandlerType.PERCENTAGE ? 0 : fee.fee,
  };

  return {
    ...sygmaOverrides,
    ...overrides,
  };
}
