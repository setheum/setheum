// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20PresetMinterPauser,
} from "../../../typechain-types";
import { deployBridgeContracts } from "../../helpers";

describe("ERC20Handler - [Burn ERC20]", () => {
  const domainID = 1;
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let ERC20HandlerInstance: ERC20Handler;
  let ERC20MintableInstance1: ERC20PresetMinterPauser;
  let ERC20MintableInstance2: ERC20PresetMinterPauser;

  let resourceID1: string;
  let resourceID2: string;
  let initialResourceIDs: Array<string>;
  let initialContractAddresses: Array<string>;
  let burnableContractAddresses: Array<string>;

  beforeEach(async () => {
    [bridgeInstance] = await deployBridgeContracts(domainID, routerAddress);
    const ERC20MintableContract = await ethers.getContractFactory(
      "ERC20PresetMinterPauser",
    );
    ERC20MintableInstance1 = await ERC20MintableContract.deploy("Token", "TOK");
    ERC20MintableInstance2 = await ERC20MintableContract.deploy("Token", "TOK");
    const ERC20HandlerContract =
      await ethers.getContractFactory("ERC20Handler");
    ERC20HandlerInstance = await ERC20HandlerContract.deploy(
      await bridgeInstance.getAddress(),
    );
    resourceID1 = ethers.zeroPadValue(
      (await ERC20MintableInstance1.getAddress()) +
        ethers.toBeHex(domainID).substring(2),
      32,
    );
    resourceID2 = ethers.zeroPadValue(
      (await ERC20MintableInstance2.getAddress()) +
        ethers.toBeHex(domainID).substring(2),
      32,
    );
    initialResourceIDs = [resourceID1, resourceID2];
    initialContractAddresses = [
      await ERC20MintableInstance1.getAddress(),
      await ERC20MintableInstance2.getAddress(),
    ];
    burnableContractAddresses = [await ERC20MintableInstance1.getAddress()];
  });

  it("[sanity] contract should be deployed successfully", async () => {
    expect(await ERC20HandlerInstance.getAddress()).not.to.be.undefined;
  });

  it("burnableContractAddresses should be marked as burnable", async () => {
    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetResource(
          await ERC20HandlerInstance.getAddress(),
          initialResourceIDs[i],
          initialContractAddresses[i],
          emptySetResourceData,
        ),
      ).not.to.be.reverted;
    }

    for (let i = 0; i < burnableContractAddresses.length; i++) {
      await expect(
        bridgeInstance.adminSetBurnable(
          await ERC20HandlerInstance.getAddress(),
          burnableContractAddresses[i],
        ),
      ).not.to.be.reverted;
    }

    for (const burnableAddress of burnableContractAddresses) {
      const isBurnable = (
        await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
          burnableAddress,
        )
      ).isBurnable;

      assert.isTrue(isBurnable, "Contract wasn't successfully marked burnable");
    }
  });

  it("await ERC20MintableInstance2.getAddress() should not be marked as burnable", async () => {
    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetResource(
          await ERC20HandlerInstance.getAddress(),
          initialResourceIDs[i],
          initialContractAddresses[i],
          emptySetResourceData,
        ),
      ).not.to.be.reverted;
    }

    for (let i = 0; i < burnableContractAddresses.length; i++) {
      await expect(
        bridgeInstance.adminSetBurnable(
          await ERC20HandlerInstance.getAddress(),
          burnableContractAddresses[i],
        ),
      ).not.to.be.reverted;
    }

    const isBurnable = (
      await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance2.getAddress(),
      )
    ).isBurnable;

    assert.isFalse(isBurnable, "Contract shouldn't be marked burnable");
  });

  it("await ERC20MintableInstance2.getAddress() should be marked as burnable after setBurnable is called", async () => {
    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetResource(
          await ERC20HandlerInstance.getAddress(),
          initialResourceIDs[i],
          initialContractAddresses[i],
          emptySetResourceData,
        ),
      ).not.to.be.reverted;
    }

    for (let i = 0; i < burnableContractAddresses.length; i++) {
      await expect(
        bridgeInstance.adminSetBurnable(
          await ERC20HandlerInstance.getAddress(),
          burnableContractAddresses[i],
        ),
      ).not.to.be.reverted;
    }

    await bridgeInstance.adminSetBurnable(
      await ERC20HandlerInstance.getAddress(),
      await ERC20MintableInstance2.getAddress(),
    );
    const isBurnable = (
      await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
        await ERC20MintableInstance2.getAddress(),
      )
    ).isBurnable;

    assert.isTrue(isBurnable, "Contract wasn't successfully marked burnable");
  });

  it(`ERC20MintableInstances should not be marked as
      burnable after setResource is called on already burnable tokens`, async () => {
    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetResource(
          await ERC20HandlerInstance.getAddress(),
          initialResourceIDs[i],
          initialContractAddresses[i],
          emptySetResourceData,
        ),
      ).not.to.be.reverted;
    }

    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetBurnable(
          await ERC20HandlerInstance.getAddress(),
          initialContractAddresses[i],
        ),
      ).not.to.be.reverted;
    }

    // tokens should be marked as burnable
    for (let i = 0; i < initialResourceIDs.length; i++) {
      const isBurnableBeforeReRegisteringResource = (
        await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
          initialContractAddresses[i],
        )
      ).isBurnable;

      assert.isTrue(
        isBurnableBeforeReRegisteringResource,
        "Contract wasn't successfully marked burnable",
      );
    }

    // re-register resource - sets isBurnable to false for tokens
    for (let i = 0; i < initialResourceIDs.length; i++) {
      await expect(
        bridgeInstance.adminSetResource(
          await ERC20HandlerInstance.getAddress(),
          initialResourceIDs[i],
          initialContractAddresses[i],
          emptySetResourceData,
        ),
      ).not.to.be.reverted;
    }

    // tokens should not be marked as burnable if resource is re-registered
    for (let i = 0; i < initialResourceIDs.length; i++) {
      const isBurnableAfterReRegisteringResource = (
        await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
          initialContractAddresses[i],
        )
      ).isBurnable;

      assert.isFalse(
        isBurnableAfterReRegisteringResource,
        "Contract shouldn't be marked burnable",
      );
    }
  });
});
