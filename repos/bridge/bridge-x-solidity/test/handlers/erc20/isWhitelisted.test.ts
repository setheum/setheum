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
  ERC20PresetMinterPauser,
} from "../../../typechain-types";
import { deployBridgeContracts } from "../../helpers";

describe("ERC20Handler - [isWhitelisted]", () => {
  const domainID = 1;
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;

  let resourceID1: string;

  beforeEach(async () => {
    [bridgeInstance] = await deployBridgeContracts(domainID, routerAddress);
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    resourceID1 = ethers.zeroPadValue(
      (await ERC20MintableInstance.getAddress()) +
        ethers.toBeHex(domainID).substring(2),
      32,
    );
  });

  it("[sanity] contract should be deployed successfully", async () => {
    expect(await ERC20HandlerInstance.getAddress()).not.to.be.undefined;
  });

  it("initialContractAddress should be whitelisted", async () => {
    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance.getAddress(),
      resourceID1,
      await ERC20MintableInstance.getAddress(),
      emptySetResourceData,
    );
    const isWhitelisted = (
      await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance.getAddress(),
      )
    ).isWhitelisted;

    assert.isTrue(isWhitelisted, "Contract wasn't successfully whitelisted");
  });
});
