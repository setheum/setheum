// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  Router,
  ERC20Handler,
  ERC20PresetMinterPauser,
  ERC20Safe__factory,
} from "../../typechain-types";
import {
  createERCDepositData,
  createResourceID,
  deployBridgeContracts,
} from "../helpers";

describe("Bridge - [deposit - ERC20]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const originChainInitialTokenAmount = 100;
  const depositAmount = 10;
  const expectedDepositNonce = 1;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let ERC20MintableInstance1: ERC20PresetMinterPauser;
  let ERC20MintableInstance2: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let erc20SafeContract: ERC20Safe__factory;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;

  let resourceID1: string;
  let resourceID2: string;
  let depositData: string;

  beforeEach(async () => {
    [, depositorAccount, recipientAccount] = await ethers.getSigners();

    [bridgeInstance, routerInstance] = await deployBridgeContracts(
      originDomainID,
      routerAddress,
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    const ERC20PresetMinterPauserMock = await ethers.getContractFactory(
      "ERC20PresetMinterPauserMock",
    );
    ERC20MintableInstance1 = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20MintableInstance2 = await ERC20PresetMinterPauserMock.deploy(
      "Token",
      "TOK",
    );
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    resourceID1 = createResourceID(
      await ERC20MintableInstance1.getAddress(),
      originDomainID,
    );
    resourceID2 = createResourceID(
      await ERC20MintableInstance2.getAddress(),
      originDomainID,
    );

    await Promise.all([
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID1,
        await ERC20MintableInstance1.getAddress(),
        emptySetResourceData,
      ),
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID2,
        await ERC20MintableInstance2.getAddress(),
        emptySetResourceData,
      ),
      ERC20MintableInstance1.mint(
        depositorAccount,
        originChainInitialTokenAmount,
      ),
      ERC20MintableInstance2.mint(
        depositorAccount,
        originChainInitialTokenAmount,
      ),
    ]);
    await ERC20MintableInstance1.connect(depositorAccount).approve(
      await ERC20HandlerInstance.getAddress(),
      depositAmount * 2,
    );
    await ERC20MintableInstance2.connect(depositorAccount).approve(
      await ERC20HandlerInstance.getAddress(),
      depositAmount,
    );

    depositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );

    erc20SafeContract = await ethers.getContractFactory("ERC20Safe");
  });

  it("[sanity] test depositorAccount' balance", async () => {
    const originChainDepositorBalance =
      await ERC20MintableInstance1.balanceOf(depositorAccount);
    assert.strictEqual(
      originChainDepositorBalance,
      BigInt(originChainInitialTokenAmount),
    );
  });

  it("[sanity] test await ERC20HandlerInstance.getAddress()' allowance", async () => {
    const originChainHandlerAllowance = await ERC20MintableInstance1.allowance(
      depositorAccount,
      await ERC20HandlerInstance.getAddress(),
    );
    assert.strictEqual(originChainHandlerAllowance, BigInt(depositAmount * 2));
  });

  it("ERC20 deposit can be made", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID1,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;
  });

  it("should revert ERC20 deposit if Bridge is paused", async () => {
    assert.isFalse(await bridgeInstance.paused());
    await expect(bridgeInstance.adminPauseTransfers()).not.to.be.reverted;
    assert.isTrue(await bridgeInstance.paused());
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID1,
          securityModel,
          depositData,
          feeData,
        ),
    ).to.be.revertedWithCustomError(routerInstance, "BridgeIsPaused()");
  });

  it("_depositCounts should be increments from 0 to 1", async () => {
    await routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID1,
        securityModel,
        depositData,
        feeData,
      );

    const depositCount =
      await routerInstance._depositCounts(destinationDomainID);
    assert.strictEqual(depositCount, BigInt(expectedDepositNonce));
  });

  it("ERC20 can be deposited with correct balances", async () => {
    await routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID1,
        securityModel,
        depositData,
        feeData,
      );

    const originChainDepositorBalance =
      await ERC20MintableInstance1.balanceOf(depositorAccount);
    assert.strictEqual(
      originChainDepositorBalance,
      BigInt(originChainInitialTokenAmount - depositAmount),
    );

    const originChainHandlerBalance = await ERC20MintableInstance1.balanceOf(
      await ERC20HandlerInstance.getAddress(),
    );
    assert.strictEqual(originChainHandlerBalance, BigInt(depositAmount));
  });

  it("Deposit event is fired with expected value", async () => {
    const depositTx1 = routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID1,
        securityModel,
        depositData,
        feeData,
      );

    await expect(depositTx1)
      .to.emit(routerInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        resourceID1.toLowerCase(),
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        depositData.toLowerCase(),
      );

    const depositTx2 = routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID1,
        securityModel,
        depositData,
        feeData,
      );

    await expect(depositTx2)
      .to.emit(routerInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        resourceID1,
        expectedDepositNonce + 1,
        await depositorAccount.getAddress(),
        depositData.toLowerCase(),
      );
  });

  it("deposit requires resourceID that is mapped to a handler", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          ethers.zeroPadValue("0x01", 32),
          securityModel,
          depositData,
          feeData,
        ),
    ).to.be.rejectedWith("ResourceIDNotMappedToHandler()");
  });

  it("Deposit destination domain can not be current bridge domain ", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          originDomainID,
          resourceID1,
          securityModel,
          depositData,
          feeData,
        ),
    ).to.be.rejectedWith("DepositToCurrentDomain()");
  });

  it("should revert if ERC20Safe contract call fails", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID2,
          securityModel,
          depositData,
          feeData,
        ),
    ).to.be.revertedWithCustomError(erc20SafeContract, "ERC20OperationFailed");
  });
});
