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

import { expect } from "chai";
import { ethers } from "hardhat";

describe("Bridge/Router/Executor - [constructor]", () => {
  const domainID = 1;

  it("[sanity] should revert deploying Bridge contract if zero address is provided in constructor", async () => {
    const BridgeContract = await ethers.getContractFactory("Bridge");
    await expect(
      BridgeContract.deploy(domainID, ethers.ZeroAddress),
    ).to.be.revertedWithCustomError(BridgeContract, "ZeroAddressProvided");
  });

  it("[sanity] should revert deploying Router contract if zero address is provided in constructor", async () => {
    const RouterContract = await ethers.getContractFactory("Router");
    await expect(
      RouterContract.deploy(ethers.ZeroAddress, ethers.ZeroAddress),
    ).to.be.revertedWithCustomError(RouterContract, "ZeroAddressProvided");
  });

  it("[sanity] should revert deploying Executor contract if zero address is provided in constructor", async () => {
    const ExecutorContract = await ethers.getContractFactory("Executor");
    await expect(
      ExecutorContract.deploy(ethers.ZeroAddress, ethers.ZeroAddress),
    ).to.be.revertedWithCustomError(ExecutorContract, "ZeroAddressProvided");
  });
});
