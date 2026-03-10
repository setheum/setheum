// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20PresetMinterPauser,
  Executor,
  Router,
  StateRootStorage,
} from "../../../../typechain-types";
import {
  deployBridgeContracts,
  createERCDepositData,
  getDepositEventData,
} from "../../../helpers";
import {
  accountProof5,
  storageProof5,
  accountProof6,
  storageProof6,
} from "../../../testingProofs";

describe("E2E ERC20 - Two EVM Chains, one with decimal places == 18, other with == 0", () => {
  const originDomainID = 1;
  const destinationDomainID = 2;

  const originDecimalPlaces = 0;
  const destinationDecimalPlaces = 18;
  const bridgeDefaultDecimalPlaces = 18;
  const initialTokenAmount = BigInt("10000000000000000");
  const originDepositAmount = BigInt("1400000000000000");
  const destinationDepositAmount = ethers.parseUnits(
    originDepositAmount.toString(),
    destinationDecimalPlaces,
  );
  const convertedDepositAmount = ethers.parseUnits(
    originDepositAmount.toString(),
    bridgeDefaultDecimalPlaces,
  );
  const expectedDepositNonce = 1;
  const feeData = "0x";
  const emptySetResourceData = "0x";
  const securityModel = 1;
  const destinationSlot = 5145696;
  const originSlot = 5145852;
  const originRouterAddress = "0xA42a494c011Ca1f1CBaf2710348dC277135a03Cd";
  const destinationRouterAddress = "0xdf7ab612d3F8cF804FE5755DC0393957b7aFA4fD";
  const originStateRoot =
    "0xaa6d1cf3f2703b09ea26f734f66c9af1e7bebc6ab89e55824efbc5dada08c976";
  const destinationStateRoot =
    "0x4ecdf432ceab3e074a17f4ae103a768966025daf309628b2d7418693f01a59e4";

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
  let destinationDepositProposalData: string;

  let destinationDomainProposal: {
    originDomainID: number;
    securityModel: number;
    depositNonce: number;
    data: string;
    resourceID: string;
    storageProof: Array<string>;
  };

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
        emptySetResourceData,
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
      await recipientAccount.getAddress(),
    );
    destinationDepositProposalData = createERCDepositData(
      convertedDepositAmount.toString(),
      20,
      await depositorAccount.getAddress(),
    );

    destinationDomainProposal = {
      originDomainID: destinationDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: destinationDepositProposalData,
      resourceID: originResourceID,
      storageProof: storageProof6[0].proof,
    };

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

    const destinationDecimalsSetOnHandler = (
      await destinationERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await destinationERC20MintableInstance.getAddress(),
      )
    ).decimals;

    assert.strictEqual(
      originTokenContractDecimals.toString(),
      originDecimalsSetOnHandler["externalDecimals"].toString(),
    );
    assert.isFalse(destinationDecimalsSetOnHandler["isSet"]);
    assert.strictEqual(
      "0",
      destinationDecimalsSetOnHandler["externalDecimals"].toString(),
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

    const originDomainProposal = {
      originDomainID: originDomainID,
      securityModel: securityModel,
      depositNonce: expectedDepositNonce,
      data: await getDepositEventData(originDepositTx),
      resourceID: destinationResourceID,
      storageProof: storageProof5[0].proof,
    };

    // destinationRelayer1 executes the proposal
    await expect(
      destinationExecutorInstance
        .connect(destinationRelayer1)
        .executeProposal(originDomainProposal, accountProof5, destinationSlot),
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
    const depositTx = await destinationRouterInstance
      .connect(recipientAccount)
      .deposit(
        originDomainID,
        destinationResourceID,
        securityModel,
        destinationDepositData,
        feeData,
      );
    await expect(depositTx).not.to.be.reverted;

    // check that handlerResponse is empty - deposits from networks with 18 decimal
    // places shouldn't return handlerResponse
    await expect(depositTx)
      .to.emit(destinationRouterInstance, "Deposit")
      .withArgs(
        originDomainID,
        securityModel,
        destinationResourceID.toLowerCase(),
        expectedDepositNonce,
        await recipientAccount.getAddress(),
        destinationDepositData.toLowerCase(),
      );

    // Recipient should have a balance of 0 (deposit amount)
    recipientBalance =
      await destinationERC20MintableInstance.balanceOf(recipientAccount);
    assert.strictEqual(recipientBalance.toString(), "0");

    // originRelayer1 executes the proposal
    await expect(
      originExecutorInstance
        .connect(originRelayer1)
        .executeProposal(destinationDomainProposal, accountProof6, originSlot),
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
