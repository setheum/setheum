// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

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
