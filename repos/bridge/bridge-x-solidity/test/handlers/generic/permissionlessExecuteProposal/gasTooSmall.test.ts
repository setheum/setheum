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
  TestDeposit,
  Bridge,
  Router,
  Executor,
  StateRootStorage,
  PermissionlessGenericHandler,
  TestStore,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  constructGenericHandlerSetResourceData,
  createPermissionlessGenericDepositData,
  blankFunctionDepositorOffset,
  blankFunctionSig,
  deployMockTestContracts,
} from "../../../helpers";
import { gmpStorageProof2, gmpAccountProof2 } from "../../../testingProofs";

describe("PermissionlessGenericHandler - [Execute Proposal] - Gas to small", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const expectedDepositNonce = 1;

  const feeData = "0x";
  const destinationMaxFee = BigInt(900000);
  const hashOfTestStore = ethers.keccak256("0xc0ffee");
  const testDepositAddress = "0x7e62dE4008D51B0E91EaB6d21642e427dbBFb9Bb";
  const testStoreAddress = "0x5dc74c72438aECb5348f3121F1223B626627D826";
  const securityModel = 1;
  const slot = 5183086;
  const routerAddress = "0x5d2f2755cc8E2f5569a92F828C15cC1Bd2e6cf44";
  const stateRoot =
    "0xcc95b04488bc4e4918eb5d93870d7f49ded2d51d3847a6ee9489179ea02fdd71";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let executorInstance: Executor;
  let permissionlessGenericHandlerInstance: PermissionlessGenericHandler;
  let stateRootStorageInstance: StateRootStorage;
  let testStoreInstance: TestStore;
  let testDepositInstance: TestDeposit;
  let depositorAccount: HardhatEthersSigner;

  let resourceID: string;
  let depositFunctionSignature: string;
  let depositData: string;
  let proposal: {
    originDomainID: number;
    securityModel: number;
    depositNonce: number;
    resourceID: string;
    data: string;
    storageProof: Array<string>;
  };

  beforeEach(async () => {
    [, depositorAccount] = await ethers.getSigners();

    [
      bridgeInstance,
      routerInstance,
      executorInstance,
      stateRootStorageInstance,
    ] = await deployBridgeContracts(destinationDomainID, routerAddress);
    const PermissionlessGenericHandlerContract =
      await ethers.getContractFactory("PermissionlessGenericHandler");
    permissionlessGenericHandlerInstance =
      await PermissionlessGenericHandlerContract.deploy(
        await bridgeInstance.getAddress(),
      );

    [testStoreInstance, testDepositInstance] = await deployMockTestContracts(
      testStoreAddress,
      testDepositAddress,
    );

    resourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    depositFunctionSignature =
      testStoreInstance.interface.getFunction("storeWithDepositor").selector;

    const PermissionlessGenericHandlerSetResourceData =
      constructGenericHandlerSetResourceData(
        depositFunctionSignature,
        blankFunctionDepositorOffset,
        blankFunctionSig,
      );
    await bridgeInstance.adminSetResource(
      await permissionlessGenericHandlerInstance.getAddress(),
      resourceID,
      await testStoreInstance.getAddress(),
      ethers.toBeHex(PermissionlessGenericHandlerSetResourceData),
    );

    await bridgeInstance.adminSetResource(
      await permissionlessGenericHandlerInstance.getAddress(),
      resourceID,
      await testStoreInstance.getAddress(),
      ethers.toBeHex(PermissionlessGenericHandlerSetResourceData),
    );
    depositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testStoreInstance.getAddress(),
      destinationMaxFee,
      await depositorAccount.getAddress(),
      hashOfTestStore,
    );
    proposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      resourceID: resourceID,
      data: depositData,
      storageProof: gmpStorageProof2[0].proof,
    };

    await stateRootStorageInstance.storeStateRoot(
      originDomainID,
      slot,
      stateRoot,
    );
  });

  it("ProposalExecution should be emitted even if gas specified too small", async () => {
    const num = 6;
    const addresses = [
      await bridgeInstance.getAddress(),
      await testStoreInstance.getAddress(),
    ];
    const message = ethers.encodeBytes32String("message");
    const executionData = ethers.AbiCoder.defaultAbiCoder().encode(
      ["uint256", "address[]", "bytes"],
      [num, addresses, message],
    );

    // If the target function accepts (address depositor, bytes executionData)
    // then this helper can be used
    const preparedExecutionData =
      await testDepositInstance.prepareDepositData(executionData);
    const depositFunctionSignature =
      testDepositInstance.interface.getFunction("executePacked").selector;
    const tooSmallGas = BigInt(500);

    const depositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testDepositInstance.getAddress(),
      tooSmallGas,
      await depositorAccount.getAddress(),
      preparedExecutionData,
    );

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          originDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;

    // relayerAccount executes the proposal
    const executeTx = await executorInstance
      .connect(depositorAccount)
      .executeProposal(proposal, gmpAccountProof2, slot);

    // check that ProposalExecution event is emitted
    await expect(executeTx).to.emit(executorInstance, "ProposalExecution");

    await expect(executeTx).not.to.emit(testDepositInstance, "TestExecute");
  });
});
