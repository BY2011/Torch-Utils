
use std::collections::BTreeMap;

use mpc20_base::{
    msg::{TransferFromMsg, TransferMsg},
    state::{MPC20ContractState, Minter, TokenInfo},
};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroup,
};
use utils::{decimal::DecimalRatio, events::IntoShortnameRPCEvent};

use crate::{
    actions::{execute_claim, execute_compound, execute_init, execute_stake, execute_unstake},
    msg::{ClaimMsg, CompoundMsg, Mpc20StakingInitMsg, StakeMsg, UnstakeMsg},
    state::{MPC20StakingContractState, Staker},
};

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

fn mock_contract_context(sender: u8, block_time: i64) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time,
        block_production_time: block_time,
        current_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
        original_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    }
}

#[test]
fn test_staking() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let mut block_production_time = 100;

    let msg = Mpc20StakingInitMsg {
        deposit_token: None,
        distribution_amount: 1_000,
        distribution_epoch: 10,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (mut state, events) =
        execute_init(&mock_contract_context(MINTER, block_production_time), &msg);
    assert_eq!(events, vec![]);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 0,
            last_distributed: 100,
            stakers: BTreeMap::new(),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),