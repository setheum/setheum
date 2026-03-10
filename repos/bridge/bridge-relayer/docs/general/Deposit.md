# Deposit
This document describes the expected format of the deposit for different networks
## BTC Deposit
## Format

### OP_RETURN Output

- **Purpose**: Stores arbitrary data within the transaction.
- **Requirements**:
  - There should be at most one output with a `ScriptPubKey.Type` of `OP_RETURN`.
  - The `OP_RETURN` data must be formatted as `receiverEVMAddress_destinationDomainID`.


### Amount Calculation

- The total deposit amount is calculated by summing the values of the outputs that match the resource address.
- Only outputs with script types of `witness_v1_taproot` are considered for the amount calculation.
