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
import { accountProof1, storageProof1 } from "../../testingProofs";

describe("E2E ERC20 - Same Chain", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const initialTokenAmount = 100;
  const depositAmount = 10;
  const expectedDepositNonce = 1;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const slot = 5090531;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const stateRoot =
    "0xdf5a6882ccba1fd513c68a254fa729e05f769b2fa312011e1f5c38cde69964c7";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let executorInstance: Executor;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let stateRootStorageInstance: StateRootStorage;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;
  let relayer1: HardhatEthersSigner;

  let resourceID: string;
  let depositData: string;
  let depositProposalData;

  let proposal: {
    originDomainID: number;
    securityModel: number;
    depositNonce: number;
    resourceID: string;
    data: string;
    storageProof: Array<string>;
  };

  beforeEach(async () => {
    [, depositorAccount, recipientAccount, relayer1] =
      await ethers.getSigners();

    [
      bridgeInstance,
      routerInstance,
      executorInstance,
      stateRootStorageInstance,
    ] = await deployBridgeContracts(destinationDomainID, routerAddress);
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
    resourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    await Promise.all([
      ERC20MintableInstance.mint(depositorAccount, initialTokenAmount),
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
    ]);

    await ERC20MintableInstance.connect(depositorAccount).approve(
      await ERC20HandlerInstance.getAddress(),
      depositAmount,
    );

    depositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );
    depositProposalData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );

    proposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: depositProposalData,
      resourceID: resourceID,
      storageProof: storageProof1[0].proof,
    };

    await stateRootStorageInstance.storeStateRoot(
      originDomainID,
      slot,
      stateRoot,
    );
  });

  it("[sanity] depositorAccount' balance should be equal to initialTokenAmount", async () => {
    const depositorBalance =
      await ERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(depositorBalance, BigInt(initialTokenAmount));
  });

  it("[sanity] await ERC20HandlerInstance.getAddress() should have an allowance of depositAmount from depositorAccount", async () => {
    const handlerAllowance = await ERC20MintableInstance.allowance(
      depositorAccount,
      await ERC20HandlerInstance.getAddress(),
    );
    assert.strictEqual(handlerAllowance, BigInt(depositAmount));
  });

  it("depositAmount of Destination ERC20 should be transferred to recipientAccount", async () => {
    // depositorAccount makes initial deposit of depositAmount
    assert.isFalse(await bridgeInstance.paused());
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

    // Handler should have a balance of depositAmount
    const handlerBalance = await ERC20MintableInstance.balanceOf(
      await ERC20HandlerInstance.getAddress(),
    );
    assert.strictEqual(handlerBalance, BigInt(depositAmount));

    // relayer2 executes the proposal
    await expect(
      executorInstance
        .connect(relayer1)
        .executeProposal(proposal, accountProof1, slot),
    ).not.to.be.reverted;

    // Assert ERC20 balance was transferred from depositorAccount
    const depositorBalance =
      await ERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(
      depositorBalance,
      BigInt(initialTokenAmount - depositAmount),
    );

    // // Assert ERC20 balance was transferred to recipientAccount
    const recipientBalance =
      await ERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance, BigInt(depositAmount));
  });
});
