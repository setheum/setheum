// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  Router,
  Executor,
  ERC20Handler,
  ERC20PresetMinterPauser,
  StateRootStorage,
} from "../../../typechain-types";
import { deployBridgeContracts, createERCDepositData } from "../../helpers";
import {
  accountProof1,
  storageProof1,
  accountProof2,
  storageProof2,
} from "../../testingProofs";

describe("E2E ERC20 - Two EVM Chains", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const initialTokenAmount = 100;
  const depositAmount = 10;
  const expectedDepositNonce = 1;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const destinationSlot = 5090531;
  const originSlot = 5096975;
  const originRouterAddress = "0x8a54cA98Bd754eA44df27f877a71753DC262cD7d";
  const destinationRouterAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const originStateRoot =
    "0x48c58cc9b3715b8ed660a7162f1aa0276ae8b48cacd74d74ebea25a4d1100833";
  const destinationStateRoot =
    "0xdf5a6882ccba1fd513c68a254fa729e05f769b2fa312011e1f5c38cde69964c7";

  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;
  let originDepositData: string;
  let originResourceID: string;
  let originBridgeInstance: Bridge;
  let originRouterInstance: Router;
  let originExecutorInstance: Executor;
  let originStateRootStorageInstance: StateRootStorage;
  let originERC20MintableInstance: ERC20PresetMinterPauser;
  let originERC20HandlerInstance: ERC20Handler;
  let originRelayer1: HardhatEthersSigner;

  let destinationBridgeInstance: Bridge;
  let destinationRouterInstance: Router;
  let destinationExecutorInstance: Executor;
  let destinationStateRootStorageInstance: StateRootStorage;
  let destinationDepositData: string;
  let destinationResourceID: string;
  let destinationERC20MintableInstance: ERC20PresetMinterPauser;
  let destinationERC20HandlerInstance: ERC20Handler;
  let destinationRelayer1: HardhatEthersSigner;

  let originDomainProposal: {
    originDomainID: number;
    securityModel: number;
    depositNonce: number;
    data: string;
    resourceID: string;
    storageProof: Array<string>;
  };
  let destinationDomainProposal: {
    originDomainID: number;
    securityModel: number;
    depositNonce: number;
    data: string;
    resourceID: string;
    storageProof: Array<string>;
  };

  beforeEach(async () => {
    [
      ,
      depositorAccount,
      recipientAccount,
      originRelayer1,
      destinationRelayer1,
    ] = await ethers.getSigners();
    [
      originBridgeInstance,
      originRouterInstance,
      originExecutorInstance,
      originStateRootStorageInstance,
    ] = await deployBridgeContracts(originDomainID, originRouterAddress);
    [
      destinationBridgeInstance,
      destinationRouterInstance,
      destinationExecutorInstance,
      destinationStateRootStorageInstance,
    ] = await deployBridgeContracts(
      destinationDomainID,
      destinationRouterAddress,
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    originERC20MintableInstance = await ERC20MintableContract.deploy(
      "Token",
      "TOK",
    );
    destinationERC20MintableInstance = await ERC20MintableContract.deploy(
      "Token",
      "TOK",
    );
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    originERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await originBridgeInstance.getAddress(),
    );
    destinationERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await destinationBridgeInstance.getAddress(),
    );

    originResourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    destinationResourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    await originERC20MintableInstance.mint(
      depositorAccount,
      initialTokenAmount,
    );

    await originERC20MintableInstance
      .connect(depositorAccount)
      .approve(await originERC20HandlerInstance.getAddress(), depositAmount),
      await originERC20MintableInstance.grantRole(
        await originERC20MintableInstance.MINTER_ROLE(),
        await originERC20HandlerInstance.getAddress(),
      ),
      await destinationERC20MintableInstance.grantRole(
        await destinationERC20MintableInstance.MINTER_ROLE(),
        await destinationERC20HandlerInstance.getAddress(),
      ),
      await originBridgeInstance.adminSetResource(
        await originERC20HandlerInstance.getAddress(),
        originResourceID,
        await originERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
      await originBridgeInstance.adminSetBurnable(
        await originERC20HandlerInstance.getAddress(),
        await originERC20MintableInstance.getAddress(),
      ),
      await destinationBridgeInstance.adminSetResource(
        await destinationERC20HandlerInstance.getAddress(),
        destinationResourceID,
        await destinationERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
      await destinationBridgeInstance.adminSetBurnable(
        await destinationERC20HandlerInstance.getAddress(),
        await destinationERC20MintableInstance.getAddress(),
      );

    originDepositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );

    destinationDepositData = createERCDepositData(
      depositAmount,
      20,
      await depositorAccount.getAddress(),
    );

    originDomainProposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: originDepositData,
      resourceID: destinationResourceID,
      storageProof: storageProof1[0].proof,
    };

    destinationDomainProposal = {
      originDomainID: destinationDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: destinationDepositData,
      resourceID: originResourceID,
      storageProof: storageProof2[0].proof,
    };

    await destinationStateRootStorageInstance.storeStateRoot(
      originDomainID,
      destinationSlot,
      destinationStateRoot,
    );
    await originStateRootStorageInstance.storeStateRoot(
      destinationDomainID,
      originSlot,
      originStateRoot,
    );
  });

  it("[sanity] depositorAccount' balance should be equal to initialTokenAmount", async () => {
    const depositorBalance =
      await originERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(depositorBalance, BigInt(initialTokenAmount));
  });

  it("[sanity] await originERC20HandlerInstance.getAddress() should have an allowance of depositAmount from depositorAccount", async () => {
    const handlerAllowance = await originERC20MintableInstance.allowance(
      depositorAccount,
      await originERC20HandlerInstance.getAddress(),
    );
    assert.strictEqual(handlerAllowance, BigInt(depositAmount));
  });

  it("[sanity] await destinationERC20HandlerInstance.getAddress()  should have minterRole for destinationERC20MintableInstance", async () => {
    const isMinter = await destinationERC20MintableInstance.hasRole(
      await destinationERC20MintableInstance.MINTER_ROLE(),
      await destinationERC20HandlerInstance.getAddress(),
    );
    assert.isTrue(isMinter);
  });

  it(`E2E: depositAmount of Origin ERC20 owned by depositAddress to Destination ERC20
      owned by recipientAccount and back again`, async () => {
    let depositorBalance;
    let recipientBalance;

    // depositorAccount makes initial deposit of depositAmount
    await expect(
      originRouterInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          originResourceID,
          securityModel,
          originDepositData,
          feeData,
        ),
    ).not.to.be.reverted;

    // destinationRelayer1 executes the proposal
    await expect(
      destinationExecutorInstance
        .connect(destinationRelayer1)
        .executeProposal(originDomainProposal, accountProof1, destinationSlot),
    ).not.to.be.reverted;

    // Assert ERC20 balance was transferred from depositorAccount
    depositorBalance =
      await originERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(
      depositorBalance,
      BigInt(initialTokenAmount) - BigInt(depositAmount),
      "depositAmount wasn't transferred from depositorAccount",
    );

    // Assert ERC20 balance was transferred to recipientAccount
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(
      recipientBalance,
      BigInt(depositAmount),
      "depositAmount wasn't transferred to recipientAccount",
    );

    // At this point a representation of OriginERC20Mintable has been transferred from
    // depositor to the recipient using Both Bridges and DestinationERC20Mintable.
    // Next we will transfer DestinationERC20Mintable back to the depositor

    await destinationERC20MintableInstance
      .connect(recipientAccount)
      .approve(
        await destinationERC20HandlerInstance.getAddress(),
        depositAmount,
      );

    // recipientAccount makes a deposit of the received depositAmount
    await expect(
      destinationRouterInstance
        .connect(recipientAccount)
        .deposit(
          originDomainID,
          destinationResourceID,
          securityModel,
          destinationDepositData,
          feeData,
        ),
    ).not.to.be.reverted;

    // Recipient should have a balance of 0 (deposit amount - deposit amount)
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance, BigInt(0));

    // destinationRelayer1 executes the proposal
    await expect(
      originExecutorInstance
        .connect(originRelayer1)
        .executeProposal(destinationDomainProposal, accountProof2, originSlot),
    ).not.to.be.reverted;

    // Assert ERC20 balance was transferred from recipientAccount
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance, BigInt(0));

    // Assert ERC20 balance was transferred to recipientAccount
    depositorBalance =
      await originERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(depositorBalance, BigInt(initialTokenAmount));
  });
});
