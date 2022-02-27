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

impl MPC1155ContractState {
    /// ## Description
    /// Sets new base uri
    /// ## Params
    /// * **uri** is an object of type [`str`]
    pub fn set_uri(&mut self, uri: &str) {
        self.uri = uri.to_string()
    }

    /// ## Description
    /// Stores new token at specified token id
    /// ## Params
    /// * **token_id** is an object of type [`u128`]
    ///
    /// * **info** is an object of type [`TokenInfo`]
    pub fn store_token(&mut self, token_id: u128, info: &TokenInfo) {
        self.tokens.entry(token_id).or_insert_with(|| info.clone());
    }

    /// ## Description
    /// Transfers token from owner to spender
    /// ## Params
    /// * **from** is an objec