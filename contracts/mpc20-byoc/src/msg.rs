use create_type_spec_derive::CreateTypeSpec;
use mpc20_base::msg::Mpc20InitMsg;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, P