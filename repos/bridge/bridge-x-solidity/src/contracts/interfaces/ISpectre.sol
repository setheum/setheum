// The Licensed Work is (c) 2023 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;


/**
    @title Interface for Spectre (https://github.com/ChainSafe/Spectre/blob/main/contracts/src/Spectre.sol) contract
    @author ChainSafe Systems.
 */
interface ISpectre {
    struct SyncStepInput {
        uint64 attestedSlot;
        uint64 finalizedSlot;
        uint64 participation;
        bytes32 finalizedHeaderRoot;
        bytes32 executionPayloadRoot;
    }

    /// @notice Verify that a sync committee has attested to a block that finalizes 
    /// the given header root and execution payload
    /// @param input The input to the sync step. Defines the slot and attestation to verify
    /// @param proof The proof for the sync step
    function step(SyncStepInput calldata input, bytes calldata proof) external;


    /// @notice Use the current sync committee to verify the transition to a new sync committee
    /// @param rotateProof The proof for the rotation
    /// @param stepInput The input to the sync step.
    /// @param stepProof The proof for the sync step
    function rotate(
        bytes calldata rotateProof, 
        SyncStepInput calldata stepInput, 
        bytes calldata stepProof
    ) external;

    /// @notice Fetches the execution payload root for the given slot
    /// @param  slot Beacon chain slot for requesting execution payload root
    /// @return Execution payload root for given slot
    function executionPayloadRoots(uint256 slot) external view returns (bytes32);

}
