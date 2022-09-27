# MPC20-Staking-Base Contract

Base implementation of MPC20-STAKING contract.

# Actions

## execute_stake

Stake specified amount of tokens to earn rewards.

Pararms:

```json
StakeMsg {
    amount: 10,
}
```

## execute_unstake

Withdraw staked tokens.

Pararms:

```json
UnstakeMsg {
    amount: 11,
}