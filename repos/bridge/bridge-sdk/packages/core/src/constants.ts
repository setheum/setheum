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

export enum ConfigUrl {
  DEVNET = 'https://chainbridge-assets-stage.s3.us-east-2.amazonaws.com/shared-config-dev.json',
  TESTNET = 'https://chainbridge-assets-stage.s3.us-east-2.amazonaws.com/shared-config-test.json',
  MAINNET = 'https://sygma-assets-mainnet.s3.us-east-2.amazonaws.com/shared-config-mainnet.json',
}

export enum IndexerUrl {
  MAINNET = 'https://api.buildwithsygma.com',
  TESTNET = 'https://api.test.buildwithsygma.com',
}

export enum ExplorerUrl {
  MAINNET = 'https://scan.buildwithsygma.com',
  TESTNET = 'https://scan.test.buildwithsygma.com',
}
