// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC20 {
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function approve(address spender, uint256 value) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
}

interface IUniswapV2Router {
    function swapExactTokensForTokens(uint amountIn, uint outMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory);
}

contract DustSweeper {
    address public immutable router;

    constructor(address _router) { router = _router; }

    function sweep(address target, address[] calldata tokens, uint256[] calldata amounts) external {
        for (uint i = 0; i < tokens.length; i++) {
            IERC20(tokens[i]).transferFrom(msg.sender, address(this), amounts[i]);
            IERC20(tokens[i]).approve(router, 0); 
            IERC20(tokens[i]).approve(router, amounts[i]);

            address[] memory path = new address[](2);
            path[0] = tokens[i];
            path[1] = target;

            IUniswapV2Router(router).swapExactTokensForTokens(amounts[i], 0, path, msg.sender, block.timestamp + 600);
        }
    }
}
