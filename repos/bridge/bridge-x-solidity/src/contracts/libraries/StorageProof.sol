// SPDX-License-Identifier: MIT
pragma solidity 0.8.11;

import { RLPReader } from "./RLPReader.sol";
import { MerkleTrie } from "./MerkleTrie.sol";

library StorageProof {
    using RLPReader for RLPReader.RLPItem;
    using RLPReader for bytes;

    error StorageValueDoesNotExist();
    error AccountDoesNotExist();
    error InvalidAccountListLength();

    function getStorageValue(bytes32 slotHash, bytes32 storageRoot, bytes[] memory stateProof)
        internal pure
        returns (bytes32)
    {
        bytes memory valueRlpBytes =
            MerkleTrie.get(abi.encodePacked(slotHash), stateProof, storageRoot);
        if (valueRlpBytes.length <= 0) revert StorageValueDoesNotExist();
        return valueRlpBytes.toRLPItem().readBytes32();
    }

    function getStorageRoot(bytes[] memory proof, address contractAddress, bytes32 stateRoot)
        internal pure
        returns (bytes32)
    {
        bytes32 addressHash = keccak256(abi.encodePacked(contractAddress));
        bytes memory acctRlpBytes = MerkleTrie.get(abi.encodePacked(addressHash), proof, stateRoot);
        if (acctRlpBytes.length <= 0) revert AccountDoesNotExist();
        RLPReader.RLPItem[] memory acctFields = acctRlpBytes.toRLPItem().readList();
        if (acctFields.length != 4) revert InvalidAccountListLength();
        return bytes32(acctFields[2].readUint256());
    }
}
