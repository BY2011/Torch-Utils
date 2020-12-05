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
const APPROVE_FOR_ALL: u32 = 0x05;
const SET_URI: u32 = 0x07;
const MINT: u32 = 0x09;
const BATCH_MINT: u32 = 0x11;
const BURN: u32 = 0x13;
const BATCH_BURN: u32 = 0x15;
const REVOKE_FOR_ALL: u32 = 0x17;
const CHECK_BALANCES: u32 = 0x18;

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = Tra