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
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

const STAKE: u32 = 0x17;
const UNSTAKE: u32 = 0x19;
const CLAIM: u32 = 0x21;
const COMPOUND: u32 = 0x23;

#[test]
fn proper_stake_action_call() {
    let dest = mock_address(30u8);

    let msg = StakeMsg { amount: 100 };

    let mut event_group = EventG