use mpc20_staking_base::msg::{ClaimMsg, CompoundMsg, StakeMsg, UnstakeMsg};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};
use utils::events::IntoShortnameRPCEv