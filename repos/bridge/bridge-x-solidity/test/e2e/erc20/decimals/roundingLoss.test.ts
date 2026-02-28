// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20PresetMinterPauser,
  Router,
  Executor,
  StateRootStorage,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  createERCDepositData,
  toHex,
  getDepositEventData,
} from "../../../helpers";
import {
  accountProof7,
  storageProof7,
  accountProof8,
  storageProof8,
} from "../../../testingProofs";

describe("E2E ERC20 - Two EVM Chains both with decimal places != 18 with rounding loss", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const originDecimalPlaces = BigInt(20);
  const destinationDecimalPlaces = BigInt(14);
  const bridgeDefaultDecimalPlaces = 18;
  const initialTokenAmount = ethers.parseUnits("100", originDecimalPlaces);
  const originDepositAmount = ethers.parseUnits(
    "1.00000000000000005683",
    originDecimalPlaces,
  );
  const destinationDepositAmount = ethers.parseUnits(
    "1",
    destinationDecimalPlaces,
  );
  const convertedDepositAmount = ethers.parseUnits(
    "1.000000000000000056",
    bridgeDefaultDecimalPlaces,
  );
  const destinationRelayerConvertedAmount = ethers.parseUnits(
    "1",
    bridgeDefaultDecimalPlaces,
  );
  const roundingLoss =
    ethers.toBigInt(originDepositAmount) -
    ethers.parseUnits("1", originDecimalPlaces);
  const expectedDepositNonce = 1;
  const feeData = "0x";
  const securityModel = 1;
  const destinationSlot = 5146128;
  const originSlot = 5146277;
  const originRouterAddress = "0x8C7478407e0f26Ec1EA26D0d0Bbc72aCC42e54c6";
  const destinationRouterAddress = "0x5d539A36f74A61d3b3C1499A428af77eD37264B1";
  const originStateRoot =
    "0x0ed60112db1cfc10f22e2884ddf30917ac55b158e9fcfc89f012ee0f2e26bb34";
  const destinationStateRoot =
    "0xa1bfd59e5618c0c1be852b96e56293bc13dbe98278245e4bbf1ad5bf59e0f399";

  let depositorAccount: HardhatEthersSigner;
  let recipientAccount: HardhatEthersSigner;
  let originDepositData: string;
  let originResourceID: string;
  let originBridgeInstance: Bridge;
  let originRouterInstance: Router;
  let originExecutorInstance: Executor;
  let originStateRootStorageInstance: StateRootStorage;
  let originERC20MintableInstance: ERC20PresetMinterPauser;
  let originERC20HandlerInstance: ERC20Handler;

  let destinationBridgeInstance: Bridge;
  let destinationRouterInstance: Router;
  let destinationExecutorInstance: Executor;
  let destinationStateRootStorageInstance: StateRootStorage;
  let destinationDepositData: string;
  let destinationResourceID: string;
  let destinationERC20MintableInstance: ERC20PresetMinterPauser;
  let destinationERC20HandlerInstance: ERC20Handler;
  let destinationRelayer1: HardhatEthersSigner;
  let originRelayer1: HardhatEthersSigner;

  beforeEach(async () => {
    [
      ,
      depositorAccount,
      recipientAccount,
      originRelayer1,
      destinationRelayer1,
    ] = await ethers.getSigners();

    [
      originBridgeInstance,
      originRouterInstance,
      originExecutorInstance,
      originStateRootStorageInstance,
    ] = await deployBridgeContracts(originDomainID, originRouterAddress);
    [
      destinationBridgeInstance,
      destinationRouterInstance,
      destinationExecutorInstance,
      destinationStateRootStorageInstance,
    ] = await deployBridgeContracts(
      destinationDomainID,
      destinationRouterAddress,
    );
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauserDecimals",
    );
    originERC20MintableInstance = await ERC20MintableContract.deploy(
      "Token",
      "TOK",
      originDecimalPlaces,
    );
    destinationERC20MintableInstance = await ERC20MintableContract.deploy(
      "Token",
      "TOK",
      destinationDecimalPlaces,
    );
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    originERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await originBridgeInstance.getAddress(),
    );
    destinationERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await destinationBridgeInstance.getAddress(),
    );

    originResourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    destinationResourceID =
      "0x0000000000000000000000000000000000000000000000000000000000000000";

    await originERC20MintableInstance.mint(
      depositorAccount,
      initialTokenAmount,
    );

    await originERC20MintableInstance
      .connect(depositorAccount)
      .approve(
        await originERC20HandlerInstance.getAddress(),
        originDepositAmount,
      ),
      await originERC20MintableInstance.grantRole(
        await originERC20MintableInstance.MINTER_ROLE(),
        await originERC20HandlerInstance.getAddress(),
      ),
      await destinationERC20MintableInstance.grantRole(
        await destinationERC20MintableInstance.MINTER_ROLE(),
        await destinationERC20HandlerInstance.getAddress(),
      ),
      await originBridgeInstance.adminSetResource(
        await originERC20HandlerInstance.getAddress(),
        originResourceID,
        await originERC20MintableInstance.getAddress(),
        // set decimal places for handler and token
        ethers.toBeHex(originDecimalPlaces),
      ),
      await originBridgeInstance.adminSetBurnable(
        await originERC20HandlerInstance.getAddress(),
        await originERC20MintableInstance.getAddress(),
      ),
      await destinationBridgeInstance.adminSetResource(
        await destinationERC20HandlerInstance.getAddress(),
        destinationResourceID,
        await destinationERC20MintableInstance.getAddress(),
        // set decimal places for handler and token
        ethers.toBeHex(destinationDecimalPlaces),
      ),
      await destinationBridgeInstance.adminSetBurnable(
        await destinationERC20HandlerInstance.getAddress(),
        await destinationERC20MintableInstance.getAddress(),
      );

    originDepositData = createERCDepositData(
      originDepositAmount.toString(),
      20,
      await recipientAccount.getAddress(),
    );
    destinationDepositData = createERCDepositData(
      destinationDepositAmount.toString(),
      20,
      await depositorAccount.getAddress(),
    );

    await destinationStateRootStorageInstance.storeStateRoot(
      originDomainID,
      destinationSlot,
      destinationStateRoot,
    );
    await originStateRootStorageInstance.storeStateRoot(
      destinationDomainID,
      originSlot,
      originStateRoot,
    );
  });

  it("[sanity] check token contract decimals match set decimals on handlers", async () => {
    const originTokenContractDecimals =
      await originERC20MintableInstance.decimals();
    const originDecimalsSetOnHandler = (
      await originERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await originERC20MintableInstance.getAddress(),
      )
    ).decimals;

    const destinationTokenContractDecimals =
      await destinationERC20MintableInstance.decimals();
    const destinationDecimalsSetOnHandler = (
      await destinationERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await destinationERC20MintableInstance.getAddress(),
      )
    ).decimals;

    assert.strictEqual(
      originTokenContractDecimals,
      originDecimalsSetOnHandler["externalDecimals"],
    );
    assert.strictEqual(
      destinationTokenContractDecimals,
      destinationDecimalsSetOnHandler["externalDecimals"],
    );
  });

  it(`E2E: depositAmount of Origin ERC20 owned by depositAddress to Destination ERC20
        owned by recipientAccount and back again`, async () => {
    let depositorBalance;
    let recipientBalance;

    // depositorAccount makes initial deposit of depositAmount
    const originDepositTx = await originRouterInstance
      .connect(depositorAccount)
      .deposit(
        destinationDomainID,
        originResourceID,
        securityModel,
        originDepositData,
        feeData,
      );
    await expect(originDepositTx).not.to.be.reverted;

    // check that deposited amount converted to 18 decimal places is emitted in data
    const originExpectedDepositAmount =
      toHex(convertedDepositAmount.toString(), 32) +
      originDepositData.substring(66);
    await expect(originDepositTx)
      .to.emit(originRouterInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        originResourceID.toLowerCase(),
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        originExpectedDepositAmount.toLowerCase(),
      );

    const originDomainProposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: await getDepositEventData(originDepositTx),
      resourceID: destinationResourceID,
      storageProof: storageProof7[0].proof,
    };

    // destinationRelayer1 executes the proposal
    await expect(
      destinationExecutorInstance
        .connect(destinationRelayer1)
        .executeProposal(originDomainProposal, accountProof7, destinationSlot),
    ).not.to.be.reverted;

    // Assert ERC20 balance was transferred from depositorAccount
    depositorBalance =
      await originERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(
      depositorBalance.toString(),
      (initialTokenAmount - originDepositAmount).toString(),
      "originDepositAmount wasn't transferred from depositorAccount",
    );

    // Assert ERC20 balance was transferred to recipientAccount
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(
      recipientBalance,
      destinationDepositAmount,
      "originDepositAmount wasn't transferred to recipientAccount",
    );

    // At this point a representation of OriginERC20Mintable has been transferred from
    // depositor to the recipient using Both Bridges and DestinationERC20Mintable.
    // Next we will transfer DestinationERC20Mintable back to the depositor

    await destinationERC20MintableInstance
      .connect(recipientAccount)
      .approve(
        await destinationERC20HandlerInstance.getAddress(),
        destinationDepositAmount,
      );

    // recipientAccount makes a deposit of the received depositAmount
    const destinationDepositTx = await destinationRouterInstance
      .connect(recipientAccount)
      .deposit(
        originDomainID,
        destinationResourceID,
        securityModel,
        destinationDepositData,
        feeData,
      );
    await expect(destinationDepositTx).not.to.be.reverted;

    // check that deposited amount converted to 18 decimal places is emitted in data
    const destinationExpectedDepositData =
      toHex(destinationRelayerConvertedAmount.toString(), 32) +
      destinationDepositData.substring(66);
    await expect(destinationDepositTx)
      .to.emit(destinationRouterInstance, "Deposit")
      .withArgs(
        originDomainID,
        securityModel,
        destinationResourceID,
        expectedDepositNonce,
        await recipientAccount.getAddress(),
        destinationExpectedDepositData.toLowerCase(),
      );

    const destinationDomainProposal = {
      originDomainID: destinationDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: await getDepositEventData(destinationDepositTx),
      resourceID: originResourceID,
      storageProof: storageProof8[0].proof,
    };

    // Recipient should have a balance of 0 (deposit amount)
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance.toString(), "0");

    // destinationRelayer1 executes the proposal
    await expect(
      originExecutorInstance
        .connect(originRelayer1)
        .executeProposal(destinationDomainProposal, accountProof8, originSlot),
    ).not.to.be.reverted;

    // Assert ERC20 balance was transferred from recipientAccount
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance.toString(), "0");

    // Assert ERC20 balance was transferred to recipientAccount minus the roundingLoss
    depositorBalance =
      await originERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(depositorBalance, initialTokenAmount - roundingLoss);
  });

  it(`should revert if deposit amount would result in converted zero amount`, async () => {
    const depositAmount = "50";

    const originDepositData = createERCDepositData(
      depositAmount,
      20,
      await recipientAccount.getAddress(),
    );

    // hardhat locally deploys contract to predictable address, so
    // ERCHandlerHelpers locally is always deployed to this address
    const ercHandlerHelpersAddress =
      "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    const ercHandlerHelpersInstance = await ethers.getContractAt(
      "ERCHandlerHelpers",
      ercHandlerHelpersAddress,
    );

    // depositorAccount makes initial deposit of depositAmount
    await expect(
      originRouterInstance
        .connect(depositorAccount)
        .deposit(
          destinationDomainID,
          originResourceID,
          securityModel,
          originDepositData,
          feeData,
        ),
    ).to.be.revertedWithCustomError(
      ercHandlerHelpersInstance,
      "DepositAmountTooSmall(uint256)",
    );
  });
});
