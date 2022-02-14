
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc1155 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// optional owner address
    pub owner: Option<Address>,
    /// base uri
    pub uri: String,
    /// minter address
    pub minter: Address,
}

/// ## Description
/// This structure describes fields for mpc1155 transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x01)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token info for transfer
    pub token_info: TokenTransferInfoMsg,
}

/// ## Description
/// This structure describes fields for mpc1155 batch transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct BatchTransferFromMsg {