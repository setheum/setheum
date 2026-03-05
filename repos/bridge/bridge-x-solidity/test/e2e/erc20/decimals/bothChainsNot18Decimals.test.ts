// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  StateRootStorage,
  Bridge,
  ERC20Handler,
  ERC20PresetMinterPauser,
  Executor,
  Router,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  createERCDepositData,
  toHex,
} from "../../../helpers";
import {
  accountProof3,
  storageProof3,
  accountProof4,
  storageProof4,
} from "../../../testingProofs";

describe("E2E ERC20 - Two EVM Chains both with decimal places != 18", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const originDecimalPlaces = BigInt(20);
  const destinationDecimalPlaces = BigInt(14);
  const bridgeDefaultDecimalPlaces = BigInt(18);
  const initialTokenAmount = ethers.parseUnits("100", originDecimalPlaces);
  const originDepositAmount = ethers.parseUnits("14", originDecimalPlaces);
  const destinationDepositAmount = ethers.parseUnits(
    "14",
    destinationDecimalPlaces,
  );
  const relayerConvertedAmount = ethers.parseUnits(
    "14",
    bridgeDefaultDecimalPlaces,
  );
  const expectedDepositNonce = 1;
  const feeData = "0x";
  const securityModel = 1;
  const destinationSlot = 5133413;
  const originSlot = 5139374;
  const originRouterAddress = "0x48b09e09E10A2cd8E991bc00392a689590ed8e77";
  const destinationRouterAddress = "0xc7B065F6c2A3e203692C0b01217f423caC15662e";
  const originStateRoot =
    "0xed91524a194957d604ab37fd1c8aa37e9435714c31e8c788819a1b6d8f1ff783";
  const destinationStateRoot =
    "0xdd18a15cab4810bf308fe626ba155496b77f73b3a68e6daf29fc2c7c7fb52a44";

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
  let originRelayer1: HardhatEthersSigner;

  let destinationBridgeInstance: Bridge;
  let destinationRouterInstance: Router;
  let destinationExecutorInstance: Executor;
  let destinationStateRootStorageInstance: StateRootStorage;
  let destinationDepositData: string;
  let destinationResourceID: string;
  let destinationERC20MintableInstance: ERC20PresetMinterPauser;
  let destinationERC20HandlerInstance: ERC20Handler;
  let destinationRelayer1: HardhatEthersSigner;

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

    const originExpectedDepositData =
      toHex(relayerConvertedAmount.toString(), 32) +
      originDepositData.substring(66);
    // check that deposited amount converted to 18 decimal places is
    // emitted in handlerResponse
    await expect(originDepositTx)
      .to.emit(originRouterInstance, "Deposit")
      .withArgs(
        destinationDomainID,
        securityModel,
        originResourceID.toLowerCase(),
        expectedDepositNonce,
        await depositorAccount.getAddress(),
        originExpectedDepositData.toLowerCase(),
      );

    const originDomainProposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: originExpectedDepositData,
      resourceID: destinationResourceID,
      storageProof: storageProof3[0].proof,
    };

    // destinationRelayer1 executes the proposal
    await expect(
      destinationExecutorInstance
        .connect(destinationRelayer1)
        .executeProposal(originDomainProposal, accountProof3, destinationSlot),
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

    const destinationExepectedDepositData =
      toHex(relayerConvertedAmount.toString(), 32) +
      destinationDepositData.substring(66);
    // check that deposited amount converted to 18 decimal places is
    // emitted in handlerResponse
    await expect(destinationDepositTx)
      .to.emit(destinationRouterInstance, "Deposit")
      .withArgs(
        originDomainID,
        securityModel,
        destinationResourceID.toLowerCase(),
        expectedDepositNonce,
        await recipientAccount.getAddress(),
        destinationExepectedDepositData.toLowerCase(),
      );

    const destinationDomainProposal = {
      originDomainID: destinationDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: destinationExepectedDepositData,
      resourceID: originResourceID,
      storageProof: storageProof4[0].proof,
    };

    // Recipient should have a balance of 0 (deposit amount)
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance.toString(), "0");

    // originRelayer1 executes the proposal
    await expect(
      originExecutorInstance
        .connect(originRelayer1)
        .executeProposal(destinationDomainProposal, accountProof4, originSlot),
    ).not.to.be.reverted;

    // Assert ERC20 balance was transferred from recipientAccount
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance.toString(), "0");

    // Assert ERC20 balance was transferred to recipientAccount
    depositorBalance =
      await originERC20MintableInstance.balanceOf(depositorAccount);
    assert.strictEqual(
      depositorBalance.toString(),
      initialTokenAmount.toString(),
    );
  });
});
