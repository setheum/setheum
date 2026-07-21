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
  FeeHandlerRouter,
  ERC20PresetMinterPauser,
} from "../../../../typechain-types";
import { createResourceID, deployBridgeContracts } from "../../../helpers";

describe("BasicFeeHandler - [changeFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const securityModel = 1;

  let bridgeInstance: Bridge;
  let basicFeeHandlerInstance: BasicFeeHandler;
  let originERC20MintableInstance: ERC20PresetMinterPauser;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let nonAdminAccount: HardhatEthersSigner;

  let resourceID: string;

  beforeEach(async () => {
    [, nonAdminAccount] = await ethers.getSigners();

    [bridgeInstance] = await deployBridgeContracts(
      originDomainID,
      routerAddress,
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
    const ERC20PresetMinterPauserContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    originERC20MintableInstance = await ERC20PresetMinterPauserContract.deploy(
      "token",
      "TOK",
    );

    resourceID = createResourceID(
      await originERC20MintableInstance.getAddress(),
      originDomainID,
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    originERC20MintableInstance = await ERC20MintableContract.deploy(
      "token",
      "TOK",
    );
    resourceID = createResourceID(
      await originERC20MintableInstance.getAddress(),
      originDomainID,
    );
  });

  it("[sanity] contract should be deployed successfully", async () => {
    expect(await basicFeeHandlerInstance.getAddress()).not.to.be.undefined;
  });

  it("should set fee", async () => {
    const fee = ethers.parseEther("0.05");
    const changeFeeTx = await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      fee,
    );

    await expect(changeFeeTx)
      .to.emit(basicFeeHandlerInstance, "FeeChanged")
      .withArgs(ethers.parseEther("0.05"));

    const newFee =
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        securityModel,
      );
    assert.deepEqual(ethers.formatEther(newFee), "0.05");
  });

  it("should not set the same fee", async () => {
    await expect(
      basicFeeHandlerInstance.changeFee(
        destinationDomainID,
        resourceID,
        securityModel,
        0,
      ),
    ).to.be.revertedWithCustomError(
      basicFeeHandlerInstance,
      "NewFeeEqualsCurrentFee",
    );
  });

  it("should require admin role to change fee", async () => {
    await expect(
      basicFeeHandlerInstance
        .connect(nonAdminAccount)
        .changeFee(destinationDomainID, resourceID, securityModel, 1),
    ).to.be.revertedWithCustomError(basicFeeHandlerInstance, "SenderNotAdmin");
  });
});
