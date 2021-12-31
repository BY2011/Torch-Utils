use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::Vote;
use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc1-multisig initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MultisigMember {
    /// multisig member address
    pub address: Address,
    /// member weight
    pub weight: u64,
}

/// ## Description
/// This structure describes fields for mpc1-multisig initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// multisig members
    pub members: Vec<MultisigMember>,
    /// required threshold
    pub threshold_weight: u64,
    /// voting phase period in UTC timestamp
    pub voting_phase_period: u64,
}

/// ## Description
/// This structure describes fields for 