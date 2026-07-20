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
import { expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20Handler__factory,
  ERC20PresetMinterPauser,
} from "../../../typechain-types";
import {
  createERCDepositData,
  createResourceID,
  deployBridgeContracts,
} from "../../helpers";

describe("ERC20Handler - [constructor]", function () {
  const domainID = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const depositAmount = 10;

  let depositData: string;
  let resourceID: string;

  let bridgeInstance: Bridge;
  let ERC20HandlerContract: ERC20Handler__factory;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;

  beforeEach(async () => {
    [, depositorAccount, recipientAccount] = await ethers.getSigners();
    [bridgeInstance] = await deployBridgeContracts(domainID, routerAddress);

    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20HandlerContract = await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      domainID,
    );

    depositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );
  });

  it("[sanity] should revert if deposit is not called by Router", async () => {
    await expect(
      ERC20HandlerInstance.deposit(resourceID, depositorAccount, depositData),
    ).to.be.revertedWithCustomError(
      ERC20HandlerInstance,
      "SenderNotRouterContract()",
    );
  });

  it("[sanity] should revert if execution is not called by Executor", async () => {
    await expect(
      ERC20HandlerInstance.executeProposal(resourceID, depositData),
    ).to.be.revertedWithCustomError(
      ERC20HandlerInstance,
      "SenderNotExecutorContract()",
    );
  });

  it("[sanity] should revert if setResource is not called by Bridge", async () => {
    await expect(
      ERC20HandlerInstance.setResource(
        resourceID,
        await ERC20MintableInstance.getAddress(),
        "0x",
      ),
    ).to.be.revertedWithCustomError(
      ERC20HandlerInstance,
      "SenderNotBridgeContract()",
    );
  });

  it("[sanity] should revert if setBurnable is not called by Bridge", async () => {
    await expect(
      ERC20HandlerInstance.setBurnable(await ERC20HandlerInstance.getAddress()),
    ).to.be.revertedWithCustomError(
      ERC20HandlerInstance,
      "SenderNotBridgeContract()",
    );
  });

  it("[sanity] should revert if withdraw is not called by Bridge", async () => {
    await expect(
      ERC20HandlerInstance.withdraw("0x"),
    ).to.be.revertedWithCustomError(
      ERC20HandlerInstance,
      "SenderNotBridgeContract()",
    );
  });
});
