// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BiomisToken {
    string public name = "Biomis Token";
    string public symbol = "BIO";
    uint256 public totalSupply = 1000000;
    mapping(address => uint256) public balanceOf;

    constructor() {
        balanceOf[msg.sender] = totalSupply;
    }

    function transfer(address to, uint256 amount) public {
        require(balanceOf[msg.sender] >= amount, "Insufficient balance");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
    }
}
