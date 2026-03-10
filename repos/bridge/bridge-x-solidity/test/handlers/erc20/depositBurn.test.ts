// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert } from "chai";
import { ethers } from "hardhat";

import type {
  Bridge,
  ERC20Handler,
  ERC20PresetMinterPauser,
} from "../../../typechain-types";
import { deployBridgeContracts, createResourceID } from "../../helpers";

describe("ERC20Handler - [Deposit Burn ERC20]", () => {
  const domainID = 1;
  const depositAmount = 10;
  const emptySetResourceData = "0x";
  const routerAddress = "0x1a60efB48c61A79515B170CA61C84DD6dCA80418";

  let bridgeInstance: Bridge;
  let ERC20MintableInstance1: ERC20PresetMinterPauser;
  let ERC20MintableInstance2: ERC20PresetMinterPauser;
  let ERC20HandlerInstance: ERC20Handler;
  let depositorAccount: HardhatEthersSigner;

  let resourceID1: string;
  let resourceID2: string;
  const burnableContractAddresses: Array<string> = [];

  beforeEach(async () => {
    [, depositorAccount] = await ethers.getSigners();

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

    resourceID1 = createResourceID(
      await ERC20MintableInstance1.getAddress(),
      domainID,
    );
    resourceID2 = createResourceID(
      await ERC20MintableInstance2.getAddress(),
      domainID,
    );
    burnableContractAddresses.push(await ERC20MintableInstance1.getAddress());

    await Promise.all([
      ERC20MintableInstance1.connect(depositorAccount).approve(
        await ERC20HandlerInstance.getAddress(),
        depositAmount,
      ),
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID1,
        await ERC20MintableInstance1.getAddress(),
        emptySetResourceData,
      ),
      bridgeInstance.adminSetResource(
        await ERC20HandlerInstance.getAddress(),
        resourceID2,
        await ERC20MintableInstance2.getAddress(),
        emptySetResourceData,
      ),
      bridgeInstance.adminSetBurnable(
        await ERC20HandlerInstance.getAddress(),
        await ERC20MintableInstance1.getAddress(),
      ),
    ]);
  });

  it("[sanity] burnableContractAddresses should be marked as burnable", async () => {
    for (const burnableAddress of burnableContractAddresses) {
      const isBurnable = (
        await ERC20HandlerInstance._tokenContractAddressToTokenProperties(
          burnableAddress,
        )
      ).isBurnable;

      assert.isTrue(isBurnable, "Contract wasn't successfully marked burnable");
    }
  });
});
