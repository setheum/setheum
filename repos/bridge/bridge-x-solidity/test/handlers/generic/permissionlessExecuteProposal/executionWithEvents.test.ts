// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
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
import { gmpStorageProof1, gmpAccountProof1 } from "../../../testingProofs";

describe("PermissionlessGenericHandler - [Execute Proposal] - TestStore", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const expectedDepositNonce = 1;

  const feeData = "0x";
  const destinationMaxFee = BigInt(900000);
  const hashOfTestStore = ethers.keccak256("0xc0ffee");
  const testDepositAddress = "0x7e62dE4008D51B0E91EaB6d21642e427dbBFb9Bb";
  const testStoreAddress = "0x5dc74c72438aECb5348f3121F1223B626627D826";
  const securityModel = 1;
  const slot = 5177893;
  const routerAddress = "0x00fe3528b1FC9ec96E232cb4d234D1515d404600";
  const stateRoot =
    "0x37891de84a8a5b6195dd2ef4379cb153e7a129d14e4d85be66c23e0161aa7453";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let executorInstance: Executor;
  let permissionlessGenericHandlerInstance: PermissionlessGenericHandler;
  let stateRootStorageInstance: StateRootStorage;
  let testStoreInstance: TestStore;
  let depositorAccount: HardhatEthersSigner;
  let relayerAccount: HardhatEthersSigner;
  let invalidExecutionContractAddress: HardhatEthersSigner;

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
    [, depositorAccount, relayerAccount, invalidExecutionContractAddress] =
      await ethers.getSigners();

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

    [testStoreInstance] = await deployMockTestContracts(
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
      storageProof: gmpStorageProof1[0].proof,
    };

    await stateRootStorageInstance.storeStateRoot(
      originDomainID,
      slot,
      stateRoot,
    );
  });

  it("AssetStored event should be emitted", async () => {
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
      .connect(relayerAccount)
      .executeProposal(proposal, gmpAccountProof1, slot);

    await expect(executeTx)
      .to.emit(testStoreInstance, "AssetStored")
      .withArgs(hashOfTestStore);

    assert.isTrue(
      await testStoreInstance._assetsStored(hashOfTestStore),
      "TestStore asset was not successfully stored",
    );
  });

  it("deposit can be executed successfully", async () => {
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
    await expect(
      executorInstance
        .connect(relayerAccount)
        .executeProposal(proposal, gmpAccountProof1, slot),
    ).not.to.be.reverted;

    // Verifying asset was marked as stored in testStoreInstance
    assert.isTrue(await testStoreInstance._assetsStored(hashOfTestStore));
  });

  it("ProposalExecution should be emitted even if handler execution fails", async () => {
    // execution contract address
    const invalidDepositData = createPermissionlessGenericDepositData(
      depositFunctionSignature,
      await invalidExecutionContractAddress.getAddress(),
      destinationMaxFee,
      await depositorAccount.getAddress(),
      hashOfTestStore,
    );

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          originDomainID,
          resourceID,
          securityModel,
          invalidDepositData,
          feeData,
        ),
    ).not.to.be.reverted;

    // relayerAccount executes the proposal
    const executeTx = await executorInstance
      .connect(relayerAccount)
      .executeProposal(proposal, gmpAccountProof1, slot);

    // check that ProposalExecution event is emitted
    await expect(executeTx).to.emit(executorInstance, "ProposalExecution");

    // check that deposit nonce isn't unmarked as used in bitmap
    assert.isTrue(
      await executorInstance.isProposalExecuted(
        originDomainID,
        expectedDepositNonce,
      ),
    );

    // Check that asset isn't marked as stored in testStoreInstance
    assert.isTrue(await testStoreInstance._assetsStored(hashOfTestStore));
  });
});
