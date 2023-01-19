# MPC Smart Contracts Utils

## Contract Deployer

Provides API for deploying new contracts on-chain.

## Decimal

Provides `DecimalRatio` type, for on-chain floating point calculations.

## Events

Provides API and Interfaces for building contract calls(interactions).

There is two ways how a contract call can be built using this API.

1. By implementing `IntoShortnameRPCEvent` trait on your own. Example:

```rust
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TestTransferMsg {
    pub to: Addr