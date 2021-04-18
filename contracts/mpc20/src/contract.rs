use crate::state::TokenState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use mpc20_base::{
    actions::{
        execute_approve, execute_burn, execute_burn_from, execute_decrease_allowance,
        execute_increase_allowance, execute_init, execute_mint, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
        Mpc20InitMsg, TransferFromMsg, TransferMsg,
    },
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: Mpc20InitMsg) -> (TokenState, Vec<EventGroup>) {
    let (mpc20, events) = execute_init(&ctx, &msg);
    let state = TokenState {
        mpc20,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)