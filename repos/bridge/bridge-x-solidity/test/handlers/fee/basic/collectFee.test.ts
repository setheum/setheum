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

describe("BasicFeeHandler - [collectFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const depositAmount = 10;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const expectedDepositNonce = 1;
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let basicFeeHandlerInstance: BasicFeeHandler;
  let ERC20HandlerInstance: ERC20Handler;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;
  let erc20ResourceID: string;
  let erc20depositData: string;

  beforeEach(async () => {
    [, depositorAccount, recipientAccount] = await ethers.getSigners();

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
    erc20ResourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      originDomainID,
    );

    await Promise.all([
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        erc20ResourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
      ERC20MintableInstance.mint(depositorAccount, depositAmount),
      ERC20MintableInstance.connect(depositorAccount).approve(
        await ERC20HandlerInstance.getAddress(),
        depositAmount,
      ),
      bridgeInstance.adminChangeFeeHandler(
        await feeHandlerRouterInstance.getAddress(),
      ),
      feeHandlerRouterInstance.adminSetResourceHandler(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
        await basicFeeHandlerInstance.getAddress(),
      ),
    ]);

    erc20depositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );
  });

  it("[sanity] ERC20 deposit can be made", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
          erc20depositData,
          feeData,
        ),
    ).not.to.be.reverted;
  });

  it("deposit should revert if invalid fee amount supplied", async () => {
    // current fee is set to 0
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
      ),
      BigInt(0),
    );
    const incorrectFee = ethers.parseEther("1.0");

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
          erc20depositData,
          feeData,
          {
            value: incorrectFee,
          },
        ),
    )
      .to.be.revertedWithCustomError(
        basicFeeHandlerInstance,
        "IncorrectFeeSupplied(uint256)",
      )
      .withArgs(incorrectFee);
  });

  it("deposit should pass if valid fee amount supplied for ERC20 deposit", async () => {
    const fee = ethers.parseEther("0.5");
    // current fee is set to 0
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
      ),
      BigInt(0),
    );
    // Change fee to 0.5 ether
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      erc20ResourceID,
      securityModel,
      fee,
    );
    assert.deepEqual(
      ethers.formatEther(
        await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
        ),
      ),
      "0.5",
    );

    const balanceBefore = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );

    const depositTx = routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
        erc20depositData,
        feeData,
        {
          value: fee,
        },
      );

    await expect(depositTx)
      .to.emit(routerInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        erc20ResourceID.toLowerCase(),
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        erc20depositData.toLowerCase(),
      );

    await expect(depositTx)
      .to.emit(basicFeeHandlerInstance, "FeeCollected")
      .withArgs(
        await depositorAccount.getAddress(),
        originDomainID,
        destinationDomainID,
        erc20ResourceID.toLowerCase(),
        fee,
        ethers.ZeroAddress,
      );

    const balanceAfter = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(
      balanceAfter,
      BigInt(fee.toString()) + BigInt(balanceBefore),
    );
  });

  it("deposit should revert if fee handler not set and fee supplied", async () => {
    await bridgeInstance.adminChangeFeeHandler(
      "0x0000000000000000000000000000000000000000",
    );

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
          erc20depositData,
          feeData,
          {
            value: ethers.parseEther("1.0"),
          },
        ),
    ).to.be.revertedWithCustomError(routerInstance, "MsgValueNotZero");
  });

  it("deposit should pass if fee handler not set and fee not supplied", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
          erc20depositData,
          feeData,
        ),
    ).not.to.be.reverted;
  });

  it("deposit should revert if not called by router on BasicFeeHandler contract", async () => {
    const fee = ethers.parseEther("0.5");
    await bridgeInstance.adminChangeFeeHandler(
      await basicFeeHandlerInstance.getAddress(),
    );
    // current fee is set to 0
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
      ),
      BigInt(0),
    );
    // Change fee to 0.5 ether
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      erc20ResourceID,
      securityModel,
      fee,
    );
    assert.deepEqual(
      ethers.formatEther(
        await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
        ),
      ),
      "0.5",
    );

    const balanceBefore = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );

    await expect(
      basicFeeHandlerInstance
        .connect(depositorAccount)
        .collectFee(
          depositorAccount,
          originDomainID,
          destinationDomainID,
          erc20ResourceID,
          securityModel,
          erc20depositData,
          feeData,
          {
            value: ethers.parseEther("0.5"),
          },
        ),
    ).to.be.revertedWithCustomError(
      basicFeeHandlerInstance,
      "SenderNotBridgeOrRouter",
    );

    const balanceAfter = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balanceAfter, balanceBefore);
  });

  it("deposit should revert if not called by bridge on FeeHandlerRouter contract", async () => {
    const fee = ethers.parseEther("0.5");
    await bridgeInstance.adminChangeFeeHandler(
      await basicFeeHandlerInstance.getAddress(),
    );
    // current fee is set to 0
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
      ),
      BigInt(0),
    );
    // Change fee to 0.5 ether
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      erc20ResourceID,
      securityModel,
      fee,
    );
    assert.deepEqual(
      ethers.formatEther(
        await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
        ),
      ),
      "0.5",
    );

    const balanceBefore = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );

    await expect(
      feeHandlerRouterInstance
        .connect(depositorAccount)
        .collectFee(
          depositorAccount,
          originDomainID,
          destinationDomainID,
          erc20ResourceID,
          securityModel,
          erc20depositData,
          feeData,
          {
            value: ethers.parseEther("0.5"),
          },
        ),
    ).to.be.revertedWithCustomError(
      feeHandlerRouterInstance,
      "SenderNotRouterContract",
    );

    const balanceAfter = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balanceAfter, balanceBefore);
  });

  it("should successfully change fee handler from FeeRouter to basicFeeHandlerInstance and collect fee", async () => {
    await bridgeInstance.adminChangeFeeHandler(
      await basicFeeHandlerInstance.getAddress(),
    );

    const fee = ethers.parseEther("0.5");
    // current fee is set to 0
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
      ),
      BigInt(0),
    );
    // Change fee to 0.5 ether
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      erc20ResourceID,
      securityModel,
      fee,
    );
    assert.deepEqual(
      ethers.formatEther(
        await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
          destinationDomainID,
          erc20ResourceID,
          securityModel,
        ),
      ),
      "0.5",
    );

    const balanceBefore = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );

    const depositTx = routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        erc20ResourceID,
        securityModel,
        erc20depositData.toLowerCase(),
        feeData,
        {
          value: fee,
        },
      );

    await expect(depositTx)
      .to.emit(routerInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        erc20ResourceID,
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        erc20depositData.toLowerCase(),
      );

    await expect(depositTx)
      .to.emit(basicFeeHandlerInstance, "FeeCollected")
      .withArgs(
        await depositorAccount.getAddress(),
        originDomainID,
        destinationDomainID,
        erc20ResourceID,
        fee,
        ethers.ZeroAddress,
      );

    const balanceAfter = await ethers.provider.getBalance(
      await basicFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(
      balanceAfter,
      BigInt(fee.toString()) + BigInt(balanceBefore),
    );
  });
});
