
use std::collections::BTreeMap;

use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
    events::EventGroup,
};

use crate::{
    actions::{
        execute_close_proposal, execute_create_proposal, execute_execute_proposal, execute_init,
        execute_vote,
    },
    msg::{
        CreateProposalMsg, InitMsg, MultisigMember, ProposalCloseMsg, ProposalExecuteCallMsg,
        ProposalExecuteMsg, ProposalVoteMsg,
    },
    state::{
        Ballot, MPC1MultisigContractState, Proposal, ProposalExecuteCall, SubmittedVotes,
        EXECUTED_STATUS, REJECTED_STATUS, VOTING_PHASE_STATUS, YES_VOTE,
    },
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

fn mock_transfer_base64_payload() -> String {
    "yu3VvwIACQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABk".to_string()
}

fn mock_transfer_payload_with_name_bytes() -> Vec<u8> {
    vec![
        202, 237, 213, 191, 2, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,
    ]
}

#[test]
fn proper_execute_init() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,