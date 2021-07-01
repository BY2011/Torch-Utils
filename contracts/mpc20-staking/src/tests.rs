use mpc20_staking_base::msg::{ClaimMsg, CompoundMsg, StakeMsg, UnstakeMsg};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};
use utils::events::IntoShortnameRPCEvent;

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u