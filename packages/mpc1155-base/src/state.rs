use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes main mpc1155 contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC1155ContractState {
    /// optional owner address
    pub owner: Option<Address>,
    /// base uri for the tokens
    pub uri: String,
    /// minter address
    pub minter: Address,
    /// token holders balance
    pub balances: BTreeMap<u128, BTreeMap<Address, u128>>,
    /// token approvals
    pub operator_approvals: BTreeMap<Address, BTreeMap<Address, bool>>,
    /// token info by token id
    pub tokens: BTreeMap<u128, TokenInfo>,
}

/// ## Description
/// This structure describes minted mpc1155 token information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenInfo {
    /// optional token uri
    pub token_uri: Option<String>,
}

im