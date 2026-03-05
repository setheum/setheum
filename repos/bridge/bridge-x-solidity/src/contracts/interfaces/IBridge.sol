// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "./IFeeHandler.sol";
import "./IAccessControlSegregator.sol";

/**
    @title Interface for Bridge contract.
    @author ChainSafe Systems.
 */
interface IBridge {
    /**
        @notice Exposing getter for {_domainID} instead of forcing the use of call.
        @return uint8 The {_domainID} that is currently set for the Bridge contract.
     */
    function _domainID() external view returns (uint8);

    /**
        @notice Exposing getter for {_feeHandler} instead of forcing the use of call.
        @return IFeeHandler The {_feeHandler} that is currently set for the Bridge contract.
     */
    function _feeHandler() external view returns (IFeeHandler);

    /**
        @notice Exposing getter for {_routerAddress} instead of forcing the use of call.
        @return address The {_routerAddress} that is currently set for the Bridge contract.
     */
    function _routerAddress() external returns (address);

        /**
        @notice Exposing getter for {_executorAddress} instead of forcing the use of call.
        @return address The {_executorAddress} that is currently set for the Bridge contract.
     */
    function _executorAddress() external returns (address);

    /**
        @notice Exposing getter for {_accessControl} instead of forcing the use of call.
        @return IAccessControlSegregator The {_accessControl} that is currently set for the Bridge contract.
     */
    function _accessControl() external view returns (IAccessControlSegregator);

    /**IFeeHandler
        @notice Exposing getter for {_resourceIDToHandlerAddress}.
        @param resourceID ResourceID to be used when making deposits.
        @return address The {handlerAddress} that is currently set for the resourceID.
     */
    function _resourceIDToHandlerAddress(bytes32 resourceID) external view returns (address);

    /**
        @notice Exposing getter for {paused} instead of forcing the use of call.
        @return bool The {paused} status that is currently set for the Bridge contract.
     */
    function paused() external view returns (bool);
}
