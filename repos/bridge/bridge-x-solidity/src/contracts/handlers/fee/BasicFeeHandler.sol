// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "../../interfaces/IFeeHandler.sol";
import "../../interfaces/IBridge.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

import "../FeeHandlerRouter.sol";

/**
    @title Handles deposit fees.
    @author ChainSafe Systems.
    @notice This contract is intended to be used with the Bridge contract.
 */
contract BasicFeeHandler is IFeeHandler, AccessControl {
    IBridge public immutable _bridge;
    address public immutable _feeHandlerRouterAddress;
    // domainID => resourceID => securityModel => fee
    mapping (uint8 => mapping(bytes32 => mapping(uint8 => uint256))) public _domainResourceIDSecurityModelToFee;


    event FeeChanged(uint256 newFee);

    error SenderNotBridgeOrRouter();
    error IncorrectFeeSupplied(uint256);
    error ZeroAddressProvided();
    error SenderNotAdmin();
    error CannotRenounceOneself();
    error NewFeeEqualsCurrentFee(uint256 currentFee);
    error AddressesAndAmountsArraysDifferentLength(
        uint256 addressesLength,
        uint256 amountsLength
    );
    error EtherFeeTransferFailed();

    modifier onlyAdmin() {
        if (!hasRole(DEFAULT_ADMIN_ROLE, msg.sender)) revert SenderNotAdmin();
        _;
    }

    modifier onlyRouterOrFeeRouter() {
        _onlyRouterOrFeeRouter();
        _;
    }

    function _onlyRouterOrFeeRouter() private {
        if (msg.sender != _feeHandlerRouterAddress &&
            msg.sender != _bridge._routerAddress()
        ) revert SenderNotBridgeOrRouter();
    }

    /**
        @param bridge Contract address of previously deployed Bridge.
        @param feeHandlerRouterAddress Contract address of previously deployed FeeHandlerRouter.
     */
    constructor(IBridge bridge, address feeHandlerRouterAddress) {
        if (address(bridge) == address(0) ||
            feeHandlerRouterAddress == address(0)
        ) revert ZeroAddressProvided();

        _bridge = bridge;
        _feeHandlerRouterAddress = feeHandlerRouterAddress;
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /**
        @notice Removes admin role from {_msgSender()} and grants it to {newAdmin}.
        @notice Only callable by an address that currently has the admin role.
        @param newAdmin Address that admin role will be granted to.
     */
    function renounceAdmin(address newAdmin) external {
        if (newAdmin == address(0)) revert ZeroAddressProvided();
        address sender = _msgSender();
        if (sender == newAdmin) revert CannotRenounceOneself();
        grantRole(DEFAULT_ADMIN_ROLE, newAdmin);
        renounceRole(DEFAULT_ADMIN_ROLE, sender);
    }

    /**
        @notice Collects fee for deposit.
        @param sender Sender of the deposit.
        @param fromDomainID ID of the source chain.
        @param destinationDomainID ID of chain deposit will be bridged to.
        @param resourceID ResourceID to be used when making deposits.
        @param securityModel Security model to be used when making deposits.
        @param depositData Additional data to be passed to specified handler.
        @param feeData Additional data to be passed to the fee handler.
     */
    function collectFee(
        address sender,
        uint8 fromDomainID,
        uint8 destinationDomainID,
        bytes32 resourceID,
        uint8 securityModel,
        bytes calldata depositData,
        bytes calldata feeData
    ) external virtual payable onlyRouterOrFeeRouter {
        uint256 currentFee = _domainResourceIDSecurityModelToFee[destinationDomainID][resourceID][securityModel];
        if (msg.value != currentFee) revert IncorrectFeeSupplied(msg.value);
        emit FeeCollected(sender, fromDomainID, destinationDomainID, resourceID, currentFee, address(0));
    }

    /**
        @notice Calculates fee for deposit.
        @param sender Sender of the deposit.
        @param fromDomainID ID of the source chain.
        @param destinationDomainID ID of chain deposit will be bridged to.
        @param resourceID ResourceID to be used when making deposits.
        @param securityModel Security model to be used when making deposits.
        @param depositData Additional data to be passed to specified handler.
        @param feeData Additional data to be passed to the fee handler.
        @return Returns the fee amount.
     */
    function calculateFee(
        address sender,
        uint8 fromDomainID,
        uint8 destinationDomainID,
        bytes32 resourceID,
        uint8 securityModel,
        bytes calldata depositData,
        bytes calldata feeData
    ) virtual external  view returns(uint256, address) {
        return (_domainResourceIDSecurityModelToFee[destinationDomainID][resourceID][securityModel], address(0));
    }

    /**
        @notice Maps the {newFee} to {destinantionDomainID} to {resourceID} to
        {securityModel} in {_domainResourceIDSecurityModelToFee}.
        @notice Only callable by admin.
        @param destinationDomainID ID of chain fee will be set.
        @param resourceID ResourceID for which fee will be set.
        @param securityModel securityModel for which fee will be set.
        @param newFee Value to which fee will be updated to for the provided {destinantionDomainID} and {resourceID}.
     */
    function changeFee(
            uint8 destinationDomainID,
            bytes32 resourceID,
            uint8 securityModel,
            uint256 newFee
        ) external onlyAdmin {
        uint256 currentFee = _domainResourceIDSecurityModelToFee[destinationDomainID][resourceID][securityModel];
        if (currentFee == newFee) revert NewFeeEqualsCurrentFee(currentFee);
        _domainResourceIDSecurityModelToFee[destinationDomainID][resourceID][securityModel] = newFee;
        emit FeeChanged(newFee);
    }

    /**
        @notice Transfers eth in the contract to the specified addresses.
        The parameters addrs and amounts are mapped 1-1.
        This means that the address at index 0 for addrs will receive the amount (in WEI) from amounts at index 0.
        @param addrs Array of addresses to transfer {amounts} to.
        @param amounts Array of amounts to transfer to {addrs}.
     */
    function transferFee(address payable[] calldata addrs, uint256[] calldata amounts) external onlyAdmin {
        if (addrs.length != amounts.length) revert AddressesAndAmountsArraysDifferentLength(
            addrs.length,
            amounts.length
        );
        for (uint256 i = 0; i < addrs.length; i++) {
            (bool success, ) = addrs[i].call{value: amounts[i]}("");
            if (!success) revert EtherFeeTransferFailed();
            emit FeeDistributed(address(0), addrs[i], amounts[i]);
        }
    }
}
