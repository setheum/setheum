// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.11;

import "../../interfaces/IBridge.sol";
import "../../interfaces/IERCHandler.sol";
import "../../ERC20Safe.sol";
import {BasicFeeHandler} from "./BasicFeeHandler.sol";

/**
    @title Handles deposit fees.
    @author ChainSafe Systems.
    @notice This contract is intended to be used with the Bridge contract.
 */
contract PercentageERC20FeeHandlerEVM is BasicFeeHandler, ERC20Safe {
    uint32 public constant HUNDRED_PERCENT = 1e8;

    /**
        @notice _domainResourceIDSecurityModelToFee[destinationDomainID][resourceID][securityModel]
        inherited from BasicFeeHandler in this implementation is in BPS and should be multiplied by
        10000 to avoid precision loss
     */
    struct Bounds {
        uint128 lowerBound; // min fee in token amount
        uint128 upperBound; // max fee in token amount
    }

    mapping(bytes32 => Bounds) public _resourceIDToFeeBounds;

    event FeeBoundsChanged(uint256 newLowerBound, uint256 newUpperBound);

    error MsgValueNotZero();
    error InvalidBoundsRatio(uint128 newLowerBound, uint128 newUpperBound);
    error NewBoundsEqualCurrentBounds(uint128 currentLowerBound, uint128 currentUpperBound);

    /**
        @param bridge Contract address of previously deployed Bridge.
        @param feeHandlerRouterAddress Contract address of previously deployed FeeHandlerRouter.
     */
    constructor(
        IBridge bridge,
        address feeHandlerRouterAddress
    ) BasicFeeHandler(bridge, feeHandlerRouterAddress) {}

    // Admin functions

    /**
        @notice Calculates fee for deposit.
        @param sender Sender of the deposit.
        @param fromDomainID ID of the source chain.
        @param destinationDomainID ID of chain deposit will be bridged to.
        @param resourceID ResourceID to be used when making deposits.
        @param securityModel Security model to be used when making deposits.
        @param depositData Additional data about the deposit.
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
    ) external view override returns (uint256 fee, address tokenAddress) {
        return _calculateFee(
            sender,
            fromDomainID,
            destinationDomainID,
            resourceID,
            securityModel,
            depositData,
            feeData
        );
    }

    function _calculateFee(
        address sender,
        uint8 fromDomainID,
        uint8 destinationDomainID,
        bytes32 resourceID,
        uint8 securityModel,
        bytes calldata depositData,
        bytes calldata feeData
    ) internal view returns (uint256 fee, address tokenAddress) {
        address tokenHandler = _bridge._resourceIDToHandlerAddress(resourceID);
        tokenAddress = IERCHandler(tokenHandler)._resourceIDToTokenContractAddress(resourceID);
        Bounds memory bounds = _resourceIDToFeeBounds[resourceID];

        uint256 depositAmount = abi.decode(depositData, (uint256));

        // 10000 for BPS and 10000 to avoid precision loss
        fee = depositAmount * _domainResourceIDSecurityModelToFee[
            destinationDomainID][resourceID][securityModel] / HUNDRED_PERCENT;

        if (fee < bounds.lowerBound) {
            fee = bounds.lowerBound;
        }
        // if upper bound is not set, fee is % of token amount
        else if (fee > bounds.upperBound && bounds.upperBound > 0) {
            fee = bounds.upperBound;
        }

        return (fee, tokenAddress);
    }

    /**
        @notice Collects fee for deposit.
        @param sender Sender of the deposit.
        @param fromDomainID ID of the source chain.
        @param destinationDomainID ID of chain deposit will be bridged to.
        @param resourceID ResourceID to be used when making deposits.
        @param securityModel Security model to be used when making deposits.
        @param depositData Additional data about the deposit.
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
    ) external payable override onlyRouterOrFeeRouter {
        if (msg.value != 0) revert MsgValueNotZero();

        (uint256 fee, address tokenAddress) = _calculateFee(
            sender,
            fromDomainID,
            destinationDomainID,
            resourceID,
            securityModel,
            depositData,
            feeData
        );
        lockERC20(tokenAddress, sender, address(this), fee);

        emit FeeCollected(sender, fromDomainID, destinationDomainID, resourceID, fee, tokenAddress);
    }

    /**
        @notice Sets new value for lower and upper fee bounds, both are in token amount.
        @notice Only callable by admin.
        @param resourceID ResourceID for which new fee bounds will be set.
        @param newLowerBound Value {_newLowerBound} will be updated to.
        @param newUpperBound Value {_newUpperBound} will be updated to.
     */
    function changeFeeBounds(bytes32 resourceID, uint128 newLowerBound, uint128 newUpperBound) external onlyAdmin {
        if (newUpperBound != 0 && (newUpperBound <= newLowerBound)) {
            revert InvalidBoundsRatio(newLowerBound, newUpperBound);
        }

        Bounds memory existingBounds = _resourceIDToFeeBounds[resourceID];
        if (existingBounds.lowerBound == newLowerBound && existingBounds.upperBound == newUpperBound) {
            revert NewBoundsEqualCurrentBounds(existingBounds.lowerBound, existingBounds.upperBound);
        }

        Bounds memory newBounds = Bounds(newLowerBound, newUpperBound);
        _resourceIDToFeeBounds[resourceID] = newBounds;

        emit FeeBoundsChanged(newLowerBound, newUpperBound);
    }

    /**
        @notice Transfers tokens from the contract to the specified addresses.
        The parameters addrs and amounts are mapped 1-1.
        This means that the address at index 0 for addrs will receive the amount of tokens from amounts at index 0.
        @param resourceID ResourceID of the token.
        @param addrs Array of addresses to transfer {amounts} to.
        @param amounts Array of amounts to transfer to {addrs}.
     */
    function transferERC20Fee(
        bytes32 resourceID,
        address[] calldata addrs,
        uint256[] calldata amounts
    ) external onlyAdmin {
        if (addrs.length != amounts.length) revert AddressesAndAmountsArraysDifferentLength(
            addrs.length,
            amounts.length
        );
        address tokenHandler = _bridge._resourceIDToHandlerAddress(resourceID);
        address tokenAddress = IERCHandler(tokenHandler)._resourceIDToTokenContractAddress(resourceID);
        for (uint256 i = 0; i < addrs.length; i++) {
            releaseERC20(tokenAddress, addrs[i], amounts[i]);
            emit FeeDistributed(tokenAddress, addrs[i], amounts[i]);
        }
    }
}
