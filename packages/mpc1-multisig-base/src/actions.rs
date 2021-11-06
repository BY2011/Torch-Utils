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
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`InitMsg`]
pub fn execute_init(
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC1MultisigContractState, Vec<EventGroup>) {
    assert!(
        !msg.members.is_empty(),
        "{}",
        ContractError::MembersListIsEmpty
    );
    assert!(
        msg.threshold_weight != 0,
        "{}",
        ContractError::RequiredWeightIsZero
    );

    let total_weight = msg.members.iter().map(|m| m.weight).sum();
    assert!(
        msg.threshold_weight <= total_weight,
        "{}",
        ContractError::UnreachableWeight
    );

    let mut members: BTreeMap<Address, u64> = BTreeMap::new();
    for member in msg.members.iter() {
        assert!(
            !members.contains_key(&member.address),
            "{}",
            ContractError::DuplicatedMember
        );
        assert!(member.weight > 0, "{}", ContractError::InvalidVotingPower);

        members.insert(member.address, member.weight);
    }

    let state = MPC1MultisigContractState {
        members,
        threshold_weight: msg.threshold_weight,
        total_weight,
        voting_phase_period: msg.voting_phase_period,
        proposals_count: 0,
        proposals: BTreeMap::new(),
    };

    (state, vec![])
}

/// ## Description
/// Creates a new proposal.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1MultisigContractState`]
///
/// * **msg** is an object of type [`CreateProposalMsg`]
pub fn execute_create_proposal(
    ctx: &ContractContext,
    state: &mut MPC1MultisigContractState,
    msg: &CreateProposalMsg,
) -> Vec<EventGroup> {
    assert!(
        state.members.contains_key(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );
    let member_power = *state.members.get(&ctx.sender).unwrap();

    let max_voting_phase = ctx.block_production_time as u64 + state.voting_phase_period;
    let voting_phase_end = if let Some(period) = msg.voting_phase_period {
        let voting_phase = ctx.block_production_time as u64 + period;
        assert!(
            voting_phase <= max_voting_phase,
            "{}",
            ContractError::InvalidVotingPhase
        );
        voting_phase
    } else {
        max_voting_phase
    };

    assert!(
        !msg.calls.is_empty(),
        "{}",
        ContractError::EmptyExecuteCallsList
    );

    let execute_calls: Vec<ProposalExecuteCall> = msg
        .calls
        .iter()
        .map(|call|