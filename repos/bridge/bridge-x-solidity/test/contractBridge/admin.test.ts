// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  Router,
  ERC20Handler,
  ERC20Handler__factory,
  ERC20PresetMinterPauser,
  ERC20PresetMinterPauser__factory,
  Executor,
} from "../../typechain-types";
import {
  deployBridgeContracts,
  createResourceID,
  createERCWithdrawData,
} from "../helpers";

// This test does NOT include all getter methods, just
// getters that should work with only the constructor called
describe("Bridge - [admin]", () => {
  const domainID = 1;

  const emptySetResourceData = "0x";
  const bytes32 = ethers.zeroPadValue("0x01", 32);
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let executorInstance: Executor;
  let ERC20MintableContract: ERC20PresetMinterPauser__factory;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerContract: ERC20Handler__factory;
  let ERC20HandlerInstance: ERC20Handler;
  let tokenOwnerAccount: HardhatEthersSigner;
  let nonAdminAccount: HardhatEthersSigner;
  let someAddress: HardhatEthersSigner;

  let withdrawData: string;

  beforeEach(async () => {
    [tokenOwnerAccount, nonAdminAccount, someAddress] =
      await ethers.getSigners();

    [bridgeInstance, routerInstance, executorInstance] =
      await deployBridgeContracts(domainID, routerAddress);
    ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20HandlerContract = await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
  });

  // Testing pausable methods
  it("[sanity] Bridge should not be paused after deployments", async () => {
    assert.isFalse(await bridgeInstance.paused());
  });

  it("Bridge should be paused after being paused by admin", async () => {
    await expect(bridgeInstance.adminPauseTransfers()).not.to.be.reverted;
    assert.isTrue(await bridgeInstance.paused());
  });

  it("Bridge should be unpaused after being unpaused by admin", async () => {
    await expect(bridgeInstance.adminPauseTransfers()).not.to.be.reverted;
    assert.isTrue(await bridgeInstance.paused());
    await expect(bridgeInstance.adminUnpauseTransfers()).not.to.be.reverted;
    assert.isFalse(await bridgeInstance.paused());
  });

  // Set Handler Address

  it("Should set a Resource ID for handler address", async () => {
    const resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      domainID,
    );

    assert.deepEqual(
      await bridgeInstance._resourceIDToHandlerAddress(resourceID),
      ethers.ZeroAddress,
    );

    await expect(
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
    ).not.to.be.reverted;
    assert.deepEqual(
      await bridgeInstance._resourceIDToHandlerAddress(resourceID),
      await ERC20HandlerInstance.getAddress(),
    );
  });

  // Set resource ID

  it("Should set a ERC20 Resource ID and contract address", async () => {
    const ERC20MintableInstance = await ERC20MintableContract.deploy(
      "token",
      "TOK",
    );
    const resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      domainID,
    );
    const ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    await expect(
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
    ).not.to.be.reverted;
    assert.deepEqual(
      await ERC20HandlerInstance._resourceIDToTokenContractAddress(resourceID),
      await ERC20MintableInstance.getAddress(),
    );

    const retrievedResourceID = (
      await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance.getAddress(),
      )
    ).resourceID;

    assert.deepEqual(retrievedResourceID, resourceID);
  });

  it("Should require admin role to set a ERC20 Resource ID and contract address", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminSetResource(
          someAddress,
          bytes32,
          someAddress,
          emptySetResourceData,
        ),
    ).to.be.revertedWithCustomError(
      bridgeInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  // Set Generic Resource

  it("Should require admin role to set a Generic Resource ID and contract address", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminSetResource(
          someAddress,
          bytes32,
          someAddress,
          emptySetResourceData,
        ),
    ).to.be.revertedWithCustomError(
      bridgeInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  // Set burnable

  it("Should set await ERC20MintableInstance.getAddress() as burnable", async () => {
    const ERC20MintableInstance = await ERC20MintableContract.deploy(
      "Token",
      "TOK",
    );
    const resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      domainID,
    );
    const ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    await expect(
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
    ).not.to.be.reverted;
    await expect(
      bridgeInstance.adminSetBurnable(
        await ERC20HandlerInstance.getAddress(),
        await ERC20MintableInstance.getAddress(),
      ),
    ).not.to.be.reverted;
    const isBurnable = (
      await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance.getAddress(),
      )
    ).isBurnable;

    assert.isTrue(isBurnable, "Contract wasn't successfully marked burnable");
  });

  it("Should require admin role to set await ERC20MintableInstance.getAddress() as burnable", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminSetBurnable(someAddress, someAddress),
    ).to.be.revertedWithCustomError(
      bridgeInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  // Withdraw

  it("Should withdraw funds", async () => {
    const numTokens = 10;

    let ownerBalance;

    const ERC20MintableInstance = await ERC20MintableContract.deploy(
      "token",
      "TOK",
    );
    const resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      domainID,
    );
    const ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    await expect(
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
    ).not.to.be.reverted;

    await ERC20MintableInstance.mint(tokenOwnerAccount, numTokens);
    ownerBalance = await ERC20MintableInstance.balanceOf(tokenOwnerAccount);
    assert.deepEqual(ownerBalance, BigInt(numTokens));

    await ERC20MintableInstance.transfer(
      await ERC20HandlerInstance.getAddress(),
      numTokens,
    );

    ownerBalance = await ERC20MintableInstance.balanceOf(tokenOwnerAccount);
    assert.deepEqual(ownerBalance, BigInt(0));
    const handlerBalance = await ERC20MintableInstance.balanceOf(
      await ERC20HandlerInstance.getAddress(),
    );
    assert.deepEqual(handlerBalance, BigInt(numTokens));

    withdrawData = createERCWithdrawData(
      await ERC20MintableInstance.getAddress(),
      await tokenOwnerAccount.getAddress(),
      numTokens,
    );

    await bridgeInstance.adminWithdraw(
      await ERC20HandlerInstance.getAddress(),
      withdrawData,
    );
    ownerBalance = await ERC20MintableInstance.balanceOf(tokenOwnerAccount);
    assert.deepEqual(ownerBalance, BigInt(numTokens));
  });

  it("Should require admin role to withdraw funds", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminWithdraw(someAddress, "0x01"),
    ).to.be.revertedWithCustomError(
      bridgeInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  // Nonces manipulation

  it("Should set nonce", async () => {
    const nonce = 3;
    await routerInstance
      .connect(tokenOwnerAccount)
      .adminSetDepositNonce(domainID, nonce);
    const nonceAfterSet = await routerInstance._depositCounts(domainID);
    assert.deepEqual(nonceAfterSet, BigInt(nonce));
  });

  it("Should require admin role to set nonce", async () => {
    await expect(
      routerInstance.connect(nonAdminAccount).adminSetDepositNonce(1, 3),
    ).to.be.revertedWithCustomError(
      routerInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  it("Should not allow for decrements of the nonce", async () => {
    const currentNonce = 3;
    await routerInstance
      .connect(tokenOwnerAccount)
      .adminSetDepositNonce(domainID, currentNonce);
    const newNonce = 2;
    await expect(
      routerInstance
        .connect(tokenOwnerAccount)
        .adminSetDepositNonce(domainID, newNonce),
    ).to.be.revertedWithCustomError(
      routerInstance,
      "NonceDecrementNotAllowed(uint64)",
    );
  });

  it("Should require admin role to mark nonce as used", async () => {
    const nonce = 5;
    await expect(
      executorInstance
        .connect(nonAdminAccount)
        .adminMarkNonceAsUsed(domainID, nonce),
    ).to.be.revertedWithCustomError(
      executorInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  it("Should successfully mark nonce as used", async () => {
    const usedNonce = 5;
    await executorInstance
      .connect(tokenOwnerAccount)
      .adminMarkNonceAsUsed(domainID, usedNonce);
    const isNonceUsed = await executorInstance.isProposalExecuted(
      domainID,
      usedNonce,
    );

    assert.isTrue(isNonceUsed, "Nonce wasn't successfully marked as used");
  });

  // Change access control contract

  it("Should require admin role to change access control contract", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminChangeAccessControl(someAddress),
    ).to.be.revertedWithCustomError(
      bridgeInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  // change domain slot index

  it("Should require admin role to change domain slot index", async () => {
    await expect(
      executorInstance.connect(nonAdminAccount).adminChangeSlotIndex(3, 1),
    ).to.be.revertedWithCustomError(
      bridgeInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  it("Should successfully change domain slot index if called by admin", async () => {
    await expect(
      executorInstance.connect(tokenOwnerAccount).adminChangeSlotIndex(3, 1),
    ).not.to.be.reverted;
  });

  it("Should require admin role to change router address", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminChangeRouterAddress("0x388C818CA8B9251b393131C08a736A67ccB19297"),
    ).to.be.revertedWithCustomError(
      routerInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });

  it("Should require admin role to change executor address", async () => {
    await expect(
      bridgeInstance
        .connect(nonAdminAccount)
        .adminChangeExecutorAddress(
          "0x388C818CA8B9251b393131C08a736A67ccB19297",
        ),
    ).to.be.revertedWithCustomError(
      routerInstance,
      "AccessNotAllowed(address,bytes4)",
    );
  });
});
