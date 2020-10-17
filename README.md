[![Tests and Linter checks](https://github.com/partisiablockchainapplications/CoreContracts/actions/workflows/basic.yml/badge.svg)](https://github.com/partisiablockchainapplications/CoreContracts/actions/workflows/basic.yml)

# Partisia Core Contracts

## Contracts

| Name                                                   | Description                                                               |
| ------------------------------------------------------ | ------------------------------------------------------------------------- |
| [`MPC20`](contracts/mpc20/)                            | Implementation of ERC20 Interface                                         |
| [`MPC721`](contracts/mpc721/)                          | Implementation of ERC721 Interface                                        |
| [`MPC1155`](contracts/mpc1155/)                        | Implementation of ERC1155 Interface                                       |
| [`MPC20-Staking`](contracts/mpc20-staking)             | Implementation of ERC20 Interface with staking mechanism                  |
| [`MPC20-BYOC`](contracts/mpc20-byoc)                   | Implementation of ERC20 Interface with wrapping mechanism for BYOC tokens |
| [`MPC721-Payble-Mint`](contracts/mpc721-payable-mint/) | Implementation of ERC721 Interface with payable mint feature              |
| [`MPC1-Multisig`](contracts/mpc1-multisig)             | On-chain multisig contract                                                |

## Packages

| Name                                                  | Description                                 |
| ----------------------------------------------------- | ------------------------------------------- |
| [`Access Control`](packages/access-control-base/)     | Access Control Smart Contract Extension Lib |
| [`Contract Version`](packages/contract-version-base/) | Contract Versioning Lib                     |
| [`Counter`](packages/counter-base/)                   | Counters Lib                                |
| [`Ownable`](packages/ownable-base/)                   | Ownable Smart Contract Extension Lib        |
| [`Pausable`](packages/pausable-base/)                 | Pausable Smart Contract Extension Lib       |
| [`Utils`](packages/utils/)                            | Set of tool for Smart Contracts             |

## Test

Run `cargo test` to run all unit tests

## How to build contracts