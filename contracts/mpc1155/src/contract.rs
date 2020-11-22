
use crate::state::ContractState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use mpc1155_base::{
    actions::{
        execute_approve_for_all, execute_batch_burn, execute_batch_mint,
        execute_batch_transfer_from, execute_burn, execute_check_balances, execute_init,
        execute_mint, execute_revoke_for_all, execute_set_uri, execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg,
        CheckBalancesMsg, InitMsg, MintMsg, RevokeForAllMsg, SetUriMsg, TokenMintInfoMsg,
        TokenTransferInfoMsg, TransferFromMsg,
    },
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (mpc1155, events) = execute_init(&ctx, &msg);
    let state = ContractState {
        mpc1155,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    token_info: TokenTransferInfoMsg,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(
        &ctx,
        &mut state.mpc1155,
        &TransferFromMsg {
            from,
            to,
            token_info,
        },
    );

    (state, events)
}

#[action(shortname = 0x03)]
pub fn batch_transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    token_infos: Vec<TokenTransferInfoMsg>,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_batch_transfer_from(
        &ctx,
        &mut state.mpc1155,
        &BatchTransferFromMsg {
            from,
            to,
            token_infos,
        },
    );

    (state, events)
}
