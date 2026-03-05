// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import {
  impersonateAccount,
  setBalance,
} from "@nomicfoundation/hardhat-network-helpers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  BasicFeeHandler,
  Bridge,
  ERC20PresetMinterPauser,
  FeeHandlerRouter,
  Router,
} from "../../../typechain-types";
import {
  createResourceID,
  createERCDepositData,
  deployBridgeContracts,
} from "../../helpers";

describe("FeeHandlerRouter", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;
  const feeData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";
  const securityModel = 1;

  let bridgeInstance: Bridge;
  let routerInstance: Router;
  let ERC20MintableInstance: ERC20PresetMinterPauser;
  let feeHandlerRouterInstance: FeeHandlerRouter;
  let basicFeeHandlerInstance: BasicFeeHandler;
  let feeHandlerAccount: HardhatEthersSigner;
  let nonAdminAccount: HardhatEthersSigner;
  let whitelistedAccount: HardhatEthersSigner;
  let nonWhitelistedAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;

  let resourceID: string;

  beforeEach(async () => {
    [
      ,
      recipientAccount,
      feeHandlerAccount,
      nonAdminAccount,
      whitelistedAccount,
      nonWhitelistedAccount,
    ] = await ethers.getSigners();

    [bridgeInstance, routerInstance] = await deployBridgeContracts(
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
  });

  it("should successfully set handler to resourceID", async () => {
    assert.deepEqual(
      await feeHandlerRouterInstance._domainResourceIDSecurityModelToFeeHandlerAddress(
        destinationDomainID,
        resourceID,
        securityModel,
      ),
      "0x0000000000000000000000000000000000000000",
    );
    await feeHandlerRouterInstance.adminSetResourceHandler(
      destinationDomainID,
      resourceID,
      securityModel,
      feeHandlerAccount.getAddress(),
    );
    const newFeeHandler =
      await feeHandlerRouterInstance._domainResourceIDSecurityModelToFeeHandlerAddress(
        destinationDomainID,
        resourceID,
        securityModel,
      );
    assert.deepEqual(newFeeHandler, await feeHandlerAccount.getAddress());
  });

  it("should require admin role to set handler for resourceID", async () => {
    await expect(
      feeHandlerRouterInstance
        .connect(nonAdminAccount)
        .adminSetResourceHandler(
          destinationDomainID,
          resourceID,
          securityModel,
          feeHandlerAccount.getAddress(),
        ),
    ).to.be.revertedWithCustomError(feeHandlerRouterInstance, "SenderNotAdmin");
  });

  it("should successfully set whitelist on an address", async () => {
    assert.equal(
      await feeHandlerRouterInstance._whitelist(
        await whitelistedAccount.getAddress(),
      ),
      false,
    );

    const whitelistTx = await feeHandlerRouterInstance.adminSetWhitelist(
      await whitelistedAccount.getAddress(),
      true,
    );
    assert.equal(
      await feeHandlerRouterInstance._whitelist(
        await whitelistedAccount.getAddress(),
      ),
      true,
    );
    await expect(whitelistTx)
      .to.emit(feeHandlerRouterInstance, "WhitelistChanged")
      .withArgs(await whitelistedAccount.getAddress(), true);
  });

  it("should require admin role to set whitelist address", async () => {
    await expect(
      feeHandlerRouterInstance
        .connect(nonAdminAccount)
        .adminSetWhitelist(await nonWhitelistedAccount.getAddress(), true),
    ).to.be.revertedWithCustomError(feeHandlerRouterInstance, "SenderNotAdmin");
  });

  it("should return fee 0 if address whitelisted", async () => {
    await feeHandlerRouterInstance.adminSetWhitelist(
      await whitelistedAccount.getAddress(),
      true,
    );
    await feeHandlerRouterInstance.adminSetResourceHandler(
      destinationDomainID,
      resourceID,
      securityModel,
      await basicFeeHandlerInstance.getAddress(),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("0.5"),
    );

    const depositData = createERCDepositData(
      100,
      20,
      await recipientAccount.getAddress(),
    );
    const { fee: feeBefore } = await feeHandlerRouterInstance.calculateFee(
      await whitelistedAccount.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.equal(feeBefore, BigInt(0));
    const { fee: feeAfter } = await feeHandlerRouterInstance.calculateFee(
      await nonWhitelistedAccount.getAddress(),
      originDomainID,
      destinationDomainID,
      resourceID,
      securityModel,
      depositData,
      feeData,
    );
    assert.equal(feeAfter, ethers.parseEther("0.5"));
  });

  it("should revert if whitelisted address provides fee", async () => {
    // this is used to impersonate a call from Router contract
    // because ethersjs requires a signer to be passed into connect function
    const routerInstanceAddress = await routerInstance.getAddress();
    const routerContractSigner = await ethers.getSigner(routerInstanceAddress);
    await setBalance(routerInstanceAddress, 10n ** 18n);
    await impersonateAccount(routerInstanceAddress);

    await feeHandlerRouterInstance.adminSetWhitelist(
      await whitelistedAccount.getAddress(),
      true,
    );
    await feeHandlerRouterInstance.adminSetResourceHandler(
      destinationDomainID,
      resourceID,
      securityModel,
      await basicFeeHandlerInstance.getAddress(),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("0.5"),
    );

    const depositData = createERCDepositData(
      100,
      20,
      await recipientAccount.getAddress(),
    );
    await expect(
      feeHandlerRouterInstance
        .connect(routerContractSigner)
        .collectFee(
          await whitelistedAccount.getAddress(),
          originDomainID,
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
          {
            value: ethers.parseEther("0.5"),
          },
        ),
    ).to.be.revertedWithCustomError(
      feeHandlerRouterInstance,
      "IncorrectFeeSupplied(uint256)",
    );

    await expect(
      feeHandlerRouterInstance
        .connect(routerContractSigner)
        .collectFee(
          nonWhitelistedAccount.getAddress(),
          originDomainID,
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
          {
            value: ethers.parseEther("0.5"),
          },
        ),
    ).not.to.be.reverted;
  });

  it("should not collect fee from whitelisted address", async () => {
    // this is used to impersonate a call from Router contract
    // because ethersjs requires a signer to be passed into connect function
    const routerInstanceAddress = await routerInstance.getAddress();
    const routerContractSigner = await ethers.getSigner(routerInstanceAddress);
    await setBalance(routerInstanceAddress, 10n ** 18n);
    await impersonateAccount(routerInstanceAddress);

    await feeHandlerRouterInstance.adminSetWhitelist(
      await whitelistedAccount.getAddress(),
      true,
    );
    await feeHandlerRouterInstance.adminSetResourceHandler(
      destinationDomainID,
      resourceID,
      securityModel,
      await basicFeeHandlerInstance.getAddress(),
    );
    await basicFeeHandlerInstance.changeFee(
      destinationDomainID,
      resourceID,
      securityModel,
      ethers.parseEther("0.5"),
    );

    const depositData = createERCDepositData(
      100,
      20,
      await recipientAccount.getAddress(),
    );
    await expect(
      feeHandlerRouterInstance
        .connect(routerContractSigner)
        .collectFee(
          await whitelistedAccount.getAddress(),
          originDomainID,
          destinationDomainID,
          resourceID,
          securityModel,
          depositData,
          feeData,
          {
            value: "0",
          },
        ),
    ).not.to.be.reverted;
  });
});
