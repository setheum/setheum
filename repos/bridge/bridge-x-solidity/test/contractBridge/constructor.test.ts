// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

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
