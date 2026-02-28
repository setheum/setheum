// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "../interfaces/IFeeHandler.sol";
import "../interfaces/IBridge.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/**
    @title Handles FeeHandler routing for resources.
    @author ChainSafe Systems.
    @notice This contract is intended to be used with the Bridge contract.
 */
contract FeeHandlerRouter is IFeeHandler, AccessControl {
    IBridge public immutable _bridge;

    // domainID => resourceID => securityModel => feeHandlerAddress
    mapping(uint8 => mapping(bytes32 => mapping(uint8 => IFeeHandler))) public
        _domainResourceIDSecurityModelToFeeHandlerAddress;
    // whitelisted address => is whitelisted
    mapping(address => bool) public _whitelist;

    event FeeChanged(uint256 newFee);

    event WhitelistChanged(address whitelistAddress, bool isWhitelisted);

    error IncorrectFeeSupplied(uint256);
    error SenderNotRouterContract();
    error SenderNotAdmin();

    modifier onlyRouter() {
        _onlyRouter();
        _;
    }

    function _onlyRouter() private {
        if (msg.sender != _bridge._routerAddress()) revert SenderNotRouterContract();
    }

    modifier onlyAdmin() {
        _onlyAdmin();
        _;
    }

    function _onlyAdmin() private view {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) revert SenderNotAdmin();
    }

    /**
        @param bridge Contract address of previously deployed Bridge.
     */
    constructor(IBridge bridge) {
        _bridge = bridge;
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /**
        @notice Sets or revokes fee whitelist from an address.
        @param whitelistAddress Address to be whitelisted.
        @param isWhitelisted Set to true to exempt an address from paying fees.
     */
    function adminSetWhitelist(address whitelistAddress, bool isWhitelisted) external onlyAdmin {
        _whitelist[whitelistAddress] = isWhitelisted;

        emit WhitelistChanged(whitelistAddress, isWhitelisted);
    }

    /**
        @notice Maps the {handlerAddress} to {securityModel} to {resourceID} to
        {destinantionDomainID} in {_domainResourceIDSecurityModelToFeeHandlerAddress}.
        @param destinationDomainID ID of chain FeeHandler contracts will be called.
        @param resourceID ResourceID for which the corresponding FeeHandler will collect/calcualte fee.
        @param securityModel Security model for which fee handler address will be set.
        @param handlerAddress Address of FeeHandler which will be called for specified resourceID.
     */
    function adminSetResourceHandler(
        uint8 destinationDomainID,
        bytes32 resourceID,
        uint8 securityModel,
        IFeeHandler handlerAddress
    ) external onlyAdmin {
        _domainResourceIDSecurityModelToFeeHandlerAddress[destinationDomainID][resourceID][securityModel] =
            handlerAddress;
    }

    /**
        @notice Initiates collecting fee with corresponding fee handler contract using IFeeHandler interface.
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
    ) external payable onlyRouter {
        if (_whitelist[sender]) {
            if (msg.value != 0) revert IncorrectFeeSupplied(msg.value);
            return;
        }
        IFeeHandler feeHandler =
            _domainResourceIDSecurityModelToFeeHandlerAddress[destinationDomainID][resourceID][securityModel];

        feeHandler.collectFee{value: msg.value}(
            sender,
            fromDomainID,
            destinationDomainID,
            resourceID,
            securityModel,
            depositData,
            feeData
        );
    }

    /**
        @notice Initiates calculating fee with corresponding fee handler contract using IFeeHandler interface.
        @param sender Sender of the deposit.
        @param fromDomainID ID of the source chain.
        @param destinationDomainID ID of chain deposit will be bridged to.
        @param resourceID ResourceID to be used when making deposits.
        @param securityModel Security model to be used when making deposits.
        @param depositData Additional data to be passed to specified handler.
        @param feeData Additional data to be passed to the fee handler.
        @return fee Returns the fee amount.
        @return tokenAddress Returns the address of the token to be used for fee.
     */
    function calculateFee(
        address sender,
        uint8 fromDomainID,
        uint8 destinationDomainID,
        bytes32 resourceID,
        uint8 securityModel,
        bytes calldata depositData,
        bytes calldata feeData
    ) external view returns (uint256 fee, address tokenAddress) {
        if (_whitelist[sender]) {
            return (0, address(0));
        }
        IFeeHandler feeHandler =
            _domainResourceIDSecurityModelToFeeHandlerAddress[destinationDomainID][resourceID][securityModel];

        return feeHandler.calculateFee(
            sender,
            fromDomainID,
            destinationDomainID,
            resourceID,
            securityModel,
            depositData,
            feeData
        );
    }
}
