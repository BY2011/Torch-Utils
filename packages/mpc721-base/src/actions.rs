
use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, InitMsg, MintMsg, MultiMintMsg,
        RevokeForAllMsg, RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
    state::MPC721ContractState,
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`InitMsg`]
pub fn execute_init(
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    let state = MPC721ContractState {
        owner: msg.owner,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        base_uri: msg.base_uri.clone(),
        minter: msg.minter,
        supply: 0,
        tokens: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
    };

    (state, vec![])
}

/// ## Description
/// Set base uri for the tokens.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`SetBaseUriMsg`]
pub fn execute_set_base_uri(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &SetBaseUriMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    state.set_base_uri(&msg.new_base_uri);
    vec![]
}

/// ## Description
/// Mint a new token. Can only be executed by minter account.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`MintMsg`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    assert!(!state.is_minted(msg.token_id), "{}", ContractError::Minted);

    state.mint(msg.token_id, &msg.to, &msg.token_uri);
    state.increase_supply();

    vec![]