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

import { mnemonicToSeed } from 'bip39';
import { crypto } from 'bitcoinjs-lib';
import { toXOnly } from 'bitcoinjs-lib/src/psbt/bip371';

import { TypeOfAddress } from '../types.js';
import type { GetPublicKeyResult, PublicKeyParams } from '../types.js';

/**
 * @category Bitcoin Wallet Helpers
 * @description Return either tweakedSigner and publicKeyDropedDERHeader or derivedNode to sign a transaction
 * @param {PublicKeyParams} - bip32, mnemonic, derivationPath, network, typeOfAddress
 * @returns {Promise<GetPublicKeyResult>}
 */
export const getPublicKey = async ({
  bip32,
  mnemonic,
  derivationPath,
  network,
  typeOfAddress,
}: PublicKeyParams): Promise<GetPublicKeyResult> => {
  const seed = await mnemonicToSeed(mnemonic);
  const rootKey = bip32.fromSeed(seed, network);
  const derivedNode = rootKey.derivePath(derivationPath);

  if (typeOfAddress === TypeOfAddress.P2TR) {
    const publicKeyDropedDERHeader = toXOnly(derivedNode.publicKey);

    const tweakedSigner = derivedNode.tweak(
      crypto.taggedHash('TapTweak', publicKeyDropedDERHeader),
    );

    return { tweakedSigner, publicKeyDropedDERHeader };
  }

  return { derivedNode };
};
