// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity 0.8.11;

import "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";
import "./handlers/ERCHandlerHelpers.sol";
import "./interfaces/IERC20Plus.sol";
import "./interfaces/IBridge.sol";

contract NoArgument {
    event NoArgumentCalled();

    function noArgument() external {
        emit NoArgumentCalled();
    }
}

contract OneArgument {
    event OneArgumentCalled(uint256 indexed argumentOne);

    function oneArgument(uint256 argumentOne) external {
        emit OneArgumentCalled(argumentOne);
    }
}

contract TwoArguments {
    event TwoArgumentsCalled(address[] argumentOne, bytes4 argumentTwo);

    function twoArguments(address[] calldata argumentOne, bytes4 argumentTwo) external {
        emit TwoArgumentsCalled(argumentOne, argumentTwo);
    }
}

contract ThreeArguments {
    event ThreeArgumentsCalled(string argumentOne, int8 argumentTwo, bool argumentThree);

    function threeArguments(string calldata argumentOne, int8 argumentTwo, bool argumentThree) external {
        emit ThreeArgumentsCalled(argumentOne, argumentTwo, argumentThree);
    }
}

contract WithDepositor {
    event WithDepositorCalled(address argumentOne, uint256 argumentTwo);

    function withDepositor(address argumentOne, uint256 argumentTwo) external {
        emit WithDepositorCalled(argumentOne, argumentTwo);
    }
}

contract ReturnData {
    function returnData(string memory argument) external pure returns (bytes32 response) {
        assembly {
            response := mload(add(argument, 32))
        }
    }
}

abstract contract HandlerRevert is ERCHandlerHelpers {
    uint256 private _totalAmount;

    constructor(
        IBridge bridgeAddress
    ) ERCHandlerHelpers(
        bridgeAddress
    ) {}

    function executeProposal(bytes32, bytes calldata) external view {
        if (_totalAmount == 0) {
            revert("Something bad happened");
        }
        return;
    }

    function virtualIncreaseBalance(uint256 amount) external {
        _totalAmount = amount;
    }

    function setResource(bytes32 resourceID, address contractAddress, bytes calldata args) external {
        _setResource(resourceID, contractAddress);
    }
}

contract TestForwarder {
    function execute(bytes memory data, address to, address sender) external {
        bytes memory callData = abi.encodePacked(data, sender);
        (bool success, ) = to.call(callData);
        require(success, "Relay call failed");
    }
}

contract TestTarget {
    uint256 public calls = 0;
    uint256 public gasLeft;
    bytes public data;
    bool public burnAllGas;

    fallback() external payable {
        gasLeft = gasleft();
        calls++;
        data = msg.data;
        if (burnAllGas) {
            assert(false);
        }
    }

    function setBurnAllGas() public {
        burnAllGas = true;
    }
}

contract TestStore {
    mapping(bytes32 => bool) public _assetsStored;

    event AssetStored(bytes32 indexed asset);

    /**
    @notice Marks {asset} as stored.
    @param asset Hash of asset deposited.
    @notice {asset} must not have already been stored.
    @notice Emits {AssetStored} event.
   */
    function store(bytes32 asset) external {
        require(!_assetsStored[asset], "asset is already stored");

        _assetsStored[asset] = true;
        emit AssetStored(asset);
    }

    /**
    @notice Marks {asset} as stored.
    @param depositor Depositor address padded to 32 bytes.
    @param asset Hash of asset deposited.
    @param depositorCheck Depositor address (padded to 32 bytes) to check
      on destination chain if depositor passed through metadata is valid.
    @notice {asset} must not have already been stored.
    @notice Emits {AssetStored} event.
   */
    function storeWithDepositor(address depositor, bytes32 asset, address depositorCheck) external {
        require(!_assetsStored[asset], "asset is already stored");

        require(depositor == depositorCheck, "invalid depositor address");

        _assetsStored[asset] = true;
        emit AssetStored(asset);
    }
}

/**
  @dev This contract mocks ERC20PresetMinterPauser where and "transferFrom()" always fails
 */
contract ERC20PresetMinterPauserMock is ERC20PresetMinterPauser {
    constructor(string memory name, string memory symbol) ERC20PresetMinterPauser(name, symbol) {}

    function transferFrom(address from, address to, uint256 amount) public virtual override(ERC20) returns (bool) {
        address spender = _msgSender();
        _spendAllowance(from, spender, amount);
        _transfer(from, to, amount);
        return false;
    }
}

contract ERC20PresetMinterPauserDecimals is ERC20PresetMinterPauser {
    uint8 private immutable customDecimals;

    constructor(string memory name, string memory symbol, uint8 decimals) ERC20PresetMinterPauser(name, symbol) {
        customDecimals = decimals;
    }

    function decimals() public view virtual override(ERC20) returns (uint8) {
        return customDecimals;
    }
}

contract TestDeposit {
    event TestExecute(address depositor, uint256 num, address addr, bytes message);

    /**
        This helper can be used to prepare execution data for Bridge.deposit() on the source chain
        if PermissionlessGenericHandler is used
        and if the target function accepts (address depositor, bytes executionData).
        The execution data (packed as bytes) will be packed together with depositorAddress
        in PermissionlessGenericHandler before execution on the target chain.
        This function packs the bytes parameter together with a fake address and removes the address.
        After repacking in the handler together with depositorAddress, the offsets will be correct.
        Usage: pack all parameters as bytes, then use this function, then pack the result of this function
        together with maxFee, executeFuncSignature etc and pass it to Bridge.deposit().
    */
    function prepareDepositData(bytes calldata executionData) external view returns (bytes memory) {
        bytes memory encoded = abi.encode(address(0), executionData);
        return this.slice(encoded, 32);
    }

    function slice(bytes calldata input, uint256 position) public pure returns (bytes memory) {
        return input[position:];
    }

    function executePacked(address depositor, bytes calldata data) external {
        uint256 num;
        address[] memory addresses;
        bytes memory message;
        (num, addresses, message) = abi.decode(data, (uint256, address[], bytes));
        emit TestExecute(depositor, num, addresses[1], message);
    }

    function executeUnpacked(
        address depositor,
        uint256 num,
        address[] memory addresses,
        bytes memory message
    ) external {
        emit TestExecute(depositor, num, addresses[1], message);
    }
}

contract TestSpectre {
    struct SyncStepInput {
        uint64 attestedSlot;
        uint64 finalizedSlot;
        uint64 participation;
        bytes32 finalizedHeaderRoot;
        bytes32 executionPayloadRoot;
    }

    mapping(uint256 => bytes32) public executionPayloadRoots;

    function step(SyncStepInput calldata input, bytes calldata proof) external {
        executionPayloadRoots[input.finalizedSlot] = input.executionPayloadRoot;
    }

    function rotate(
        bytes calldata rotateProof,
        SyncStepInput calldata stepInput,
        bytes calldata stepProof
    ) external {}
}

contract StateRootStorage {
    mapping(uint8 => mapping(uint256 => bytes32)) public _stateRoots;

    function getStateRoot(uint8 domainID, uint256 blockNumber) public view returns (bytes32) {
        return _stateRoots[domainID][blockNumber];
    }

    function storeStateRoot(uint8 domainID, uint256 blockNumber, bytes32 stateRoot) public {
        _stateRoots[domainID][blockNumber] = stateRoot;
    }
}
