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
import { gmpStorageProof3, gmpAccountProof3 } from "../../../testingProofs";

describe("PermissionlessGenericHandler - [Execute Proposal] - packed deposit data", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const expectedDepositNonce = 1;

  const feeData = "0x";
  const destinationMaxFee = BigInt(900000);
  const handlerResponseLength = 64;
  const contractCallReturndata = ethers.ZeroHash;
  const testDepositAddress = "0x7e62dE4008D51B0E91EaB6d21642e427dbBFb9Bb";
  const testStoreAddress = "0x5dc74c72438aECb5348f3121F1223B626627D826";
  const testBridgeAddress = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512";
  const securityModel = 1;
  const slot = 5183814;
  const routerAddress = "0x5c3B8962b03d004900b9bcaF79A3cAB056A3e740";
  const stateRoot =
    "0xcd38ff0fa708db8f23c5ead59467f55bf2d3507d54a6f28680264ac3ed837c1f";

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
      testDepositInstance.interface.getFunction("executePacked").selector;

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

    await stateRootStorageInstance.storeStateRoot(
      originDomainID,
      slot,
      stateRoot,
    );
  });

  it("call with packed depositData should be successful", async () => {
    const num = 5;
    const addresses = [testBridgeAddress, await testStoreInstance.getAddress()];

    const message = ethers.encodeBytes32String("message");
    const executionData = ethers.AbiCoder.defaultAbiCoder().encode(
      ["uint", "address[]", "bytes"],
      [num, addresses, message],
    );

    // If the target function accepts (address depositor, bytes executionData)
    // then this helper can be used
    const preparedExecutionData =
      await testDepositInstance.prepareDepositData(executionData);
    const depositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testDepositInstance.getAddress(),
      destinationMaxFee,
      await depositorAccount.getAddress(),
      preparedExecutionData,
    );

    const proposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      resourceID: resourceID,
      data: depositData,
      storageProof: gmpStorageProof3[0].proof,
    };

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
      .executeProposal(proposal, gmpAccountProof3, slot);

    // check that ProposalExecution event is emitted
    await expect(executeTx)
      .to.emit(executorInstance, "ProposalExecution")
      .withArgs(
        originDomainID,
        expectedDepositNonce,
        ethers.AbiCoder.defaultAbiCoder().encode(
          ["bool", "uint256", "bytes32"],
          [true, handlerResponseLength, contractCallReturndata],
        ),
      );

    await expect(executeTx)
      .to.emit(testDepositInstance, "TestExecute")
      .withArgs(
        await depositorAccount.getAddress(),
        num,
        await testStoreInstance.getAddress(),
        message,
      );
  });
});
