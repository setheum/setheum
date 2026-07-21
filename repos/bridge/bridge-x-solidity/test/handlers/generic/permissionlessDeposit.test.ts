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
import { expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  Router,
  PermissionlessGenericHandler,
  TestStore,
} from "../../../typechain-types";
import {
  deployBridgeContracts,
  createResourceID,
  createPermissionlessGenericDepositData,
} from "../../helpers";

describe("PermissionlessGenericHandler - [deposit]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const expectedDepositNonce = 1;

  const feeData = "0x";
  const destinationMaxFee = BigInt("900000");
  const hashOfTestStore = ethers.keccak256("0xc0ffee");
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let permissionlessGenericHandlerInstance: PermissionlessGenericHandler;
  let testStoreInstance: TestStore;
  let depositorAccount: HardhatEthersSigner;
  let invalidDepositorAccount: HardhatEthersSigner;

  let resourceID: string;
  let depositFunctionSignature: string;
  let depositData: string;

  beforeEach(async () => {
    [, depositorAccount, invalidDepositorAccount] = await ethers.getSigners();

    [bridgeInstance, routerInstance] = await deployBridgeContracts(
      originDomainID,
      routerAddress,
    );
    const PermissionlessGenericHandlerContract =
      await ethers.getContractFactory("PermissionlessGenericHandler");
    permissionlessGenericHandlerInstance =
      await PermissionlessGenericHandlerContract.deploy(
        await bridgeInstance.getAddress(),
      );
    const TestStoreContract = await ethers.getContractFactory("TestStore");
    testStoreInstance = await TestStoreContract.deploy();

    resourceID = createResourceID(
      await testStoreInstance.getAddress(),
      originDomainID,
    );

    await bridgeInstance.adminSetResource(
      await permissionlessGenericHandlerInstance.getAddress(),
      resourceID,
      testStoreInstance.getAddress(),
      emptySetResourceData,
    );

    depositFunctionSignature =
      testStoreInstance.interface.getFunction("storeWithDepositor").selector;

    depositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testStoreInstance.getAddress(),
      destinationMaxFee,
      await depositorAccount.getAddress(),
      hashOfTestStore,
    );
  });

  it("deposit can be made successfully", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;
  });

  it("depositEvent is emitted with expected values", async () => {
    const depositTx = await routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        depositData,
        feeData,
      );

    await expect(depositTx)
      .to.emit(routerInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        resourceID.toLowerCase(),
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        depositData.toLowerCase(),
      );
  });

  it("deposit data should be of required length", async () => {
    // Min length is 76 bytes
    const invalidDepositData = "0x" + "aa".repeat(75);

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          invalidDepositData,
          feeData,
        ),
    ).to.be.revertedWithCustomError(
      permissionlessGenericHandlerInstance,
      "IncorrectDataLength",
    );
  });

  it("should revert if metadata encoded depositor does not match deposit depositor", async () => {
    const invalidDepositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testStoreInstance.getAddress(),
      destinationMaxFee,
      await invalidDepositorAccount.getAddress(),
      hashOfTestStore,
    );

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          invalidDepositData,
          feeData,
        ),
    ).to.be.revertedWithCustomError(
      permissionlessGenericHandlerInstance,
      "InvalidExecutioDataDepositor",
    );
  });

  it("should revert if max fee exceeds 1000000", async () => {
    const invalidMaxFee = BigInt(1000001);
    const invalidDepositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testStoreInstance.getAddress(),
      invalidMaxFee,
      await depositorAccount.getAddress(),
      hashOfTestStore,
    );

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          invalidDepositData,
          feeData,
          {
            from: depositorAccount,
          },
        ),
    ).to.be.revertedWithCustomError(
      permissionlessGenericHandlerInstance,
      "RequestedFeeTooLarge",
    );
  });
});
