// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  BasicFeeHandler,
  Bridge,
  Router,
  ERC20Handler,
  ERC20PresetMinterPauser,
  FeeHandlerRouter,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  createResourceID,
  createERCDepositData,
} from "../../../helpers";

describe("BasicFeeHandler - [distributeFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const depositAmount = 10;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let resourceID: string;
  let depositData: string;
  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let basicFeeHandlerInstance: BasicFeeHandler;
  let ERC20HandlerInstance: ERC20Handler;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount1: HardhatEthersSigner;
  let recipientAccount2: HardhatEthersSigner;
  let nonAdminAccount: HardhatEthersSigner;

  beforeEach(async () => {
    [
      ,
      depositorAccount,
      recipientAccount1,
      recipientAccount2,
      nonAdminAccount,
    ] = await ethers.getSigners();

    [bridgeInstance, routerInstance] = await deployBridgeContracts(
      originDomainID,
      routerAddress,
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
    const FeeHandlerRouterContract =
      await ethers.getContractFactory("FeeHandlerRouter");
    feeHandlerRouterInstance = await FeeHandlerRouterContract.deploy(
      await bridgeInstance.getAddress(),
    );
    const BasicFeeHandlerContract =
      await ethers.getContractFactory("BasicFeeHandler");
    basicFeeHandlerInstance = await BasicFeeHandlerContract.deploy(
      await bridgeInstance.getAddress(),
      await feeHandlerRouterInstance.getAddress(),
    );

    resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      originDomainID,
    );

    await Promise.all([
      ERC20MintableInstance.mint(depositorAccount, depositAmount),
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
      await recipientAccount1.getAddress(),
    );
  });

  it("should distribute fees", async () => {
    await bridgeInstance.adminChangeFeeHandler(
      basicFeeHandlerInstance.getAddress(),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("1.0"),
    );
    assert.deepEqual(
      ethers.formatEther(
        await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
          destinationDomainID,
          resourceID,
          securityModel,
        ),
      ),
      "1.0",
    );

    // check the balance is 0
    assert.deepEqual(
      ethers.formatEther(
        await ethers.provider.getBalance(await bridgeInstance.getAddress()),
      ),
      "0.0",
    );

    await routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        depositData,
        feeData,
        {
          value: ethers.parseEther("1.0"),
        },
      );
    assert.deepEqual(
      ethers.formatEther(
        await ethers.provider.getBalance(await bridgeInstance.getAddress()),
      ),
      "0.0",
    );
    assert.deepEqual(
      ethers.formatEther(
        await ethers.provider.getBalance(basicFeeHandlerInstance.getAddress()),
      ),
      "1.0",
    );

    const b1Before = await ethers.provider.getBalance(
      await depositorAccount.getAddress(),
    );
    const b2Before = await ethers.provider.getBalance(
      await recipientAccount1.getAddress(),
    );

    const payout = ethers.parseEther("0.5");
    // Transfer the funds
    const transferFeeTx = await basicFeeHandlerInstance.transferFee(
      [depositorAccount, recipientAccount1],
      [payout, payout],
    );

    await expect(transferFeeTx)
      .to.emit(basicFeeHandlerInstance, "FeeDistributed")
      .withArgs(
        ethers.ZeroAddress,
        await depositorAccount.getAddress(),
        payout,
      );

    await expect(transferFeeTx)
      .to.emit(basicFeeHandlerInstance, "FeeDistributed")
      .withArgs(
        ethers.ZeroAddress,
        await recipientAccount1.getAddress(),
        payout,
      );

    const balance1After = await ethers.provider.getBalance(depositorAccount);
    const balance2After = await ethers.provider.getBalance(recipientAccount1);
    assert.deepEqual(
      balance1After,
      BigInt(b1Before) + BigInt(payout.toString()),
    );
    assert.deepEqual(
      balance2After,
      BigInt(b2Before) + BigInt(payout.toString()),
    );
  });

  it("should require admin role to distribute fee", async () => {
    await bridgeInstance.adminChangeFeeHandler(
      basicFeeHandlerInstance.getAddress(),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("1.0"),
    );

    await routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        depositData,
        feeData,
        {
          value: ethers.parseEther("1.0"),
        },
      );

    assert.deepEqual(
      ethers.formatEther(
        await ethers.provider.getBalance(basicFeeHandlerInstance.getAddress()),
      ),
      "1.0",
    );

    const payout = ethers.parseEther("0.5");
    await expect(
      basicFeeHandlerInstance
        .connect(nonAdminAccount)
        .transferFee(
          [
            await recipientAccount1.getAddress(),
            recipientAccount2.getAddress(),
          ],
          [payout, payout],
        ),
    ).to.be.revertedWithCustomError(basicFeeHandlerInstance, "SenderNotAdmin");
  });

  it("should revert if addrs and amounts arrays have different length", async () => {
    await bridgeInstance.adminChangeFeeHandler(
      basicFeeHandlerInstance.getAddress(),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("1.0"),
    );

    await routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        depositData,
        feeData,
        {
          value: ethers.parseEther("1.0"),
        },
      );

    assert.deepEqual(
      ethers.formatEther(
        await ethers.provider.getBalance(basicFeeHandlerInstance.getAddress()),
      ),
      "1.0",
    );

    const payout = ethers.parseEther("0.5");
    await expect(
      basicFeeHandlerInstance.transferFee(
        [await recipientAccount1.getAddress(), recipientAccount2.getAddress()],
        [payout, payout, payout],
      ),
    ).to.be.revertedWithCustomError(
      basicFeeHandlerInstance,
      "AddressesAndAmountsArraysDifferentLength",
    );
  });
});
