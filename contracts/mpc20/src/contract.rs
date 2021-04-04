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
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, Mi