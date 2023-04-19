# OVERLAY projects smart contract

This is the [Concordium](https://concordium.com/) smart contract modelling projects listed in
[OVERLAY](https://overlay.global/).

This smart contract module stores project data of [OVERLAY](https://overlay.global/).

# How to build

## Prerequisite

You need to install the following tools to build this smart contract source codes.

1. [rustup](https://rustup.rs/)
2. [cargo-concordium](https://developer.concordium.software/en/mainnet/net/installation/downloads-testnet.html#cargo-concordium-testnet)

Please refer to the [Concordium official Quick start guide](https://developer.concordium.software/en/mainnet/smart-contracts/guides/quick-start.html)
for more information.

## Build

* Hit the following command to build.

```shell
% cargo concordium build
```

Then you can find wasm file built under the following directory.

```shell
 % ls ./target/concordium/wasm32-unknown-unknown/release/overlay_projects.wasm.v1 
./target/concordium/wasm32-unknown-unknown/release/overlay_projects.wasm.v1
```

# How to run unit test

* Hit the following command to execute all unit tests. 

```shell
% cargo concordium test
```

# LICENSE

see [LICENSE](./LICENSE) file.
