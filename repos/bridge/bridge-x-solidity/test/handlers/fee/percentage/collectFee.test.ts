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
  FeeHandlerRouter,
  PercentageERC20FeeHandlerEVM,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  createResourceID,
  createERCDepositData,
} from "../../../helpers";

describe("PercentageFeeHandler - [collectFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const tokenAmount = ethers.parseEther("200000");

  const emptySetResourceData = "0x";
  const feeData = "0x";
  const feeBps = 60000; // BPS
  const fee = ethers.parseEther("120");
  const lowerBound = ethers.parseEther("100");
  const upperBound = ethers.parseEther("300");
  const expectedDepositNonce = 1;
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let percentageFeeHandlerInstance: PercentageERC20FeeHandlerEVM;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;

  let resourceID: string;
  let depositData: string;

  beforeEach(async () => {
    [, depositorAccount, recipientAccount] = await ethers.getSigners();

    [bridgeInstance, routerInstance] = await deployBridgeContracts(
      originDomainID,
      routerAddress,
    );
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    const FeeHandlerRouterContract =
      await ethers.getContractFactory("FeeHandlerRouter");
    feeHandlerRouterInstance = await FeeHandlerRouterContract.deploy(
      await bridgeInstance.getAddress(),
    );
    const PercentageERC20FeeHandlerEVMContract =
      await ethers.getContractFactory("PercentageERC20FeeHandlerEVM");
    percentageFeeHandlerInstance =
      await PercentageERC20FeeHandlerEVMContract.deploy(
        await bridgeInstance.getAddress(),
        await feeHandlerRouterInstance.getAddress(),
      );

    resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      originDomainID,
    );

    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      feeBps,
    );
    await percentageFeeHandlerInstance.changeFeeBounds(
      resourceID,
      lowerBound,
      upperBound,
    );

    await Promise.all([
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
      ERC20MintableInstance.mint(depositorAccount, tokenAmount + fee),
      ERC20MintableInstance.connect(depositorAccount).approve(
        await ERC20HandlerInstance.getAddress(),
        tokenAmount,
      ),
      ERC20MintableInstance.connect(depositorAccount).approve(
        await percentageFeeHandlerInstance.getAddress(),
        fee,
      ),
      bridgeInstance.adminChangeFeeHandler(
        await feeHandlerRouterInstance.getAddress(),
      ),
      feeHandlerRouterInstance.adminSetResourceHandler(
        destinationDomainID,
        resourceID,
        securityModel,
        await percentageFeeHandlerInstance.getAddress(),
      ),
    ]);

    depositData = createERCDepositData(
      tokenAmount.toString(),
      20,
      await recipientAccount.getAddress(),
    );
  });

  it("should collect fee in tokens", async () => {
    const balanceBefore = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );

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
        resourceID,
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        depositData.toLowerCase(),
      );

    await expect(depositTx)
      .to.emit(percentageFeeHandlerInstance, "FeeCollected")
      .withArgs(
        await depositorAccount.getAddress(),
        originDomainID,
        destinationDomainID,
        resourceID.toLocaleLowerCase(),
        fee,
        await ERC20MintableInstance.getAddress(),
      );

    const balanceAfter = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balanceAfter, fee + BigInt(balanceBefore));
  });

  it("deposit should revert if msg.value != 0", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
          {
            value: ethers.parseEther("0.5"),
          },
        ),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "MsgValueNotZero",
    );
  });

  it("deposit should revert if fee collection fails", async () => {
    const depositData = createERCDepositData(
      tokenAmount.toString(),
      20,
      await recipientAccount.getAddress(),
    );

    await ERC20MintableInstance.approve(
      await percentageFeeHandlerInstance.getAddress(),
      0,
    );
    await expect(
      routerInstance.deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        depositData,
        feeData,
        {
          value: ethers.parseEther("0.5").toString(),
        },
      ),
    ).to.be.reverted;
  });

  it("deposit should revert if not called by router on PercentageFeeHandler contract", async () => {
    const depositData = createERCDepositData(
      tokenAmount.toString(),
      20,
      await recipientAccount.getAddress(),
    );
    await ERC20MintableInstance.approve(
      await percentageFeeHandlerInstance.getAddress(),
      0,
    );
    await expect(
      percentageFeeHandlerInstance
        .connect(depositorAccount)
        .collectFee(
          depositorAccount,
          originDomainID,
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
          {
            value: ethers.parseEther("0.5").toString(),
          },
        ),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "SenderNotBridgeOrRouter",
    );
  });

  it("deposit should revert if not called by bridge on FeeHandlerRouter contract", async () => {
    const depositData = createERCDepositData(
      tokenAmount.toString(),
      20,
      await recipientAccount.getAddress(),
    );
    await ERC20MintableInstance.approve(
      await percentageFeeHandlerInstance.getAddress(),
      0,
    );
    await expect(
      feeHandlerRouterInstance
        .connect(depositorAccount)
        .collectFee(
          depositorAccount,
          originDomainID,
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
          {
            value: ethers.parseEther("0.5"),
          },
        ),
    ).to.be.revertedWithCustomError(
      feeHandlerRouterInstance,
      "SenderNotRouterContract",
    );
  });

  it("should successfully change fee handler from FeeRouter to PercentageFeeHandler and collect fee", async () => {
    await bridgeInstance.adminChangeFeeHandler(
      await percentageFeeHandlerInstance.getAddress(),
    );

    const balanceBefore = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );

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
        resourceID,
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        depositData.toLowerCase(),
      );

    await expect(depositTx)
      .to.emit(percentageFeeHandlerInstance, "FeeCollected")
      .withArgs(
        await depositorAccount.getAddress(),
        originDomainID,
        destinationDomainID,
        resourceID,
        fee,
        await ERC20MintableInstance.getAddress(),
      );

    const balanceAfter = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balanceAfter, fee + BigInt(balanceBefore));
  });
});
