// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import { assert } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20PresetMinterPauser,
} from "../../../typechain-types";
import { createResourceID, deployBridgeContracts } from "../../helpers";

describe("ERC20Handler - [setResourceIDAndContractAddress]", () => {
  const domainID = 1;
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let ERC20MintableInstance1: ERC20PresetMinterPauser;
  let ERC20MintableInstance2: ERC20PresetMinterPauser;
  let ERC20HandlerInstance1: ERC20Handler;
  let ERC20HandlerInstance2: ERC20Handler;

  let resourceID1: string;

  beforeEach(async () => {
    [bridgeInstance] = await deployBridgeContracts(domainID, routerAddress);
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance1 = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20MintableInstance2 = await ERC20MintableContract.deploy("Token", "TOK");
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance1 = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
    ERC20HandlerInstance2 = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );

    resourceID1 = createResourceID(
      await ERC20MintableInstance1.getAddress(),
      domainID,
    );

    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      resourceID1,
      await ERC20MintableInstance1.getAddress(),
      emptySetResourceData,
    );
  });

  it("[sanity] ERC20MintableInstance1's resourceID and contract address should be set correctly", async () => {
    const retrievedTokenAddress1 =
      await ERC20HandlerInstance1._resourceIDToTokenContractAddress(
        resourceID1,
      );
    assert.strictEqual(
      await ERC20MintableInstance1.getAddress(),
      retrievedTokenAddress1,
    );

    const retrievedResourceID1 = (
      await ERC20HandlerInstance1._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance1.getAddress(),
      )
    ).resourceID;

    assert.strictEqual(resourceID1, retrievedResourceID1);

    const secondERC20ResourceID = ethers.zeroPadValue(
      (await ERC20MintableInstance2.getAddress()) +
        ethers.toBeHex(domainID).substring(2),
      32,
    );

    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      secondERC20ResourceID,
      await ERC20MintableInstance2.getAddress(),
      emptySetResourceData,
    );

    const retrievedTokenAddress2 =
      await ERC20HandlerInstance1._resourceIDToTokenContractAddress(
        secondERC20ResourceID,
      );
    assert.strictEqual(
      await ERC20MintableInstance2.getAddress(),
      retrievedTokenAddress2,
    );

    const retrievedResourceID = (
      await ERC20HandlerInstance1._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance2.getAddress(),
      )
    ).resourceID;

    assert.strictEqual(secondERC20ResourceID, retrievedResourceID);
  });

  it("existing resourceID should be updated correctly with new token contract address", async () => {
    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      resourceID1,
      await ERC20MintableInstance1.getAddress(),
      emptySetResourceData,
    );

    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      resourceID1,
      await ERC20MintableInstance2.getAddress(),
      emptySetResourceData,
    );

    const retrievedTokenAddress =
      await ERC20HandlerInstance1._resourceIDToTokenContractAddress(
        resourceID1,
      );
    assert.strictEqual(
      await ERC20MintableInstance2.getAddress(),
      retrievedTokenAddress,
    );

    const retrievedResourceID = (
      await ERC20HandlerInstance1._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance2.getAddress(),
      )
    ).resourceID;

    assert.strictEqual(resourceID1, retrievedResourceID);
  });

  it("existing resourceID should be updated correctly with new handler address", async () => {
    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      resourceID1,
      await ERC20MintableInstance1.getAddress(),
      emptySetResourceData,
    );

    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance2.getAddress(),
      resourceID1,
      await ERC20MintableInstance2.getAddress(),
      emptySetResourceData,
    );

    const bridgeHandlerAddress =
      await bridgeInstance._resourceIDToHandlerAddress(resourceID1);
    assert.strictEqual(
      bridgeHandlerAddress,
      await ERC20HandlerInstance2.getAddress(),
    );
  });

  it("existing resourceID should be replaced by new resourceID in handler", async () => {
    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      resourceID1,
      await ERC20MintableInstance1.getAddress(),
      emptySetResourceData,
    );

    const secondERC20ResourceID = ethers.zeroPadValue(
      (await ERC20MintableInstance2.getAddress()) +
        ethers.toBeHex(domainID).substring(2),
      32,
    );

    await bridgeInstance.adminSetResource(
      await ERC20HandlerInstance1.getAddress(),
      secondERC20ResourceID,
      await ERC20MintableInstance1.getAddress(),
      emptySetResourceData,
    );

    const retrievedResourceID = (
      await ERC20HandlerInstance1._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance1.getAddress(),
      )
    ).resourceID;

    assert.strictEqual(secondERC20ResourceID, retrievedResourceID);

    const retrievedContractAddress =
      await ERC20HandlerInstance1._resourceIDToTokenContractAddress(
        secondERC20ResourceID,
      );

    assert.strictEqual(
      retrievedContractAddress,
      await ERC20MintableInstance1.getAddress(),
    );
  });
});
