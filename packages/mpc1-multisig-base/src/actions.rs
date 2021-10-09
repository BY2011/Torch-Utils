use std::collections::BTreeMap;

use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use crate::{
    msg::{CreateProposalMsg, InitMsg, ProposalCloseMsg, ProposalExecuteMsg, ProposalVoteMsg},
    state::{
        Ballot, MPC1MultisigContractState