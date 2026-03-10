// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import type { TransactionReceipt, TransactionResponse } from "ethers";
import { ethers, network } from "hardhat";

import { generateAccessControlFuncSignatures } from "../scripts/utils";
import type {
  Bridge,
  Router,
  Executor,
  StateRootStorage,
  TestStore,
  TestDeposit,
} from "../typechain-types";

export const blankFunctionSig = "0x00000000";
export const blankFunctionDepositorOffset = "0x0000";

export function toHex(covertThis: string | number, padding: number): string {
  return ethers.zeroPadValue(ethers.toBeHex(covertThis), padding);
}

export function createERCDepositData(
  tokenAmountOrID: number | string,
  lenRecipientAddress: number,
  recipientAccount: string,
): string {
  return (
    "0x" +
    toHex(tokenAmountOrID, 32).substring(2) + // Token amount or ID to deposit  (32 bytes)
    toHex(lenRecipientAddress, 32).substring(2) + // len(recipientAccount)  (32 bytes)
    recipientAccount.substring(2) // recipientAccount  (?? bytes)
  );
}

export function createERCWithdrawData(
  tokenAddress: string,
  recipientAccount: string,
  tokenAmountOrID: number,
): string {
  return (
    "0x" +
    toHex(tokenAddress, 32).substring(2) +
    toHex(recipientAccount, 32).substring(2) +
    toHex(tokenAmountOrID, 32).substring(2)
  );
}

export function createPermissionlessGenericDepositData(
  executeFunctionSignature: string,
  executeContractAddress: string,
  maxFee: bigint,
  depositor: string,
  executionData: string,
  depositorCheck = true,
): string {
  if (depositorCheck) {
    // if "depositorCheck" is true -> append depositor address for destination chain check
    executionData = executionData.concat(toHex(depositor, 32).substring(2));
  }

  return (
    "0x" +
    toHex(maxFee.toString(), 32).substring(2) + // uint256
    toHex(
      String(executeFunctionSignature).substring(2).length / 2,
      2,
    ).substring(2) + // uint16
    String(executeFunctionSignature).substring(2) + // bytes
    toHex(executeContractAddress.substring(2).length / 2, 1).substring(2) + // uint8
    executeContractAddress.substring(2) + // bytes
    toHex(depositor.substring(2).length / 2, 1).substring(2) + // uint8
    depositor.substring(2) + // bytes
    executionData.substring(2)
  ); // bytes
}

export function constructGenericHandlerSetResourceData(
  ...args: Array<string>
): string {
  return args.reduce((accumulator, currentArg) => {
    if (typeof currentArg === "number") {
      currentArg = toHex(currentArg, 2) as unknown as string;
    }
    return accumulator.toString() + currentArg.substring(2);
  });
}

export function createResourceID(
  contractAddress: string,
  domainID: number,
): string {
  return toHex(contractAddress + toHex(domainID, 1).substring(2), 32);
}

export function decimalToPaddedBinary(decimal: bigint): string {
  return decimal.toString(2).padStart(64, "0");
}

// filter out only func signatures
const contractsToGenerateSignatures = ["Bridge", "Router", "Executor"];
export const accessControlFuncSignatures = generateAccessControlFuncSignatures(
  contractsToGenerateSignatures,
).map((e) => e.hash);

