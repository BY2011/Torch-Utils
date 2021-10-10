use std::collections::BTreeMap;

use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use crate::{
    msg::{CreateProposalMsg, InitMsg, ProposalCloseMsg, ProposalExecuteMsg, ProposalVoteMsg},
    state::{
        Ballot, MPC1MultisigContractState, Proposal, ProposalExecuteCall, SubmittedVotes,
        ACCEPTED_STATUS, EXECUTED_STATUS, REJECTED_STATUS, VOTING_PHASE_STATUS, YES_VOTE,
    },
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined i