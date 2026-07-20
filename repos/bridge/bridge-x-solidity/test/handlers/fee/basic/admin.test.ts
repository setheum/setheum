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
import { ZeroAddress } from "ethers";
import { ethers } from "hardhat";

import type {
  BasicFeeHandler,
  Bridge,
  FeeHandlerRouter,
  ERC20PresetMinterPauser,
} from "../../../../typechain-types";
import { createResourceID, deployBridgeContracts } from "../../../helpers";

describe("BasicFeeHandler - [admin]", () => {
  const originDomainID = 1;
  const destinationDomainID = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const securityModel = 1;

  let bridgeInstance: Bridge;
  let basicFeeHandlerInstance: BasicFeeHandler;
  let originERC20MintableInstance: ERC20PresetMinterPauser;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let currentFeeHandlerAdmin: HardhatEthersSigner;
  let newBasicFeeHandlerAdmin: HardhatEthersSigner;
  let nonAdminAccount: HardhatEthersSigner;

  let ADMIN_ROLE: string;
  let resourceID: string;

  beforeEach(async () => {
    [currentFeeHandlerAdmin, newBasicFeeHandlerAdmin, nonAdminAccount] =
      await ethers.getSigners();

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
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    originERC20MintableInstance = await ERC20MintableContract.deploy(
      "token",
      "TOK",
    );

    ADMIN_ROLE = await basicFeeHandlerInstance.DEFAULT_ADMIN_ROLE();
    resourceID = createResourceID(
      await originERC20MintableInstance.getAddress(),
      originDomainID,
    );
  });

  it("should set fee property", async () => {
    const fee = 3;
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        securityModel,
      ),
      BigInt(0),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      fee,
    );
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        securityModel,
      ),
      BigInt(fee),
    );
  });

  it("should set fee properties for different security models", async () => {
    const fee = 3;
    const secondFee = 5;
    const secondSecurityModel = 2;

    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        securityModel,
      ),
      BigInt(0),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      fee,
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      secondSecurityModel,
      secondFee,
    );
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        securityModel,
      ),
      BigInt(fee),
    );
    assert.deepEqual(
      await basicFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        secondSecurityModel,
      ),
      BigInt(secondFee),
    );
  });

  it("should require admin role to change fee property", async () => {
    const fee = 3;
    await expect(
      basicFeeHandlerInstance
        .connect(nonAdminAccount)
        .changeFee(destinationDomainID, resourceID, securityModel, fee),
    ).to.be.revertedWithCustomError(basicFeeHandlerInstance, "SenderNotAdmin");
  });

  it("BasicFeeHandler admin should be changed to newBasicFeeHandlerAdmin", async () => {
    // check current admin
    assert.isTrue(
      await basicFeeHandlerInstance.hasRole(ADMIN_ROLE, currentFeeHandlerAdmin),
    );

    await expect(basicFeeHandlerInstance.renounceAdmin(newBasicFeeHandlerAdmin))
      .not.to.be.reverted;
    assert.isTrue(
      await basicFeeHandlerInstance.hasRole(
        ADMIN_ROLE,
        newBasicFeeHandlerAdmin,
      ),
    );

    // check that former admin is no longer admin
    assert.isFalse(
      await basicFeeHandlerInstance.hasRole(ADMIN_ROLE, currentFeeHandlerAdmin),
    );
  });

  it("BasicFeeHandler should not allow for renounced admin to be zero address", async () => {
    await expect(
      basicFeeHandlerInstance.renounceAdmin(ZeroAddress),
    ).to.be.revertedWithCustomError(
      basicFeeHandlerInstance,
      "ZeroAddressProvided()",
    );
  });
});
