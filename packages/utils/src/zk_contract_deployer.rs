use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

use crate::contract_deployer::init_msg_signature;

// Contract Deployer address
pub const ZK_CONTRACT_DEPLOYER: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [
        0x8b, 0xc1, 0xcc, 0xbb, 0x67, 0x2b, 0x87, 0x71, 0x03, 0x27, 0x71, 0x3c, 0x97, 0xd4, 0x32,
        0x04, 0x90, 0x50, 0x82, 0xcb,
    ],
};

pub const MIN_MPC_STAKE: u64 = 2000_0000;

/// ## Description
/// Creates event that will deploy a new zero-knowledge contract.
/// Returns newly deployed contract address
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **event_group** is an object of type [`EventGroupBuilder`]
///
/// * **zkwa** is an object of type [`&[u8]`]