export async function deployBridgeContracts(
  domainID: number,
  routerAddress: string,
): Promise<[Bridge, Router, Executor, StateRootStorage]> {
  const securityModel = 1;
  const [adminAccount] = await ethers.getSigners();
  const AccessControlSegregatorContract = await ethers.getContractFactory(
    "AccessControlSegregator",
  );
  const accessControlInstance = await AccessControlSegregatorContract.deploy(
    accessControlFuncSignatures,
    Array(accessControlFuncSignatures.length).fill(
      await adminAccount.getAddress(),
    ),
  );
  const BridgeContract = await ethers.getContractFactory("Bridge");
  const bridgeInstance = await BridgeContract.deploy(
    domainID,
    await accessControlInstance.getAddress(),
  );
  const RouterContract = await ethers.getContractFactory("Router");
  const routerInstance = await RouterContract.deploy(
    await bridgeInstance.getAddress(),
    await accessControlInstance.getAddress(),
  );
  const StateRootStorageContract =
    await ethers.getContractFactory("StateRootStorage");
  const stateRootStorageInstance = await StateRootStorageContract.deploy();
  const ExecutorContract = await ethers.getContractFactory("Executor");
  const executorInstance = await ExecutorContract.deploy(
    await bridgeInstance.getAddress(),
    await accessControlInstance.getAddress(),
  );
  await executorInstance.adminChangeRouter(
    // mock that the origin domain the is different than executor domainID
    domainID == 1 ? 2 : 1,
    routerAddress,
  );
  // depending on domainID value, set source domain IDs slot index to 2
  await executorInstance.adminChangeSlotIndex(domainID == 1 ? 2 : 1, 2);

  await executorInstance.adminSetVerifiers(securityModel, [
    await stateRootStorageInstance.getAddress(),
  ]);

  await bridgeInstance.adminChangeRouterAddress(
    await routerInstance.getAddress(),
  );

  await bridgeInstance.adminChangeExecutorAddress(
    await executorInstance.getAddress(),
  );

  return [
    bridgeInstance,
    routerInstance,
    executorInstance,
    stateRootStorageInstance,
  ];
}

export async function getDepositEventData(
  depositTx: TransactionResponse,
): Promise<string> {
  return (
    ((await depositTx.wait(1)) as TransactionReceipt).logs[2] as unknown as {
      args: string[];
    }
  )["args"][5];
}

// This helper can be used to prepare execution data for PermissionlessGenericHandler
// The execution data will be packed together with depositorAccount before execution.
// If the target function parameters include reference types then the offsets should be kept consistent.
// This function packs the parameters together with a fake address and removes the address.
// After repacking the data in the handler together with depositorAccount, the offsets will be correct.
// Usage: use this function to prepare execution data,
// then pack the result together with executeFunctionSignature, maxFee etc
// (using the createPermissionlessGenericDepositData() helper)
// and then pass the data to Bridge.deposit().
export function createPermissionlessGenericExecutionData(
  types: Array<string>,
  values: Array<string | number | Array<string>>,
): string {
  types.unshift("address");
  values.unshift(ethers.ZeroAddress);
  return (
    "0x" + ethers.AbiCoder.defaultAbiCoder().encode(types, values).substring(66)
  );
}

export async function deployMockTestContracts(
  testStoreAddress: string,
  testDepositAddress: string,
): Promise<[TestStore, TestDeposit]> {
  // Deploy fake contracts to generate bytecode and override address for proof  verification logic
  const fakeTestStore = await ethers.getContractFactory("TestStore");
  const fakeTestStoreInstance = await fakeTestStore.deploy();
  const fakeTestDeposit = await ethers.getContractFactory("TestDeposit");
  const fakeTestDepositInstance = await fakeTestDeposit.deploy();
  // Get bytecode of the deployed contract using "eth_getCode"
  const fakeTestStoreInstanceBytecode = await network.provider.send(
    "eth_getCode",
    [await fakeTestStoreInstance.getAddress()],
  );
  const fakeTestDepositInstanceBytecode = await network.provider.send(
    "eth_getCode",
    [await fakeTestDepositInstance.getAddress()],
  );
  // Set bytecode at the target contract address with the fake contract bytecode
  await network.provider.send("hardhat_setCode", [
    testStoreAddress,
    fakeTestStoreInstanceBytecode,
  ]);
  await network.provider.send("hardhat_setCode", [
    testDepositAddress,
    fakeTestDepositInstanceBytecode,
  ]);
  // Fetch contract instances
  const testStoreInstance = await ethers.getContractAt(
    "TestStore",
    testStoreAddress,
  );
  const testDepositInstance = await ethers.getContractAt(
    "TestDeposit",
    testDepositAddress,
  );

  return [testStoreInstance, testDepositInstance];
}

module.exports = {
  blankFunctionSig,
  blankFunctionDepositorOffset,
  accessControlFuncSignatures,
  toHex,
  createERCDepositData,
  createERCWithdrawData,
  createPermissionlessGenericDepositData,
  constructGenericHandlerSetResourceData,
  createResourceID,
  decimalToPaddedBinary,
  deployBridgeContracts,
  createPermissionlessGenericExecutionData,
  getDepositEventData,
  deployMockTestContracts,
};
