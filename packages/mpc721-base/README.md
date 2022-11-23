
# MPC721-Base Contract

Base implementation of MPC721 contract.

# Actions

## execute_set_base_uri

Set base uri for the tokens.

Params:

```json
SetBaseUriMsg {
    "new_base_uri": "<uri>",
}
```

## execute_mint

Mint a new token. Can only be executed by minter account.
