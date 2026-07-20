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
  ERC20Safe__factory,
} from "../../../typechain-types";
import {
  deployBridgeContracts,
  createResourceID,
  createERCDepositData,
} from "../../helpers";

describe("ERC20Handler - [Deposit ERC20]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const expectedDepositNonce = 1;
  const tokenAmount = 100;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let erc20SafeContract: ERC20Safe__factory;
  let adminAccount: HardhatEthersSigner;
  let depositorAccount: HardhatEthersSigner;

  let resourceID: string;

  beforeEach(async () => {
    [adminAccount, depositorAccount] = await ethers.getSigners();

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

    resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      originDomainID,
    );

    await Promise.all([
      ERC20MintableInstance.connect(depositorAccount).approve(
        await ERC20HandlerInstance.getAddress(),
        tokenAmount,
      ),
      ERC20MintableInstance.mint(depositorAccount, tokenAmount),
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        // set decimal places for handler and token
        emptySetResourceData,
      ),
    ]);

    erc20SafeContract = await ethers.getContractFactory("ERC20Safe");
  });

  it("[sanity] depositor owns tokenAmount of ERC20", async () => {
    const depositorBalance =
      await ERC20MintableInstance.balanceOf(depositorAccount);
    assert.deepEqual(BigInt(tokenAmount), depositorBalance);
  });

  it("[sanity] await ERC20HandlerInstance.getAddress() has an allowance of tokenAmount from depositorAccount", async () => {
    const handlerAllowance = await ERC20MintableInstance.allowance(
      depositorAccount,
      await ERC20HandlerInstance.getAddress(),
    );
    assert.deepEqual(BigInt(tokenAmount), handlerAllowance);
  });

  it("Varied recipient address with length 40", async () => {
    const recipientAccount =
      (await adminAccount.getAddress()) +
      (await depositorAccount.getAddress()).substring(2);
    const lenRecipientAddress = 40;

    const depositTx = routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        createERCDepositData(
          tokenAmount,
          lenRecipientAddress,
          recipientAccount,
        ),
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
        createERCDepositData(
          tokenAmount,
          lenRecipientAddress,
          recipientAccount,
        ).toLowerCase(),
      );
  });

  it("Varied recipient address with length 32", async () => {
    const recipientAccount = ethers.keccak256(await adminAccount.getAddress());
    const lenRecipientAddress = 32;

    const depositTx = routerInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        resourceID,
        securityModel,
        createERCDepositData(
          tokenAmount,
          lenRecipientAddress,
          recipientAccount,
        ),
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
        createERCDepositData(
          tokenAmount,
          lenRecipientAddress,
          recipientAccount,
        ),
      );
  });

  it(`When non-contract addresses are whitelisted in the handler,
      deposits which the addresses are set as a token address will be failed`, async () => {
    const ZERO_Address = "0x0000000000000000000000000000000000000000";
    const EOA_Address = await depositorAccount.getAddress();
    const resourceID_ZERO_Address = createResourceID(
      ZERO_Address,
      originDomainID,
    );
    const resourceID_EOA_Address = createResourceID(
      EOA_Address,
      originDomainID,
    );
    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance.getAddress(),
      resourceID_ZERO_Address,
      ZERO_Address,
      emptySetResourceData,
    );
    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance.getAddress(),
      resourceID_EOA_Address,
      EOA_Address,
      emptySetResourceData,
    );

    const recipientAccount =
      (await adminAccount.getAddress()) +
      (await depositorAccount.getAddress()).substring(2);
    const lenRecipientAddress = 40;

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID_ZERO_Address,
          securityModel,
          createERCDepositData(
            tokenAmount,
            lenRecipientAddress,
            recipientAccount,
          ),
          feeData,
        ),
    ).to.be.revertedWithCustomError(erc20SafeContract, "ERC20NonContractCall");

    await expect(
      routerInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          resourceID_EOA_Address,
          securityModel,
          createERCDepositData(
            tokenAmount,
            lenRecipientAddress,
            recipientAccount,
          ),
          feeData,
        ),
    ).to.be.revertedWithCustomError(erc20SafeContract, "ERC20NonContractCall");
  });
});
