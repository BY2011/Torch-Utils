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
pub str