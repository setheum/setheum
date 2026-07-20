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

describe("PercentageFeeHandler - [distributeFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const depositAmount = 100000;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const feeAmount = BigInt(30);
  const feeBps = 30000; // 3 BPS
  const payout = BigInt("10");
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let percentageFeeHandlerInstance: PercentageERC20FeeHandlerEVM;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount1: HardhatEthersSigner;
  let recipientAccount2: HardhatEthersSigner;
  let nonAdminAccount: HardhatEthersSigner;

  let resourceID: string;
  let depositData: string;

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
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
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

    await Promise.all([
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
      ERC20MintableInstance.mint(
        depositorAccount,
        BigInt(depositAmount) + feeAmount,
      ),
      ERC20MintableInstance.connect(depositorAccount).approve(
        await ERC20HandlerInstance.getAddress(),
        depositAmount,
      ),
      ERC20MintableInstance.connect(depositorAccount).approve(
        await percentageFeeHandlerInstance.getAddress(),
        depositAmount,
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
      percentageFeeHandlerInstance.changeFee(
        destinationDomainID,
        resourceID,
        securityModel,
        feeBps,
      ),
    ]);

    depositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount1.getAddress(),
    );
  });

  it("should distribute fees", async () => {
    // check the balance is 0
    const balance1Before = await ERC20MintableInstance.balanceOf(
      await recipientAccount1.getAddress(),
    );
    const balance2Before = await ERC20MintableInstance.balanceOf(
      await recipientAccount2.getAddress(),
    );

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;
    const balance = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balance, feeAmount);

    // Transfer the funds
    const transferFeeTx = await percentageFeeHandlerInstance.transferERC20Fee(
      resourceID,
      [
        await recipientAccount1.getAddress(),
        await recipientAccount2.getAddress(),
      ],
      [payout, payout],
    );

    await expect(transferFeeTx)
      .to.emit(percentageFeeHandlerInstance, "FeeDistributed")
      .withArgs(
        await ERC20MintableInstance.getAddress(),
        await recipientAccount1.getAddress(),
        payout,
      );

    await expect(transferFeeTx)
      .to.emit(percentageFeeHandlerInstance, "FeeDistributed")
      .withArgs(
        await ERC20MintableInstance.getAddress(),
        await recipientAccount2.getAddress(),
        payout,
      );

    const balance1After = await ERC20MintableInstance.balanceOf(
      await recipientAccount1.getAddress(),
    );
    const balance2After = await ERC20MintableInstance.balanceOf(
      await recipientAccount2.getAddress(),
    );
    assert.deepEqual(balance1After, payout + balance1Before);
    assert.deepEqual(balance2After, payout + balance2Before);
  });

  it("should not distribute fees with other resourceID", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;
    const balance = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balance, feeAmount);

    // Incorrect resourceID
    resourceID = createResourceID(
      await percentageFeeHandlerInstance.getAddress(),
      originDomainID,
    );

    // Transfer the funds: fails
    await expect(
      percentageFeeHandlerInstance.transferERC20Fee(
        resourceID,
        [
          await depositorAccount.getAddress(),
          await recipientAccount1.getAddress(),
        ],
        [payout, payout],
      ),
    ).to.be.reverted;
  });

  it("should require admin role to distribute fee", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;
    const balance = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balance, feeAmount);

    await expect(
      percentageFeeHandlerInstance
        .connect(nonAdminAccount)
        .transferERC20Fee(
          resourceID,
          [
            await depositorAccount.getAddress(),
            await recipientAccount1.getAddress(),
          ],
          [payout, payout],
        ),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "SenderNotAdmin",
    );
  });

  it("should revert if addrs and amounts arrays have different length", async () => {
    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
        ),
    ).not.to.be.reverted;
    const balance = await ERC20MintableInstance.balanceOf(
      await percentageFeeHandlerInstance.getAddress(),
    );
    assert.deepEqual(balance, feeAmount);

    await expect(
      percentageFeeHandlerInstance.transferERC20Fee(
        resourceID,
        [
          await depositorAccount.getAddress(),
          await recipientAccount1.getAddress(),
        ],
        [payout, payout, payout],
      ),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "AddressesAndAmountsArraysDifferentLength",
    );
  });
});
