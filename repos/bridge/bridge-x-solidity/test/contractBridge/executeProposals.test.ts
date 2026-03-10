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
} from "../../typechain-types";
import { deployBridgeContracts, createERCDepositData } from "../helpers";
import { accountProof1, storageProof1 } from "../testingProofs";

describe("Bridge - [execute proposals]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const initialTokenAmount = 100;
  const depositAmount = 10;
  const expectedDepositNonces = [1, 2, 3];
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

  let erc20ResourceID: string;
  let erc20DepositData: string;
  let erc20DepositProposalData: string;
  let proposalsForExecution: Array<{
    originDomainID: number;
    securityModel: number;
    depositNonce: number;
    resourceID: string;
    data: string;
    storageProof: Array<string>;
  }>;

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

    erc20ResourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    await Promise.all([
      ERC20MintableInstance.mint(depositorAccount, initialTokenAmount),
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        erc20ResourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
    ]);

    await Promise.all([
      ERC20MintableInstance.connect(depositorAccount).approve(
        await ERC20HandlerInstance.getAddress(),
        depositAmount,
      ),
    ]);

    erc20DepositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );
    erc20DepositProposalData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );

    proposalsForExecution = [
      {
        originDomainID: originDomainID,
        securityModel: securityModel,
        depositNonce: expectedDepositNonces[0],
        resourceID: erc20ResourceID,
        data: erc20DepositProposalData,
        storageProof: storageProof1[0].proof,
      },
    ];

    await stateRootStorageInstance.storeStateRoot(
      originDomainID,
      slot,
      stateRoot,
    );
  });

  it("should create and execute executeProposal successfully", async () => {
    // depositorAccount makes initial deposit of depositAmount
    assert.isFalse(await bridgeInstance.paused());
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          originDomainID,
          erc20ResourceID,
          securityModel,
          erc20DepositData,
          feeData,
        ),
    ).not.to.be.reverted;

    const executeTx = await executorInstance
      .connect(relayer1)
      .executeProposals(proposalsForExecution, accountProof1, slot);

    await expect(executeTx).not.to.be.reverted;

    // check that deposit nonces had been marked as used in bitmap
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    expectedDepositNonces.map(async (_, index) => {
      assert.isTrue(
        await executorInstance.isProposalExecuted(
          originDomainID,
          expectedDepositNonces[index],
        ),
      );
    });

    // check that tokens are transferred to recipient address
    const recipientERC20Balance =
      await ERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientERC20Balance, BigInt(depositAmount));
  });

  it("should skip executing proposal if deposit nonce is already used", async () => {
    // depositorAccount makes initial deposit of depositAmount
    assert.isFalse(await bridgeInstance.paused());
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          originDomainID,
          erc20ResourceID,
          securityModel,
          erc20DepositData,
          feeData,
        ),
    ).not.to.be.reverted;

    const executeTx = await executorInstance.executeProposals(
      proposalsForExecution,
      accountProof1,
      slot,
    );

    await expect(executeTx).not.to.be.reverted;

    // check that deposit nonces had been marked as used in bitmap
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    expectedDepositNonces.map(async (_, index) => {
      assert.isTrue(
        await executorInstance.isProposalExecuted(
          originDomainID,
          expectedDepositNonces[index],
        ),
      );
    });

    // check that tokens are transferred to recipient address
    const recipientERC20Balance =
      await ERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientERC20Balance, BigInt(depositAmount));

    const skipExecuteTx = await executorInstance
      .connect(relayer1)
      .executeProposals(proposalsForExecution, accountProof1, slot);

    // check that no ProposalExecution events are emitted
    await expect(skipExecuteTx).not.to.emit(
      executorInstance,
      "ProposalExecution",
    );
  });

  it("should fail executing proposals if empty array is passed for execution", async () => {
    await expect(
      executorInstance
        .connect(relayer1)
        .executeProposals([], accountProof1, slot),
    ).to.be.revertedWithCustomError(executorInstance, "EmptyProposalsArray()");
  });

  it("executeProposal event should be emitted with expected values", async () => {
    // depositorAccount makes initial deposit of depositAmount
    assert.isFalse(await bridgeInstance.paused());
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          originDomainID,
          erc20ResourceID,
          securityModel,
          erc20DepositData,
          feeData,
        ),
    ).not.to.be.reverted;

    const executeTx = await executorInstance
      .connect(relayer1)
      .executeProposals(proposalsForExecution, accountProof1, slot);

    await expect(executeTx)
      .to.emit(executorInstance, "ProposalExecution")
      .withArgs(
        originDomainID,
        expectedDepositNonces[0],
        ethers.AbiCoder.defaultAbiCoder().encode(
          ["address", "address", "uint256"],
          [
            await ERC20MintableInstance.getAddress(),
            await recipientAccount.getAddress(),
            depositAmount,
          ],
        ),
      );

    // check that deposit nonces had been marked as used in bitmap
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    expectedDepositNonces.map(async (_, index) => {
      assert.isTrue(
        await executorInstance.isProposalExecuted(
          originDomainID,
          expectedDepositNonces[index],
        ),
      );
    });

    // check that tokens are transferred to recipient address
    const recipientERC20Balance =
      await ERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientERC20Balance, BigInt(depositAmount));
  });
});
