
use crate::state::ContractState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use mpc20_staking_base::{
    actions::{execute_claim, execute_compound, execute_init, execute_stake, execute_unstake},
    msg::{ClaimMsg, CompoundMsg, Mpc20StakingInitMsg, StakeMsg, UnstakeMsg},
};

use mpc20_base::{
    actions::{