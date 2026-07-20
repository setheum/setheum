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

import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20Handler__factory,
  ERC20PresetMinterPauser,
} from "../../../typechain-types";
import { deployBridgeContracts } from "../../helpers";

describe("ERC20Handler - [constructor]", function () {
  const domainID = 1;
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let initialResourceIDs: Array<string> = [];
  let initialContractAddresses: Array<string> = [];

  let bridgeInstance: Bridge;
  let ERC20HandlerContract: ERC20Handler__factory;
  let ERC20MintableInstance1: ERC20PresetMinterPauser;
  let ERC20MintableInstance2: ERC20PresetMinterPauser;
  let ERC20MintableInstance3: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;

  beforeEach(async () => {
    [bridgeInstance] = await deployBridgeContracts(domainID, routerAddress);

    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance1 = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20MintableInstance2 = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20MintableInstance3 = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20HandlerContract = await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    initialResourceIDs = [
      ethers.zeroPadValue(
        (await ERC20MintableInstance1.getAddress()) +
          ethers.toBeHex(domainID).substring(2),
        32,
      ),
      ethers.zeroPadValue(
        (await ERC20MintableInstance2.getAddress()) +
          ethers.toBeHex(domainID).substring(2),
        32,
      ),

      ethers.zeroPadValue(
        (await ERC20MintableInstance3.getAddress()) +
          ethers.toBeHex(domainID).substring(2),
        32,
      ),
    ];

    initialContractAddresses = [
      await ERC20MintableInstance1.getAddress(),
      await ERC20MintableInstance2.getAddress(),
      await ERC20MintableInstance3.getAddress(),
    ];
  });

  it("[sanity] contract should be deployed successfully", async () => {
    expect(await ERC20HandlerInstance.getAddress()).not.to.be.undefined;
  });

  it("[sanity] bridge configured on domain", async () => {
    assert.deepEqual(await bridgeInstance._domainID(), BigInt(domainID));
  });

  it("[sanity] bridge should be initially unpaused", async () => {
    assert.isFalse(await bridgeInstance.paused());
  });

  it("initialResourceIDs should be parsed correctly and corresponding resourceID mappings should have expected values", async () => {
    const ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetResource(
          await ERC20HandlerInstance.getAddress(),
          initialResourceIDs[i],
          initialContractAddresses[i],
          emptySetResourceData,
        ),
      ).not.to.be.reverted;
    }

    for (const resourceID of initialResourceIDs) {
      const tokenAddress = "0x" + resourceID.substring(24, 64);

      const retrievedTokenAddress = (
        await ERC20HandlerInstance._resourceIDToTokenContractAddress(resourceID)
      ).toLowerCase();
      assert.strictEqual(tokenAddress, retrievedTokenAddress);

      const retrievedResourceID = (
        await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
          tokenAddress,
        )
      ).resourceID;

      assert.strictEqual(resourceID, retrievedResourceID);
    }
  });
});
