
use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};
use rust_decimal::prelude::*;

use crate::{
    msg::{ClaimMsg, CompoundMsg, Mpc20StakingInitMsg, StakeMsg, UnstakeMsg},
    state::MPC20StakingContractState,
    ContractError,
};

use mpc20_base::{
    actions::execute_init as mpc20_execute_init,
    msg::{Mpc20InitMsg, TransferFromMsg as Mpc20TransferFromMsg, TransferMsg as Mpc20TransferMsg},
    state::Minter as Mpc20Minter,
};
use utils::{decimal::DecimalRatio, events::IntoShortnameRPCEvent};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`Mpc20StakingInitMsg`]
pub fn execute_init(
    ctx: &ContractContext,
    msg: &Mpc20StakingInitMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    msg.validate();

    let deposit_token = if let Some(token) = msg.deposit_token {
        token
    } else {
        ctx.contract_address
    };

    let last_distributed = ctx.block_production_time as u64;

    let minter = msg.minter.map(|minter_addr| Mpc20Minter {
        minter: minter_addr,
        capacity: None,
    });

    let (mpc20, _) = mpc20_execute_init(
        ctx,
        &Mpc20InitMsg {
            info: msg.info.clone(),
            initial_balances: msg.initial_balances.clone(),
            minter,
        },
    );

    let state = MPC20StakingContractState {
        deposit_token,
        distribution_amount: msg.distribution_amount,
        distribution_epoch: msg.distribution_epoch,
        global_index: DecimalRatio::zero(),
        total_staked: 0,
        last_distributed,
        stakers: BTreeMap::new(),
        compound_frequency: msg.compound_frequency,
        mpc20,
    };

    (state, vec![])
}

/// ## Description
/// Stake specified amount of tokens to earn rewards.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20StakingContractState`]
///
/// * **msg** is an object of type [`StakeMsg`]
pub fn execute_stake(
    ctx: &ContractContext,
    state: &mut MPC20StakingContractState,
    msg: &StakeMsg,
) -> Vec<EventGroup> {
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_production_time as u64);
    staker.compute_reward(state.global_index);
    state.increase_stake_amount(&ctx.sender, &mut staker, msg.amount);

    let mut event_group = EventGroup::builder();
    Mpc20TransferFromMsg {
        from: ctx.sender,
        to: ctx.contract_address,
        amount: msg.amount,
    }
    .as_interaction(&mut event_group, &state.deposit_token);
