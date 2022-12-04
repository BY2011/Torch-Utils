
use std::collections::BTreeMap;

use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
};

use crate::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_multi_mint, execute_ownership_check, execute_revoke, execute_revoke_for_all,
        execute_set_base_uri, execute_transfer, execute_transfer_from, execute_update_minter,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, InitMsg, MintMsg, MultiMintMsg,
        RevokeForAllMsg, RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
    state::{MPC721ContractState, TokenInfo},
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
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (state, events) = execute_init(&mock_contract_context(2), &msg);
    assert_eq!(events.len(), 0);
    assert_eq!(
        state,
        MPC721ContractState {
            owner: None,
            name: "Cool Token".to_string(),
            symbol: "CTC".to_string(),
            base_uri: Some("ipfs://some.some".to_string()),
            minter: mock_address(1),
            supply: 0,
            tokens: BTreeMap::new(),
            operator_approvals: BTreeMap::new(),
        }
    );
}

#[test]
fn proper_set_base_uri() {
    let owner = 1u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetBaseUriMsg {
        new_base_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_base_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
    assert_eq!(state.base_uri, Some("ipfs://new.new".to_string()));
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn owner_is_not_set_on_set_base_uri() {
    let owner = 1u8;

    let msg = InitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetBaseUriMsg {
        new_base_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_base_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_owner_on_set_base_uri() {
    let owner = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetBaseUriMsg {
        new_base_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_base_uri(&mock_contract_context(alice), &mut state, &set_base_uri_msg);
}

#[test]
fn proper_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let token = state.token_info(1).unwrap();
    assert_eq!(
        *token,
        TokenInfo {
            owner: mock_address(alice),
            approvals: vec![],
            token_uri: None,
        }
    );
}
#[test]
fn proper_ownership_check() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);
    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);
    let ownership_msg: CheckOwnerMsg = CheckOwnerMsg {
        owner: mock_address(alice),
        token_id: 1,
    };
    let _ = execute_ownership_check(&mock_contract_context(2), &mut state, &ownership_msg);
}
#[test]
#[should_panic(expected = "Incorrect Owner")]
fn proper_ownership_check_fail() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let msg = InitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);
    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let ownership_msg: CheckOwnerMsg = CheckOwnerMsg {
        owner: mock_address(bob),
        token_id: 1,
    };
    let _ = execute_ownership_check(&mock_contract_context(2), &mut state, &ownership_msg);
}
#[test]