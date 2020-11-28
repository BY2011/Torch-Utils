use mpc1155_base::msg::{
    ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg, CheckBalancesMsg,
    MintMsg, RevokeForAllMsg, SetUriMsg, TokenMintInfoMsg, TokenTransferInfoMsg, TransferFromMsg,
};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};
use utils::events::IntoShortnameRPCEvent;

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

const TRANSFER_FROM: u32 = 0x01;
const BATCH_TRANSFER_FROM: u32 = 0x03;
con