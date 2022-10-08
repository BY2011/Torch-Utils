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
    /// UTC timestamp
    pub distribution_epoch: u64,
    /// compounding limit
    pub compound_frequency: u64,
    /// mpc20 base token info
    pub info: TokenInfo,
    /// mpc20 base initial balances
    pub initial_balances: Vec<InitialBalance>,
    /// mpc20 base optional minter address
    pub minter: Option<Address>,
}

impl Mpc20StakingInitMsg {
    pub fn validate(&self) {
        assert!(
            self.distribution_epoch > 0,
            "Distribution epo