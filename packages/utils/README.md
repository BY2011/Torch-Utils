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
    pub to: Address,
    pub amount: u128,
    pub memo: String,
    pub amounts: Vec<u128>,
}

impl IntoShortnameRPCEvent for TestTransferMsg {
    fn action_shortname(&self) -> u32 {
        0x01
    }
    fn as_interaction(
        &self,
        builder: &mut pbc_contract_common::events::EventGroupBuilder,
        dest: &Address,
    ) {
        builder
            .call(*dest, Shortname::from_u32(self.action_shortname()))
            .argument(self.to.clone())
            .argument(self.amount.clone())
            .argument(self.memo.clone())
            .argument(self.amounts.clone())
            .done();
    }
}
```

2. By using derive macro from `../rpc-msg-derive` crate