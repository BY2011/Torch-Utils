use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use mpc20_base::{msg::InitialBalance, state::TokenInfo};
use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mp