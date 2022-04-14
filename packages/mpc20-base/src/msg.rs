
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::{Minter, TokenInfo};

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc20 initial balances
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitialBalance {
    /// initial holder address
    pub address: Address,
    /// initial amount
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Mpc20InitMsg {
    /// mpc20 token information
    pub info: TokenInfo,