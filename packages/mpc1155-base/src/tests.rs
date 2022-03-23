
use std::collections::BTreeMap;

use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
};

use crate::{
    actions::{
        execute_approve_for_all, execute_batch_burn, execute_batch_mint,
        execute_batch_transfer_from, execute_burn, execute_check_balances, execute_init,
        execute_mint, execute_revoke_for_all, execute_set_uri, execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg,
        CheckBalancesMsg, InitMsg, MintMsg, RevokeForAllMsg, SetUriMsg, TokenMintInfoMsg,
        TokenTransferInfoMsg, TransferFromMsg,
    },
    state::{MPC1155ContractState, TokenInfo},
};

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

fn mock_contract_context(sender: u8) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time: 100,
        block_production_time: 100,
        current_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
        original_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    }
}

#[test]
fn proper_execute_init() {
    let msg = InitMsg {
        owner: Some(mock_address(1)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (state, events) = execute_init(&mock_contract_context(2), &msg);
    assert_eq!(events.len(), 0);
    assert_eq!(
        state,
        MPC1155ContractState {
            owner: Some(mock_address(1)),
            uri: "ipfs://random".to_string(),
            minter: mock_address(3),
            balances: BTreeMap::new(),
            operator_approvals: BTreeMap::new(),
            tokens: BTreeMap::new(),
        }
    );
}

#[test]
fn proper_set_uri() {
    let owner = 1u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetUriMsg {
        new_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
    assert_eq!(state.uri, "ipfs://new.new".to_string());
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn owner_is_not_set_on_set_base_uri() {
    let owner = 1u8;

    let msg = InitMsg {
        owner: None,
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetUriMsg {
        new_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_owner_on_set_base_uri() {
    let owner = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetUriMsg {
        new_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_uri(&mock_contract_context(alice), &mut state, &set_base_uri_msg);
}

#[test]
fn proper_mint() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(
        state.tokens,
        BTreeMap::from([(
            1,
            TokenInfo {
                token_uri: Some("1.json".to_string()),
            }
        )])
    );
    assert_eq!(
        state.balances,
        BTreeMap::from([(1, BTreeMap::from([(mock_address(alice), 10)]))])
    );

    let mut state = state;
    for msg in vec![
        MintMsg {
            to: mock_address(alice),
            token_info: TokenMintInfoMsg {
                token_id: 2,
                amount: 20,
                token_uri: Some("2.json".to_string()),
            },
        },
        MintMsg {
            to: mock_address(alice),
            token_info: TokenMintInfoMsg {
                token_id: 1,
                amount: 50,
                token_uri: None,
            },
        },
        MintMsg {
            to: mock_address(bob),
            token_info: TokenMintInfoMsg {
                token_id: 1,
                amount: 1,
                token_uri: None,
            },