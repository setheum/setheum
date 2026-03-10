// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
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

describe("PercentageFeeHandler - [calculateFee]", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const securityModel = 1;

  let bridgeInstance: Bridge;
  let percentageFeeHandlerInstance: PercentageERC20FeeHandlerEVM;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let recipientAccount: HardhatEthersSigner;
  let relayer: HardhatEthersSigner;

  let resourceID: string;

  beforeEach(async () => {
    [, recipientAccount, relayer] = await ethers.getSigners();

    [bridgeInstance] = await deployBridgeContracts(
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
      bridgeInstance.adminChangeFeeHandler(
        await feeHandlerRouterInstance.getAddress(),
      ),
      feeHandlerRouterInstance.adminSetResourceHandler(
        destinationDomainID,
        resourceID,
        securityModel,
        percentageFeeHandlerInstance.getAddress(),
      ),
    ]);
  });

  it(`should return percentage of token amount for fee if bounds
      are set [lowerBound > 0, upperBound > 0]`, async () => {
    const depositData = createERCDepositData(
      100000000,
      20,
      await recipientAccount.getAddress(),
    );

    // current fee is set to 0
    let response = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );

    assert.deepEqual(response[0].toString(), "0");
    // Change fee to 1 BPS ()
    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      10000,
    );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 100, 300000);
    response = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(response[0].toString(), "10000");
  });

  it(`should return percentage of token amount for fee if bounds
      are not set [lowerBound = 0, upperBound = 0]`, async () => {
    const depositData = createERCDepositData(
      100000000,
      20,
      await recipientAccount.getAddress(),
    );

    // current fee is set to 0
    const response1 = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );

    assert.deepEqual(response1[0].toString(), "0");
    // Change fee to 1 BPS ()
    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      10000,
    );
    const response2 = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(response2[0].toString(), "10000");
  });

  it("should return lower bound token amount for fee [lowerBound > 0, upperBound > 0]", async () => {
    const depositData = createERCDepositData(
      10000,
      20,
      await recipientAccount.getAddress(),
    );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 100, 300);
    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      10000,
    );

    const response = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(response[0].toString(), "100");
  });

  it("should return lower bound token amount for fee [lowerBound > 0, upperBound = 0]", async () => {
    const depositData = createERCDepositData(
      10000,
      20,
      await recipientAccount.getAddress(),
    );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 100, 0);
    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      10000,
    );

    const response = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(response[0].toString(), "100");
  });

  it("should return upper bound token amount for fee [lowerBound = 0, upperBound > 0]", async () => {
    const depositData = createERCDepositData(
      100000000,
      20,
      await recipientAccount.getAddress(),
    );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 0, 300);
    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      10000,
    );

    const response = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(response[0].toString(), "300");
  });

  it("should return percentage of token amount for fee [lowerBound = 0, upperBound > 0]", async () => {
    const depositData = createERCDepositData(
      100000,
      20,
      await recipientAccount.getAddress(),
    );
    await percentageFeeHandlerInstance.changeFeeBounds(resourceID, 0, 300);
    await percentageFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      10000,
    );

    const response = await feeHandlerRouterInstance.calculateFee(
      relayer.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.deepEqual(response[0].toString(), "10");
  });
});
