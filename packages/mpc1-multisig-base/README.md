# MPC1-MULTISIG-Base Contract

Base implementation of MPC1-MULTISIG contract.

# Actions

## execute_create_proposal

Creates a new proposal.

Pararms:

```json
CreateProposalMsg {
    "title": "Title",
    "description": "Decsription",
    "voting_phase_period": 86400,
    "calls": [
        "contr