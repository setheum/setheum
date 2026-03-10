// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "@openzeppelin/contracts/utils/Context.sol";
import "./interfaces/IAccessControlSegregator.sol";


import "./interfaces/IBridge.sol";
import "./interfaces/IHandler.sol";
import "./interfaces/IStateRootStorage.sol";
import "./libraries/StorageProof.sol";



/**
    @title Facilitates proposal executions.
    @author ChainSafe Systems.
 */
contract Executor is Context {

    IBridge public immutable _bridge;
    uint8 public immutable _domainID;
    IAccessControlSegregator public _accessControl;

    /*  origin domainID => metadata:
            uint8 slotIndex
            address routerAddress
    */
    mapping(uint8 => Metadata) public _originDomainIDMetadata;
    // securityModel => state root storage contract addresses
    mapping(uint8 => address[]) public _securityModels;
    // origin domainID => nonces set => used deposit nonces
    mapping(uint8 => mapping(uint256 => uint256)) public usedNonces;

    struct Proposal {
        uint8 originDomainID;
        uint8 securityModel;
        uint64 depositNonce;
        bytes32 resourceID;
        bytes data;
        bytes[] storageProof;
    }

    struct Metadata {
        uint8 slotIndex;
        address routerAddress;
    }

    event ProposalExecution(uint8 originDomainID, uint64 depositNonce, bytes handlerResponse);
    event FeeRouterChanged(uint8 originDomainID, address newRouter);
    event FailedHandlerExecution(bytes lowLevelData, uint8 originDomainID, uint64 depositNonce);

    error EmptyProposalsArray();
    error BridgeIsPaused();
    error ZeroAddressProvided();
    error NoVerifierAddressProvided();
    error AccessNotAllowed(address sender, bytes4 funcSig);
    error TransferHashDoesNotMatchSlotValue(bytes32 transferHash);
    error StateRootDoesNotMatch(IStateRootStorage stateRootStorage, bytes32 stateRoot);

    modifier onlyAllowed() {
        _onlyAllowed(msg.sig, _msgSender());
        _;
    }

    modifier whenBridgeNotPaused() {
        if (_bridge.paused()) revert BridgeIsPaused();
        _;
    }

    function _onlyAllowed(bytes4 sig, address sender) private view {
        if (!_accessControl.hasAccess(sig, sender)) revert AccessNotAllowed(sender, sig);
    }

    constructor(
        address bridge,
        address accessControl
    ) {
        if (bridge == address(0) || accessControl == address(0)) revert ZeroAddressProvided();

        _bridge = IBridge(bridge);
        _domainID = _bridge._domainID();
        _accessControl = IAccessControlSegregator(accessControl);
    }

    /**
        @notice Maps the {originDomainID} to {router} in _originDomainIDMetadata.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param newRouter Address of router that will be updated to.
     */
    function adminChangeRouter(uint8 originDomainID, address newRouter) external onlyAllowed {
        _originDomainIDMetadata[originDomainID].routerAddress = newRouter;
        emit FeeRouterChanged(originDomainID, newRouter);
    }

    /**
        @notice Maps the {originDomainID} to {slotIndex} in _originDomainIDMetadata.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param originDomainID domain from which the proposal originated.
        @param slotIndex Index number to be used for the belonging origin domain ID in slot key calculation.
     */
    function adminChangeSlotIndex(uint8 originDomainID, uint8 slotIndex) external onlyAllowed {
        _originDomainIDMetadata[originDomainID].slotIndex = slotIndex;
    }

    /**
        @notice Maps the {securityModel} to {verifiersAddresses} in _securitModels.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param securityModel .
        @param verifiersAddresses Array of verifiers addresses which store state roots.
     */
    function adminSetVerifiers(uint8 securityModel, address[] memory verifiersAddresses) external onlyAllowed {
        if (verifiersAddresses.length <= 0) revert NoVerifierAddressProvided();
        _securityModels[securityModel] = verifiersAddresses;
    }

    /**
        @notice Exposes {markNonceAsUsed} function to an address that has the right to call the
        specific function, which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param originDomainID domain from which the proposal originated.
        @param depositNonce nonce of a proposal that should be marked as used.
     */
    function adminMarkNonceAsUsed(uint8 originDomainID, uint64 depositNonce) public onlyAllowed {
        markNonceAsUsed(originDomainID, depositNonce);
    }

    /**
        @notice Executes a batch of deposit proposals using a specified handler contract for each proposal
        @notice Failed executeProposal from handler don't revert, emits {FailedHandlerExecution} event.
        @param proposals Array of Proposal which consists of:
        - originDomainID ID of chain deposit originated from.
        - resourceID ResourceID to be used when making deposits.
        - depositNonce ID of deposit generated by origin Bridge contract.
        - data Data originally provided when deposit was made.
        @notice Emits {ProposalExecution} event for each proposal in the batch.
     */
    function executeProposals(
        Proposal[] memory proposals,
        bytes[] memory accountProof,
        uint256 slot
    ) public whenBridgeNotPaused {
        if (proposals.length == 0) revert EmptyProposalsArray();

        for (uint256 i = 0; i < proposals.length; i++) {
            Proposal memory proposal = proposals[i];

            if (isProposalExecuted(proposal.originDomainID, proposal.depositNonce)) {
                continue;
            }
            bytes32 expectedStateRoot;
            bytes32 storageRoot;
            address routerAddress = _originDomainIDMetadata[proposal.originDomainID].routerAddress;

            IStateRootStorage checkingStateRootStorage = IStateRootStorage(
                _securityModels[proposal.securityModel][0]
            );
            expectedStateRoot = checkingStateRootStorage.getStateRoot(proposal.originDomainID, slot);
            uint256 numberOfSecurityModels = _securityModels[proposal.securityModel].length;

            for (uint256 j = 1; j < numberOfSecurityModels; j++) {
                IStateRootStorage stateRootStorage = IStateRootStorage(_securityModels[proposal.securityModel][j]);
                bytes32 stateRoot = stateRootStorage.getStateRoot(proposal.originDomainID, slot);
                if(expectedStateRoot != stateRoot) revert StateRootDoesNotMatch(stateRootStorage, stateRoot);
            }

            storageRoot = StorageProof.getStorageRoot(accountProof, routerAddress, expectedStateRoot);
            address handler = _bridge._resourceIDToHandlerAddress(proposal.resourceID);
            IHandler depositHandler = IHandler(handler);
            verify(proposal, storageRoot);

            markNonceAsUsed(proposal.originDomainID, proposal.depositNonce);
            try depositHandler.executeProposal(proposal.resourceID, proposal.data) returns (
                bytes memory handlerResponse
            ) {
                emit ProposalExecution(proposal.originDomainID, proposal.depositNonce, handlerResponse);
            } catch (bytes memory lowLevelData) {
                emit FailedHandlerExecution(lowLevelData, proposal.originDomainID, proposal.depositNonce);
                usedNonces[proposal.originDomainID][proposal.depositNonce / 256] &= ~(1 <<
                    (proposal.depositNonce % 256));
                continue;
            }
        }
    }


    /**
        @notice Executes a deposit proposal using a specified handler contract
        @param proposal Proposal which consists of:
        - originDomainID ID of chain deposit originated from.
        - resourceID ResourceID to be used when making deposits.
        - depositNonce ID of deposit generated by origin Bridge contract.
        - data Data originally provided when deposit was made.
        @notice Emits {ProposalExecution} event.
     */
    function executeProposal(
        Proposal memory proposal,
        bytes[] memory accountProof,
        uint256 slot
    ) public {
        Proposal[] memory proposalArray = new Proposal[](1);
        proposalArray[0] = proposal;

        executeProposals(proposalArray, accountProof, slot);
    }

    /**
        @notice Returns a boolean value.
        @param domainID ID of chain deposit originated from.
        @param depositNonce ID of deposit generated by origin Bridge contract.
        @return Boolean value depending if deposit nonce has already been used or not.
     */
    function isProposalExecuted(uint8 domainID, uint256 depositNonce) public view returns (bool) {
        return usedNonces[domainID][depositNonce / 256] & (1 << (depositNonce % 256)) != 0;
    }

    /**
        @notice Checks if the submitted propsal has been verified.
        @param proposal Proposal which consists of:
        - originDomainID ID of chain deposit originated from.
        - resourceID ResourceID to be used when making deposits.
        - depositNonce ID of deposit generated by origin Bridge contract.
        - data Data originally provided when deposit was made.
     */
    function verify(Proposal memory proposal, bytes32 storageRoot)
        internal
    {
        bytes32 transferHash;

        transferHash = keccak256(
            abi.encode(
                proposal.originDomainID,
                _domainID,
                proposal.securityModel,
                proposal.depositNonce,
                proposal.resourceID,
                keccak256(proposal.data)
            )
        );
        bytes32 slotKey = keccak256(abi.encode(keccak256(
            abi.encode(proposal.depositNonce, keccak256(
                abi.encode(_domainID, _originDomainIDMetadata[proposal.originDomainID].slotIndex))
            )
        )));

        bytes32 slotValue = StorageProof.getStorageValue(slotKey, storageRoot, proposal.storageProof);
        if (slotValue != transferHash) {
            revert TransferHashDoesNotMatchSlotValue(transferHash);
        }
    }

    /**
        @notice Marks a certain nonce that originated from a domain as used.
        @param originDomainID domain from which the proposal originated.
        @param depositNonce nonce of a proposal that should be marked as used.
     */
    function markNonceAsUsed(uint8 originDomainID, uint64 depositNonce) private {
        usedNonces[originDomainID][depositNonce / 256] |=
            1 <<
        (depositNonce % 256);
    }
}
