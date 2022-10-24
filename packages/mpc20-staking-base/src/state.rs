use rust_decimal::prelude::*;
use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use mpc20_base::state::MPC20ContractState;
use utils::decimal::DecimalRatio;

/// ## Description
/// This structure describes main mpc20-staking contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC20StakingContractState {
    /// deposit token address
    pub deposit_token: Address,
    /// per epoch distribution amount
    pub distribution_amount: u128,
    /// UTC timestamp
    pub distribution_epoch: u64,

    /// global index for calculating users share
    pub global_index: DecimalRatio,
    /// total amount of tokens staked
    pub total_staked: u128,
    /// UTC timestamp of last distribution
    pub last_distributed: u64,

    /// information about stakers
    pub stakers: BTreeMap<Address, Staker>,
    /// compounding limit
    pub compound_frequency: u64,
    /// mpc20 base state
    pub mpc20: MPC20ContractState,
}

impl MPC20StakingContractState {
    /// ## Description
    /// Distributes rewards by recalculting global index
    /// ## Params
    /// * **block_time** is an object of type [`u64`]
    pub fn distribute_rewards(