// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ProofOfBiometricTransaction {
    event TransactionVerified(address indexed user);

    function verifyTransaction() public {
        emit TransactionVerified(msg.sender);
    }
}
