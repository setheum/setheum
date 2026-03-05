// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "@openzeppelin/contracts/utils/Context.sol";


import "./interfaces/IAccessControlSegregator.sol";
import "./interfaces/IBridge.sol";
import "./interfaces/IHandler.sol";
import "./interfaces/IFeeHandler.sol";



/**
    @title Facilitates proposal deposits.
    @author ChainSafe Systems.
 */
contract Router is Context {

    IBridge public immutable _bridge;
    IAccessControlSegregator public _accessControl;
    uint8 public immutable _domainID;

    // this is used to store the hash of the transfer data in
    // the EVM state so it can be proved in the Executor contract via state proof
    // domainID => nonce => transferHashes
    mapping(uint8 => mapping(uint256 => bytes32)) public transferHashes;
    // destinationDomainID => number of deposits
    mapping(uint8 => uint64) public _depositCounts;

    error ResourceIDNotMappedToHandler();
    error DepositToCurrentDomain();
    error AccessNotAllowed(address sender, bytes4 funcSig);
    error BridgeIsPaused();
    error ZeroAddressProvided();
    error NonceDecrementNotAllowed(uint64 currentNonce);
    error MsgValueNotZero();

    event Deposit(
        uint8 destinationDomainID,
        uint8 securityModel,
        bytes32 resourceID,
        uint64 depositNonce,
        address indexed user,
        bytes data
    );

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

    constructor(address bridge, address accessControl) {
        if (bridge == address(0) || accessControl == address(0)) revert ZeroAddressProvided();

        _bridge = IBridge(bridge);
        _accessControl = IAccessControlSegregator(accessControl);
        _domainID = IBridge(_bridge)._domainID();
    }

    /**
        @notice Sets the nonce for the specific domainID.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param domainID Domain ID for increasing nonce.
        @param nonce The nonce value to be set.
     */
    function adminSetDepositNonce(uint8 domainID, uint64 nonce) external onlyAllowed {
        uint64 currentNonce = _depositCounts[domainID];
        if (nonce <= currentNonce) revert NonceDecrementNotAllowed(currentNonce);
        _depositCounts[domainID] = nonce;
    }

    /**
        @notice Initiates a transfer using a specified handler contract.
        @notice Only callable when Bridge is not paused.
        @param destinationDomainID ID of chain deposit will be bridged to.
        @param resourceID ResourceID used to find address of handler to be used for deposit.
        @param depositData Additional data to be passed to specified handler.
        @param feeData Additional data to be passed to the fee handler.
        @notice Emits {Deposit} event with all necessary parameters.
        @return depositNonce deposit nonce for the destination domain.
     */
    function deposit(
        uint8 destinationDomainID,
        bytes32 resourceID,
        uint8 securityModel,
        bytes calldata depositData,
        bytes calldata feeData
    ) external payable whenBridgeNotPaused returns (uint64 depositNonce) {
        if (destinationDomainID == _domainID) revert DepositToCurrentDomain();
        address sender = _msgSender();
        IFeeHandler feeHandler = _bridge._feeHandler();
        address handler = _bridge._resourceIDToHandlerAddress(resourceID);

        if (handler == address(0)) revert ResourceIDNotMappedToHandler();

        if (address(feeHandler) == address(0)) {
            if (msg.value != 0) revert MsgValueNotZero();
        } else {
            // Reverts on failure
            feeHandler.collectFee{value: msg.value}(
                sender,
                _domainID,
                destinationDomainID,
                resourceID,
                securityModel,
                depositData,
                feeData
            );
        }
        depositNonce = ++_depositCounts[destinationDomainID];
        IHandler depositHandler = IHandler(handler);
        bytes memory handlerDepositData = depositHandler.deposit(resourceID, sender, depositData);
        transferHashes[destinationDomainID][depositNonce] = keccak256(
            abi.encode(
                _domainID,
                destinationDomainID,
                securityModel,
                depositNonce,
                resourceID,
                keccak256(handlerDepositData)
            )
        );
        emit Deposit(destinationDomainID, securityModel, resourceID, depositNonce, sender, handlerDepositData);
    }
}
