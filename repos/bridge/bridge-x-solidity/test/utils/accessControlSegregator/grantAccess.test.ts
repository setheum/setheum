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

import type { AccessControlSegregator } from "../../../typechain-types";

describe("AccessControlSegregator - [grant access]", () => {
  const functionSignature = "0x29a71964";

  let accessControlSegregatorInstance: AccessControlSegregator;
  let accountWithAccess: HardhatEthersSigner;
  let accountWithoutAccess: HardhatEthersSigner;
  let receivingAccessAccount: HardhatEthersSigner;

  beforeEach(async () => {
    [, accountWithAccess, accountWithoutAccess, receivingAccessAccount] =
      await ethers.getSigners();

    const AccessControlSegregatorContract = await ethers.getContractFactory(
      "AccessControlSegregator",
    );
    accessControlSegregatorInstance =
      await AccessControlSegregatorContract.deploy([], []);
  });

  it("hasAccess should return false if access not granted", async () => {
    assert.isFalse(
      await accessControlSegregatorInstance.hasAccess(
        functionSignature,
        accountWithoutAccess,
      ),
    );
  });

  it("should revert if sender doesn't have  grant access rights", async () => {
    await expect(
      accessControlSegregatorInstance
        .connect(accountWithoutAccess)
        .grantAccess(functionSignature, receivingAccessAccount),
    ).to.be.revertedWithCustomError(
      accessControlSegregatorInstance,
      "SenderWithoutAccessRights",
    );
  });

  it("should successfully grant access to a function", async () => {
    await expect(
      accessControlSegregatorInstance.grantAccess(
        functionSignature,
        accountWithoutAccess,
      ),
    ).not.to.be.reverted;

    assert.isTrue(
      await accessControlSegregatorInstance.hasAccess(
        functionSignature,
        accountWithoutAccess,
      ),
    );
  });

  it("should successfully regrant access", async () => {
    await expect(
      accessControlSegregatorInstance.grantAccess(
        functionSignature,
        accountWithoutAccess,
      ),
    ).not.to.be.reverted;
    assert.isTrue(
      await accessControlSegregatorInstance.hasAccess(
        functionSignature,
        accountWithoutAccess,
      ),
    );

    await expect(
      accessControlSegregatorInstance.grantAccess(
        functionSignature,
        accountWithAccess,
      ),
    ).not.to.be.reverted;
    assert.isTrue(
      await accessControlSegregatorInstance.hasAccess(
        functionSignature,
        accountWithAccess,
      ),
    );
  });
});
