use mpc1155_base::msg::{
    ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg, CheckBalancesMsg,
    MintMsg, RevokeForAllMsg, SetUriMsg, TokenMintInfoMsg, TokenTransferInfoMsg, TransferFromMsg,
};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};
use utils::events::IntoShort