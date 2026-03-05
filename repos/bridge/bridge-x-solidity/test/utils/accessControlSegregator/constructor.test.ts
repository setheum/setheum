// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

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
