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
import { assert } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  BasicFeeHandler,
  ERC20Handler,
  ERC20PresetMinterPauser,
  FeeHandlerRouter,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  createResourceID,
  createERCDepositData,
} from "../../../helpers";

describe("BasicFeeHandler - [calculateFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const feeData = "0x";
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const securityModel = 1;

  let bridgeInstance: Bridge;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let basicFeeHandlerInstance: BasicFeeHandler;
  let recipientAccount: HardhatEthersSigner;
  let relayer1: HardhatEthersSigner;

  let resourceID: string;
  let depositData: string;

  beforeEach(async () => {
    [, , recipientAccount, relayer1] = await ethers.getSigners();

    [bridgeInstance] = await deployBridgeContracts(
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

    depositData = createERCDepositData(
      100,
      20,
      await recipientAccount.getAddress(),
    );

    await Promise.all([
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID,
        await ERC20MintableInstance.getAddress(),
        emptySetResourceData,
      ),
      bridgeInstance.adminChangeFeeHandler(
        await feeHandlerRouterInstance.getAddress(),
      ),
      feeHandlerRouterInstance.adminSetResourceHandler(
        destinationDomainID,
        resourceID,
        securityModel,
        basicFeeHandlerInstance.getAddress(),
      ),
    ]);
  });

  it("should return amount of fee", async () => {
    // current fee is set to 0
    const response1 = await feeHandlerRouterInstance.calculateFee(
      relayer1.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );

    assert.deepEqual(ethers.formatEther(response1[0]), "0.0");
    // Change fee to 0.5 ether
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("0.5"),
    );
    const response2 = await feeHandlerRouterInstance.calculateFee(
      relayer1.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(ethers.formatEther(response2[0]), "0.5");
  });
});
