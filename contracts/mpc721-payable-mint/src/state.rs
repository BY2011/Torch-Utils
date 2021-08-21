use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use mpc721_base::state::MPC721ContractState;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[state]
#[derive(PartialEq, Eq