use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

// Contract Deployer address
pub const CONTRACT_DEPLOYER: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [
        0x97, 0xa0, 0xe2, 0x38, 0xe9, 0