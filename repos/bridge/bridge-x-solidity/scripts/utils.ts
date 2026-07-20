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

import fs from "fs";
import { resolve } from "path";

import { keccak256, ContractFactory } from "ethers";
import type { InterfaceAbi, Fragment } from "ethers";
import { deployments, run } from "hardhat";
import type { DeployResult } from "hardhat-deploy/dist/types";

export function generateAccessControlFuncSignatures(contracts: Array<string>): {
  function: string;
  hash: string;
}[] {
  const allAccessControlFuncSignatures: {
    function: string;
    hash: string;
  }[] = [];

  contracts.map((contractName) => {
    const CONTRACT_PATH = resolve(
      __dirname,
      `../src/contracts/${contractName}.sol`,
    );
    const ARTIFACTS_PATH = resolve(
      __dirname,
      `../artifacts/src/contracts/${contractName}.sol/${contractName}.json`,
    );

    const contractArtifacts = JSON.parse(
      fs.readFileSync(ARTIFACTS_PATH).toString(),
    );
    const contractFactory = new ContractFactory(
      contractArtifacts.abi as InterfaceAbi,
      contractArtifacts.bytecode as string,
    );
    const contractMethods = contractFactory.interface.fragments
      .map((fragment: Fragment) => {
        if (fragment.type == "function") {
          return fragment.format();
        }
      })
      .filter((item) => item) as Array<string>;

    const contractInstance = fs.readFileSync(CONTRACT_PATH);

    // regex that will match all functions that have "onlyAllowed" modifier
    const regex = RegExp(
      "function\\s+(?:(?!_onlyAllowed|function).)+onlyAllowed",
      "gs",
    );

    let a;
    const b: Array<string> = [];
    // fetch all functions that have "onlyAllowed" modifier
    while ((a = regex.exec(contractInstance.toString())) !== null) {
      // filter out only function name from matching (onlyAllowed) functions
      b.push(a[0].split(/[\s()]+/)[1]);
    }

    // filter out from contracts ABI functions signatures with "onlyAllowed" modifier
    const accessControlFuncSignatures = contractMethods
      .filter((el1) => b.some((el2) => el1.includes(el2)))
      .map((func) => ({
        function: func,
        hash: keccak256(Buffer.from(func)).substring(0, 10),
      }));
    allAccessControlFuncSignatures.push(...accessControlFuncSignatures);
  });

  console.table(allAccessControlFuncSignatures);

  return allAccessControlFuncSignatures;
}

export async function verifyContract(
  contractAddress: DeployResult,
  constructorArgs: unknown[],
): Promise<void> {
  if (
    deployments.getNetworkName() != "localhost" &&
    deployments.getNetworkName() != "hardhat"
  ) {
    // wait 30 seconds for contract to be deployed before trying to verify
    await new Promise((resolve) => setTimeout(resolve, 30000));
    await run("verify:verify", {
      address: contractAddress.address,
      constructorArguments: constructorArgs,
    });
  }
}
