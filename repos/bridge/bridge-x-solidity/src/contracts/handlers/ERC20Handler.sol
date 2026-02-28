// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "../interfaces/IHandler.sol";
import "./ERCHandlerHelpers.sol";
import "../ERC20Safe.sol";

/**
    @title Handles ERC20 deposits and deposit executions.
    @author ChainSafe Systems.
    @notice This contract is intended to be used with the Bridge contract.
 */
contract ERC20Handler is IHandler, ERCHandlerHelpers, ERC20Safe {
    /**
        @param bridge Contract address of previously deployed Bridge.
     */
    constructor(
        IBridge bridge
    ) ERCHandlerHelpers(
        bridge
    ) {}

    modifier onlyBridge() {
        _onlyBridge();
        _;
    }

    modifier onlyRouter() {
        _onlyRouter();
        _;
    }

    modifier onlyExecutor() {
        _onlyExecutor();
        _;
    }

    function _onlyBridge() private {
        if (msg.sender != address(_bridge)) revert SenderNotBridgeContract();
    }

    function _onlyExecutor() private {
        if (msg.sender != _bridge._executorAddress()) revert SenderNotExecutorContract();
    }

    function _onlyRouter() private {
        if (msg.sender != _bridge._routerAddress()) revert SenderNotRouterContract();
    }

    /**
        @notice A deposit is initiated by making a deposit in the Bridge contract.
        @param resourceID ResourceID used to find address of token to be used for deposit.
        @param depositor Address of account making the deposit in the Bridge contract.
        @param data Consists of {amount} padded to 32 bytes.
        @notice Data passed into the function should be constructed as follows:
        amount                                      uint256     bytes   0 - 32
        destinationRecipientAddress     length      uint256     bytes  32 - 64
        destinationRecipientAddress                 bytes       bytes  64 - END
        @dev Depending if the corresponding {tokenAddress} for the parsed {resourceID} is
        marked true in {_tokenContractAddressToTokenProperties[tokenAddress].isBurnable},
        deposited tokens will be burned, if not, they will be locked.
     */
    function deposit(
        bytes32 resourceID,
        address depositor,
        bytes calldata data
    ) external override onlyRouter returns (bytes memory) {
        uint256 amount;
        uint256 destinationRecipientAddressLen;
        bytes memory destinationRecipientAddress;
        (amount, destinationRecipientAddressLen) = abi.decode(data, (uint, uint));
        destinationRecipientAddress = bytes(data[64:64 + destinationRecipientAddressLen]);

        address tokenAddress = _resourceIDToTokenContractAddress[resourceID];
        if (!_tokenContractAddressToTokenProperties[tokenAddress].isWhitelisted)
            revert ContractAddressNotWhitelisted(tokenAddress);

        if (_tokenContractAddressToTokenProperties[tokenAddress].isBurnable) {
            burnERC20(tokenAddress, depositor, amount);
        } else {
            lockERC20(tokenAddress, depositor, address(this), amount);
        }

        return abi.encodePacked(
            convertToInternalBalance(tokenAddress, amount),
            destinationRecipientAddressLen,
            destinationRecipientAddress
        );
    }

    /**
        @notice Proposal execution should be initiated when a proposal is finalized in the Bridge contract.
        by a relayer on the deposit's destination chain.
        @param resourceID ResourceID to be used when making deposits.
        @param data Consists of {resourceID}, {amount}, {lenDestinationRecipientAddress},
        and {destinationRecipientAddress} all padded to 32 bytes.
        @notice Data passed into the function should be constructed as follows:
        amount                                 uint256     bytes  0 - 32
        destinationRecipientAddress length     uint256     bytes  32 - 64
        destinationRecipientAddress            bytes       bytes  64 - END
     */
    function executeProposal(
        bytes32 resourceID,
        bytes calldata data
    ) external override onlyExecutor returns (bytes memory) {
        uint256 amount;
        uint256 lenDestinationRecipientAddress;
        bytes memory destinationRecipientAddress;

        (amount, lenDestinationRecipientAddress) = abi.decode(data, (uint256, uint256));
        destinationRecipientAddress = bytes(data[64:64 + lenDestinationRecipientAddress]);

        bytes20 recipientAddress;
        address tokenAddress = _resourceIDToTokenContractAddress[resourceID];

        assembly {
            recipientAddress := mload(add(destinationRecipientAddress, 0x20))
        }

        if (!_tokenContractAddressToTokenProperties[tokenAddress].isWhitelisted)
            revert ContractAddressNotWhitelisted(tokenAddress);

        if (_tokenContractAddressToTokenProperties[tokenAddress].isBurnable) {
            mintERC20(tokenAddress, address(recipientAddress), convertToExternalBalance(tokenAddress, amount));
        } else {
            releaseERC20(tokenAddress, address(recipientAddress), convertToExternalBalance(tokenAddress, amount));
        }
        return abi.encode(tokenAddress, address(recipientAddress), amount);
    }

    /**
        @notice Used to manually release ERC20 tokens from ERC20Safe.
        @param data Consists of {tokenAddress}, {recipient}, and {amount} all padded to 32 bytes.
        @notice Data passed into the function should be constructed as follows:
        tokenAddress                           address     bytes  0 - 32
        recipient                              address     bytes  32 - 64
        amount                                 uint256     bytes  64 - 96
     */
    function withdraw(bytes memory data) external onlyBridge {
        address tokenAddress;
        address recipient;
        uint256 amount;

        (tokenAddress, recipient, amount) = abi.decode(data, (address, address, uint256));

        releaseERC20(tokenAddress, recipient, amount);
    }

    /**
        @notice Sets {_resourceIDToContractAddress} with {contractAddress},
        {_tokenContractAddressToTokenProperties[tokenAddress].resourceID} with {resourceID} and
        {_tokenContractAddressToTokenProperties[tokenAddress].isWhitelisted} to true for
        {contractAddress} in ERCHandlerHelpers contract.
        Sets decimals value for contractAddress if value is provided in args.
        @param resourceID ResourceID to be used when making deposits.
        @param contractAddress Address of contract to be called when a deposit is made and a deposited is executed.
        @param args Additional data to be passed to specified handler.
     */
    function setResource(bytes32 resourceID, address contractAddress, bytes calldata args) external onlyBridge {
        _setResource(resourceID, contractAddress);

        if (args.length > 0) {
            uint8 externalTokenDecimals = uint8(bytes1(args));
            _setDecimals(contractAddress, externalTokenDecimals);
        }
    }

    /**
        @notice First verifies {contractAddress} is whitelisted, then sets
        {_tokenContractAddressToTokenProperties[contractAddress].isBurnable} to true.
        @param contractAddress Address of contract to be used when making or executing deposits.
     */
    function setBurnable(address contractAddress) external onlyBridge {
        _setBurnable(contractAddress);
    }
}
