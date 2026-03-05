// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/utils/cryptography/draft-EIP712.sol";
import "./utils/Pausable.sol";

import "./interfaces/IERCHandler.sol";
import "./interfaces/IHandler.sol";
import "./interfaces/IFeeHandler.sol";
import "./interfaces/IAccessControlSegregator.sol";

/**
    @title Facilitates deposits and creation of deposit proposals, and deposit executions.
    @author ChainSafe Systems.
 */
contract Bridge is Pausable, Context {

    uint8 public immutable _domainID;

    IFeeHandler public _feeHandler;
    IAccessControlSegregator public _accessControl;
    address public _executorAddress;
    address public _routerAddress;


    // resourceID => handler address
    mapping(bytes32 => address) public _resourceIDToHandlerAddress;


    event FeeHandlerChanged(address newFeeHandler);
    event AccessControlChanged(address newAccessControl);


    error AccessNotAllowed(address sender, bytes4 funcSig);
    error ZeroAddressProvided();

    modifier onlyAllowed() {
        _onlyAllowed(msg.sig, _msgSender());
        _;
    }

    function _onlyAllowed(bytes4 sig, address sender) private view {
        if (!_accessControl.hasAccess(sig, sender)) revert AccessNotAllowed(sender, sig);
    }

    /**
        @notice Initializes Bridge, creates and grants {_msgSender()} the admin role,
        sets access control contract for bridge.
        @param domainID ID of chain the Bridge contract exists on.
        @param accessControl Address of access control contract.
     */
    constructor(uint8 domainID, address accessControl) {
        if (accessControl == address(0)) revert ZeroAddressProvided();

        _domainID = domainID;
        _accessControl = IAccessControlSegregator(accessControl);
    }

    /**
        @notice Pauses deposits, proposal creation and voting, and deposit executions.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
     */
    function adminPauseTransfers() external onlyAllowed {
        _pause(_msgSender());
    }

    /**
        @notice Unpauses deposits, proposal creation and voting, and deposit executions.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
     */
    function adminUnpauseTransfers() external onlyAllowed {
        _unpause(_msgSender());
    }

    /**
        @notice Sets a new resource for handler contracts that use the IERCHandler interface,
        and maps the {handlerAddress} to {resourceID} in {_resourceIDToHandlerAddress}.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param handlerAddress Address of handler resource will be set for.
        @param resourceID ResourceID to be used when making deposits.
        @param contractAddress Address of contract to be called when a deposit is made and a deposited is executed.
        @param args Additional data to be passed to specified handler.
     */
    function adminSetResource(
        address handlerAddress,
        bytes32 resourceID,
        address contractAddress,
        bytes calldata args
    ) external onlyAllowed {
        _resourceIDToHandlerAddress[resourceID] = handlerAddress;
        IHandler handler = IHandler(handlerAddress);
        handler.setResource(resourceID, contractAddress, args);
    }

    /**
        @notice Sets a resource as burnable for handler contracts that use the IERCHandler interface.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param handlerAddress Address of handler resource will be set for.
        @param tokenAddress Address of contract to be called when a deposit is made and a deposited is executed.
     */
    function adminSetBurnable(address handlerAddress, address tokenAddress) external onlyAllowed {
        IERCHandler handler = IERCHandler(handlerAddress);
        handler.setBurnable(tokenAddress);
    }

    /**
        @notice Changes access control contract address.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param newAccessControl Address {_accessControl} will be updated to.
     */
    function adminChangeAccessControl(address newAccessControl) external onlyAllowed {
        _accessControl = IAccessControlSegregator(newAccessControl);
        emit AccessControlChanged(newAccessControl);
    }

    /**
        @notice Changes deposit fee handler contract address.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param newFeeHandler Address {_feeHandler} will be updated to.
     */
    function adminChangeFeeHandler(address newFeeHandler) external onlyAllowed {
        _feeHandler = IFeeHandler(newFeeHandler);
        emit FeeHandlerChanged(newFeeHandler);
    }

    /**
        @notice Used to manually withdraw funds from ERC safes.
        @notice Only callable by address that has the right to call the specific function,
        which is mapped in {functionAccess} in AccessControlSegregator contract.
        @param handlerAddress Address of handler to withdraw from.
        @param data ABI-encoded withdrawal params relevant to the specified handler.
     */
    function adminWithdraw(address handlerAddress, bytes memory data) external onlyAllowed {
        IERCHandler handler = IERCHandler(handlerAddress);
        handler.withdraw(data);
    }

    function adminChangeRouterAddress(address routerAddress) external onlyAllowed {
        _routerAddress = routerAddress;
    }

    function adminChangeExecutorAddress(address executorAddress) external onlyAllowed {
        _executorAddress = executorAddress;
    }
}
