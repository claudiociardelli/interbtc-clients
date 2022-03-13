# Redeem Client

Issue command line tool.

## Responsibilities

## Getting Started

### Build
cargo build --release --features parachain-metadata-testnet (testnet2022)

### Run
Run the interBTC issue cli client:

--vault-account-id points to the vault where the issue should occur
--griefing 500000000 (= 0.5 Kint, testnet 2022)
(Dom) The griefing collateral - on request issue the account that requests needs to send a small number of griefing collateral in case that user does not execute the request and reserves the vault collateral without reason (this is to prevent DDoS attacks)
Other note: you need to send at least 1000 sat since that is the dust limit which you can get from the issue module as well 🙂
Getting the griefing collateral is a bit involved (check here if you are interested: https://github.com/interlay/interbtc-api/blob/c2840a247687f8e1aa2834f42eab4a2de0e8f72e/src/parachain/fee.ts#L60). One way to go around the issue is to just submit a bit extra KINT (say 0.5). The rest that's not needed will be refunded

```
./target/release/issue_request \
--keyfile ~/.mytestvault/keyfile.json \
--keyname interlaymaincustomeraccount \
--btc-parachain-url 'wss://api-testnet.interlay.io:443/parachain' \
--vault-account-id 5ECMdBzuWUqriRNp1M74nACfm2AWxm5w1SWtjcn5SJXtGeCq \
--amount 3020 \
--griefing 500000000
```

### Options

When using cargo to run this binary, arguments to cargo and the binary are separated by `--`. For example, to pass `--help` to the faucet to get a list of all command line options that is guaranteed to be up date, run:

```
cargo run -- --help
```

For convenience, a copy of this output is included below.
```
```
