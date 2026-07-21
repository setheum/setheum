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

import type { BIP32Interface } from 'bip32';
import { BIP32Factory } from 'bip32';
import * as bip39 from 'bip39';
import * as bitcoinjs from 'bitcoinjs-lib';
import * as tinysecp from 'tiny-secp256k1';

import { TypeOfAddress } from '../types.js';
import { getPublicKey } from '../utils/wallet.js';

bitcoinjs.initEccLib(tinysecp);

describe('getPublicKey', () => {
  const bip32 = BIP32Factory(tinysecp);

  it('should return derivedNode for non-P2TR addresses', async () => {
    const p2wpkhDerivationPath = "m/84'/0'/0'/0/0";
    const mnemonic = bip39.generateMnemonic(256);
    const result = (await getPublicKey({
      bip32,
      mnemonic: mnemonic,
      derivationPath: p2wpkhDerivationPath,
      network: bitcoinjs.networks.testnet,
      typeOfAddress: TypeOfAddress.P2WPKH,
    })) as { derivedNode: BIP32Interface };

    expect(result.derivedNode).toBeDefined();
  });

  it('should return tweakedSigner and publicKeyDropedDERHeader for P2TR addresses', async () => {
    const p2trDerivationPath = "m/86'/0'/0'/0/0";
    const mnemonic = bip39.generateMnemonic(256);
    const result = (await getPublicKey({
      bip32,
      mnemonic: mnemonic,
      derivationPath: p2trDerivationPath,
      network: bitcoinjs.networks.testnet,
      typeOfAddress: TypeOfAddress.P2TR,
    })) as unknown as { tweakedSigner: Buffer; publicKeyDropedDERHeader: Buffer };

    expect(result.tweakedSigner).toBeDefined();
    expect(result.publicKeyDropedDERHeader).toBeDefined();
  });
});
