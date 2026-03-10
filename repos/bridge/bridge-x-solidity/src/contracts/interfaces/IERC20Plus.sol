// SPDX-License-Identifier: MIT
pragma solidity 0.8.11;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// This is based on the example from:
// solhint-disable-next-line max-line-length
// https://github.com/AstarNetwork/astar-frame/blob/674356e7b611e561aaf9bf581452cab965cf8e87/examples/assets-erc20/XcBurrito.sol#L12

interface IERC20Plus is IERC20 {
    function mint(address beneficiary, uint256 amount) external;

    function burn(address who, uint256 amount) external;

    function decimals() external view returns (uint8);
}
