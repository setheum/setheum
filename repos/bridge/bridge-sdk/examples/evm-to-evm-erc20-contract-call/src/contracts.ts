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

import { ethers } from "ethers";
import fs from "fs";

type Contract = "sprinterNameService" | "storage";

export function getContractInterface(
  contract: Contract
): ethers.utils.Interface {
  const abi = fs.readFileSync(`${process.cwd()}/src/abi/${contract}.json`, {
    encoding: "utf-8",
  });

  const contractInterface = new ethers.utils.Interface(abi);
  return contractInterface;
}

const DEPLOYMENTS: Record<Contract, Record<number, string>> = {
  sprinterNameService: {
    84532: "0x3F9A68fF29B3d86a6928C44dF171A984F6180009",
    11155111: "0xf70fb86F700E8Bb7cDf1c20197633518235c3425",
    1993: "0x17e4C404aD634E429ebCdF9a10F38A96Ce8eEF27",
  },
  storage: {
    11155111: "0x10791B617D2Dad4978Cc18E3A88e422310428430",
    84532: "0xF1bFBbE4174E2E6595E095BDF3ac8b97aF7796aA",
    1993: "0xF5Ac994A5C402F4f426c2D7319C27912d5DBD7a8",
  },
};

export function getContractAddress(
  network: number,
  contract: Contract
): string {
  if (!!DEPLOYMENTS[contract][network]) {
    return DEPLOYMENTS[contract][network];
  }

  throw new Error("Contract address unavailable.");
}
