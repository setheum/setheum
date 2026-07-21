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

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  AccessControlSegregator,
  AccessControlSegregator__factory,
} from "../../../typechain-types";

describe("AccessControlSegregator - [constructor]", () => {
  const initialFunctions = [
    "0x29a71964",
    "0x78728c73",
    "0x2a64052b",
    "0x3a24555a",
  ];

  let adminAccount: HardhatEthersSigner;
  let newAdminAccount: HardhatEthersSigner;
  let accessHolder1: HardhatEthersSigner;
  let accessHolder2: HardhatEthersSigner;
  let accessHolder3: HardhatEthersSigner;
  let accessHolder4: HardhatEthersSigner;
  let AccessControlSegregatorContract: AccessControlSegregator__factory;
  let accessControlSegregatorInstance: AccessControlSegregator;

  let initialAccessHolders: Array<HardhatEthersSigner>;

  const grantAccessSig = "0xa973ec93";

  beforeEach(async () => {
    [
      adminAccount,
      newAdminAccount,
      accessHolder1,
      accessHolder2,
      accessHolder3,
      accessHolder4,
    ] = await ethers.getSigners();

    initialAccessHolders = [
      accessHolder1,
      accessHolder2,
      accessHolder3,
      accessHolder4,
    ];

    AccessControlSegregatorContract = await ethers.getContractFactory(
      "AccessControlSegregator",
    );
    accessControlSegregatorInstance =
      await AccessControlSegregatorContract.deploy(
        initialFunctions,
        initialAccessHolders,
      );
  });

  it("[sanity] should deploy contract successfully", async () => {
    await expect(AccessControlSegregatorContract.deploy([], [])).not.to.be
      .reverted;
  });

  it("should revert if length of functions and accounts array is different", async () => {
    await expect(
      AccessControlSegregatorContract.deploy(
        ["0xa973ec93", "0x78728c73"],
        [adminAccount],
      ),
    ).to.be.revertedWithCustomError(
      accessControlSegregatorInstance,
      "ArrayLengthsDoNotMatch",
    );
  });

  it("should grant deployer grant access rights", async () => {
    assert.isTrue(
      await accessControlSegregatorInstance.hasAccess(
        grantAccessSig,
        adminAccount,
      ),
    );
  });

  it("should grant function access specified in params", async () => {
    for (let i = 0; i < initialFunctions.length; i++) {
      assert.isTrue(
        await accessControlSegregatorInstance.hasAccess(
          initialFunctions[i],
          initialAccessHolders[i],
        ),
      );
    }
  });

  it("should replace grant access of deployer if specified in params", async () => {
    const accessControlSegregatorInstance =
      await AccessControlSegregatorContract.deploy(
        [grantAccessSig],
        [newAdminAccount],
      );

    assert.isFalse(
      await accessControlSegregatorInstance.hasAccess(
        grantAccessSig,
        adminAccount,
      ),
    );
    assert.isTrue(
      await accessControlSegregatorInstance.hasAccess(
        grantAccessSig,
        newAdminAccount,
      ),
    );
  });
});
