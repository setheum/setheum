import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { assert, expect } from "chai";
import { ethers } from "hardhat";

import type { ISpectre, SpectreProxy } from "../../../typechain-types";

describe("Spectre Proxy", () => {
  const originDomainID = 1;

  const invalidOriginDomainID = 4;
  const validDomainID = 3;

  const rotateProof =
    "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f";
  const stepProof =
    "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f";

  const validStateRoot =
    "0x19f9a8c688e5ce7411de0bcd2fb5bd48de9b8c9abdaccf99d466b003b548ce25";
  const validStateRootProof = [
    "0x6748266472daf69f43b5b84abfafda3b948225b01af141e19289726063fc3c6c",
    "0xb11c0644596de5976fc972b6d736b49319811c3fe82979ee0e9832e6f4db243a",
    "0xd015bf510ad0aecc6cf5c9bddca152d895372bc8c4dda07a6d18ba68061c6e8f",
    "0x9048454d5dd77a07f0ba1895200689f35c4880f8a4450defbcab4fce007df369",
    "0xca6b1e1463286d78aa0a439739befea35e495736654f931b4f97cf115ec18170",
  ];
  const invalidStateRoot =
    "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f";
  const invalidStateRootProof = [
    "0x0c2e45ec77206f3b0cac1da903c4bc05cf177da367c428c1ba3cab0f654f4f78",
    "0xdf581c183b1083cf6be31fde9f6073dfacfc252f8b514577f2ca03955b921552",
    "0x59dac95a8278295a3a05d809156f69b45007af3f3df94bcabe4bbbdd9cce5c5a",
    "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f",
    "0x9048454d5dd77a07f0ba1895200689f35c4880f8a4450defbcab4fce007df369",
  ];

  const stepInput: ISpectre.SyncStepInputStruct = {
    finalizedHeaderRoot:
      "0xb87f5209e312fbecbcb6053883eb03f5d67134f391963543883d28480acdc6b1",
    finalizedSlot: 100,
    attestedSlot: 101,
    participation: 8,
    executionPayloadRoot:
      "0x996348a575957d6a4878681a40e014bcb3c102017462e91c6a83f099b210986c",
  };

  const constructorDomains = [2, 3];
  const invalidSpectreAddress = "0x9Da9DbbB87db6e9862C79651CBae0D468fa88c71";
  const constructorAddresses = [invalidSpectreAddress];

  let spectreAddress: string;

  let spectreProxyInstance: SpectreProxy;
  let nonAdminAccount: HardhatEthersSigner;

  beforeEach(async () => {
    [, nonAdminAccount] = await ethers.getSigners();
    const SpectreProxyContract =
      await ethers.getContractFactory("SpectreProxy");
    const SpectreContract = await ethers.getContractFactory("TestSpectre");
    const spectreInstance = await SpectreContract.deploy();
    spectreAddress = await spectreInstance.getAddress();
    constructorAddresses[1] = spectreAddress;
    spectreProxyInstance = await SpectreProxyContract.deploy(
      constructorDomains,
      constructorAddresses,
    );
  });

  it("constructor should set intial addresses", async () => {
    assert.equal(
      await spectreProxyInstance.spectreContracts(constructorDomains[0]),
      constructorAddresses[0],
    );
    assert.equal(
      await spectreProxyInstance.spectreContracts(constructorDomains[1]),
      spectreAddress,
    );
  });

  it("should require admin role to set spectre address", async () => {
    await expect(
      spectreProxyInstance
        .connect(nonAdminAccount)
        .adminSetSpectreAddress(originDomainID, spectreAddress),
    ).to.be.revertedWithCustomError(spectreProxyInstance, "SenderNotAdmin");
  });

  it("should set spectre address with an admin role", async () => {
    await spectreProxyInstance.adminSetSpectreAddress(
      originDomainID,
      spectreAddress,
    );

    assert.equal(
      await spectreProxyInstance.spectreContracts(originDomainID),
      spectreAddress,
    );
  });

  it("should revert if spectre address not set in rotate", async () => {
    await expect(
      spectreProxyInstance.rotate(
        invalidOriginDomainID,
        rotateProof,
        stepInput,
        stepProof,
      ),
    ).to.be.revertedWithCustomError(
      spectreProxyInstance,
      "SpectreAddressNotFound",
    );
  });

  it("should emit event even if rotate successful", async () => {
    const rotateTx = await spectreProxyInstance.rotate(
      validDomainID,
      rotateProof,
      stepInput,
      stepProof,
    );

    await expect(rotateTx)
      .to.emit(spectreProxyInstance, "CommitteeRotated")
      .withArgs(validDomainID, stepInput.attestedSlot);
  });

  it("should revert if spectre address not set in step", async () => {
    await expect(
      spectreProxyInstance.step(
        invalidOriginDomainID,
        stepInput,
        stepProof,
        validStateRoot,
        validStateRootProof,
      ),
    ).to.be.revertedWithCustomError(
      spectreProxyInstance,
      "SpectreAddressNotFound",
    );
  });

  it("should revert if step proof not valid", async () => {
    await expect(
      spectreProxyInstance.step(
        validDomainID,
        stepInput,
        stepProof,
        validStateRoot,
        invalidStateRootProof,
      ),
    ).to.be.revertedWithCustomError(spectreProxyInstance, "InvalidMerkleProof");
  });

  it("should revert if step state root not valid", async () => {
    await expect(
      spectreProxyInstance.step(
        validDomainID,
        stepInput,
        stepProof,
        invalidStateRoot,
        validStateRootProof,
      ),
    ).to.be.revertedWithCustomError(spectreProxyInstance, "InvalidMerkleProof");
  });

  it("should emit event and store state root if step valid", async () => {
    const stepTx = await spectreProxyInstance.step(
      validDomainID,
      stepInput,
      stepProof,
      validStateRoot,
      validStateRootProof,
    );

    assert.equal(
      await spectreProxyInstance.stateRoots(
        validDomainID,
        stepInput.finalizedSlot,
      ),
      validStateRoot,
    );
    assert.equal(
      await spectreProxyInstance.getStateRoot(
        validDomainID,
        stepInput.finalizedSlot,
      ),
      validStateRoot,
    );
    await expect(stepTx)
      .to.emit(spectreProxyInstance, "StateRootSubmitted")
      .withArgs(validDomainID, stepInput.finalizedSlot, validStateRoot);
  });
});
