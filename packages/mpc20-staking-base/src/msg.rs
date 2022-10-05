use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use mpc20_base::{msg::InitialBalance, state::TokenInfo};
use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc20-staking initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Mpc20StakingInitMsg {
    /// deposit token address, if None then deposit token will contract address
    pub deposit_token: Option<Address>,
    /// per epoch distribution amount
    pub distribution_amount: u128,
    /// UTC tim