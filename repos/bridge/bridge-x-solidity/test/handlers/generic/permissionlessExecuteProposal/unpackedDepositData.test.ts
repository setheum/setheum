// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

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
  createPermissionlessGenericExecutionData,
  blankFunctionDepositorOffset,
  blankFunctionSig,
  deployMockTestContracts,
} from "../../../helpers";
import { gmpStorageProof4, gmpAccountProof4 } from "../../../testingProofs";

describe("PermissionlessGenericHandler - [Execute Proposal] - Unpacked deposit data", () => {
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
  const slot = 5183832;
  const routerAddress = "0x5A89859D364c80E30e202237452B48cCF7bD88D9";
  const stateRoot =
    "0x41cf4c9584f9604ac045ae6abd0f601b2b54c2f084d142cf6d549d31419d002b";

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
      testDepositInstance.interface.getFunction("executeUnpacked").selector;

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

  it("call with unpacked depositData should be successful", async () => {
    const num = 5;
    const addresses = [testBridgeAddress, await testStoreInstance.getAddress()];
    const message = ethers.encodeBytes32String("message");

    const executionData = createPermissionlessGenericExecutionData(
      ["uint", "address[]", "bytes"],
      [num, addresses, message],
    );

    const depositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await testDepositInstance.getAddress(),
      destinationMaxFee,
      await depositorAccount.getAddress(),
      executionData,
    );

    const proposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      resourceID: resourceID,
      data: depositData,
      storageProof: gmpStorageProof4[0].proof,
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

    // relayer executes the proposal
    const executeTx = await executorInstance
      .connect(depositorAccount)
      .executeProposal(proposal, gmpAccountProof4, slot);

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
