// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20PresetMinterPauser,
  FeeHandlerRouter,
  PercentageERC20FeeHandlerEVM,
  PercentageERC20FeeHandlerEVM__factory,
} from "../../../../typechain-types";
import { deployBridgeContracts, createResourceID } from "../../../helpers";

describe("PercentageFeeHandler - [change fee and bounds]", () => {
  const originDomainID = 1;
  const destinationDomainID = 1;
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const securityModel = 1;

  let bridgeInstance: Bridge;
  let percentageFeeHandlerInstance: PercentageERC20FeeHandlerEVM;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let PercentageFeeHandlerContract: PercentageERC20FeeHandlerEVM__factory;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let nonAdminAddress: HardhatEthersSigner;
  let resourceID: string;

  beforeEach(async () => {
    [, nonAdminAddress] = await ethers.getSigners();

    [bridgeInstance] = await deployBridgeContracts(
      originDomainID,
      routerAddress,
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance = await ERC20MintableContract.deploy("Token", "TOK");
    const FeeHandlerRouterContract =
      await ethers.getContractFactory("FeeHandlerRouter");
    feeHandlerRouterInstance = await FeeHandlerRouterContract.deploy(
      await bridgeInstance.getAddress(),
    );
    PercentageFeeHandlerContract = await ethers.getContractFactory(
      "PercentageERC20FeeHandlerEVM",
    );
    percentageFeeHandlerInstance = await PercentageFeeHandlerContract.deploy(
      await bridgeInstance.getAddress(),
      await feeHandlerRouterInstance.getAddress(),
    );

    resourceID = createResourceID(
      await ERC20MintableInstance.getAddress(),
      originDomainID,
    );
  });

  it("[sanity] contract should be deployed successfully", async () => {
    expect(await percentageFeeHandlerInstance.getAddress()).not.to.be.undefined;
  });

  it("should set fee", async () => {
    const fee = ethers.parseUnits("25");
    const changeFeeTx = await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      fee,
    );

    await expect(changeFeeTx)
      .to.emit(percentageFeeHandlerInstance, "FeeChanged")
      .withArgs(BigInt(fee));
    const newFee =
      await percentageFeeHandlerInstance._domainResourceIDSecurityModelToFee(
        destinationDomainID,
        resourceID,
        securityModel,
      );
    assert.deepEqual(ethers.formatUnits(newFee), "25.0");
  });

  it("should not set the same fee", async () => {
    await expect(
      percentageFeeHandlerInstance.changeFee(
        destinationDomainID,
        resourceID,
        securityModel,
        0,
      ),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "NewFeeEqualsCurrentFee",
    );
  });

  it("should require admin role to change fee", async () => {
    await expect(
      percentageFeeHandlerInstance
        .connect(nonAdminAddress)
        .changeFee(destinationDomainID, resourceID, securityModel, 1),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "SenderNotAdmin",
    );
  });

  it("should set fee bounds", async () => {
    const changeFeeBoundsTx =
      await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 50, 100);

    await expect(changeFeeBoundsTx)
      .to.emit(percentageFeeHandlerInstance, "FeeBoundsChanged")
      .withArgs("50", "100");
    const newLowerBound = (
      await percentageFeeHandlerInstance._resourceIDToFeeBounds(resourceID)
    ).lowerBound;
    const newUpperBound = (
      await percentageFeeHandlerInstance._resourceIDToFeeBounds(resourceID)
    ).upperBound;
    assert.deepEqual(newLowerBound.toString(), "50");
    assert.deepEqual(newUpperBound.toString(), "100");
  });

  it("should not set the same fee bounds", async () => {
    const percentageFeeHandlerInstance =
      await PercentageFeeHandlerContract.deploy(
        await bridgeInstance.getAddress(),
        await feeHandlerRouterInstance.getAddress(),
      );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 25, 50);
    await expect(
      percentageFeeHandlerInstance.changeFeeBounds(resourceID, 25, 50),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "NewBoundsEqualCurrentBounds",
    );
  });

  it("should fail to set lower bound larger than upper bound", async () => {
    const percentageFeeHandlerInstance =
      await PercentageFeeHandlerContract.deploy(
        await bridgeInstance.getAddress(),
        await feeHandlerRouterInstance.getAddress(),
      );
    await expect(
      percentageFeeHandlerInstance.changeFeeBounds(resourceID, 50, 25),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "InvalidBoundsRatio",
    );
  });

  it("should set only lower bound", async () => {
    const newLowerBound = 30;
    const percentageFeeHandlerInstance =
      await PercentageFeeHandlerContract.deploy(
        await bridgeInstance.getAddress(),
        await feeHandlerRouterInstance.getAddress(),
      );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 25, 50);
    await percentageFeeHandlerInstance.changeFeeBounds(
      resourceID,
      newLowerBound,
      50,
    );
    const currentLowerBound = (
      await percentageFeeHandlerInstance._resourceIDToFeeBounds(resourceID)
    ).lowerBound;
    assert.deepEqual(currentLowerBound, BigInt(newLowerBound));
  });

  it("should set only upper bound", async () => {
    const newUpperBound = 100;
    const percentageFeeHandlerInstance =
      await PercentageFeeHandlerContract.deploy(
        await bridgeInstance.getAddress(),
        await feeHandlerRouterInstance.getAddress(),
      );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 25, 50);
    await percentageFeeHandlerInstance.changeFeeBounds(
      resourceID,
      25,
      newUpperBound,
    );
    const currentUpperBound = (
      await percentageFeeHandlerInstance._resourceIDToFeeBounds(resourceID)
    ).upperBound;
    assert.deepEqual(BigInt(newUpperBound), currentUpperBound);
  });

  it("should require admin role to change fee bunds", async () => {
    const percentageFeeHandlerInstance =
      await PercentageFeeHandlerContract.deploy(
        await bridgeInstance.getAddress(),
        await feeHandlerRouterInstance.getAddress(),
      );
    await expect(
      percentageFeeHandlerInstance
        .connect(nonAdminAddress)
        .changeFeeBounds(resourceID, 50, 100),
    ).to.be.revertedWithCustomError(
      percentageFeeHandlerInstance,
      "SenderNotAdmin",
    );
  });
});
