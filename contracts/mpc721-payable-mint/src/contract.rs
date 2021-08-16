
use crate::{msg::PayableMintInitMsg, state::ContractState};

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    address::Address,
    context::{CallbackContext, ContractContext},
    events::EventGroup,
};

use mpc20_base::msg::{TransferFromMsg as MPC20TransferFromMsg, TransferMsg as MPC20TransferMsg};
use mpc721_base::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_multi_mint, execute_ownership_check, execute_revoke, execute_revoke_for_all,
        execute_set_base_uri, execute_transfer, execute_transfer_from, execute_update_minter,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, MintMsg, MultiMintMsg,
        RevokeForAllMsg, RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
};
use utils::events::{assert_callback_success, build_msg_callback, IntoShortnameRPCEvent};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(
    ctx: ContractContext,
    msg: PayableMintInitMsg,
) -> (ContractState, Vec<EventGroup>) {
    assert!(
        msg.payable_mint_info.amount > 0,
        "Payable amount for mint must be a non-zero value"
    );
    assert!(
        msg.mpc721.owner.is_some(),
        "Payable mpc721 version must have an owner"
    );

    let (mpc721, events) = execute_init(&ctx, &msg.mpc721);
    let state = ContractState {
        mpc721,
        payable_mint_info: msg.payable_mint_info,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    to: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer(&ctx, &mut state.mpc721, &TransferMsg { to, token_id });

    (state, events)
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(
        &ctx,
        &mut state.mpc721,