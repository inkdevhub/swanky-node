# Swanky Node :sunglasses:

Swanky node is a Substrate based blockchain configured to enable `pallet-contracts` (a smart contract module) and more features to help WASM smart contract development locally.

## Features
- [pallet-contracts](https://github.com/paritytech/substrate/tree/master/frame/contracts) (polkadot-0.9.33) and its unstable-feature are enabled by default.
- `grandpa` & `aura` consensus were removed. Instead, `instant-seal` & `manual-seal` are used.
  Blocks are authored & finalized (1) as soon as a transaction get in the pool (2) when `engine_createBlock` `engine_finalizeBlock` RPC called respectively.
- [pallet-dapps-staking](https://github.com/AstarNetwork/astar-frame/tree/polkadot-v0.9.33/frame/dapps-staking) and ChainExtension to interact with it.
- [pallet-assets](https://github.com/paritytech/substrate/tree/polkadot-v0.9.33/frame/assets).
- [pallet-rmrk](https://github.com/AstarNetwork/rmrk-substrate/tree/polkadot-v0.9.33) (core, equip, market) and chain extensions for pallet-rmrk-core.
- RMRK chain-extension
- Pallet-assets chain-extension
- dApps-staking chain-extension

It is optimized to local development purpose while removing unnecessary components such as P2P.
More features and pallets to interact with (Contract <-> Runtime) will be added.

## Compatible ink! version
ink! `v4.0.0-beta` or lower is supported by pallet-contract polkadot-0.9.33 branch.

## Installation
### Download Binary
The easiest way is to download a binary release from [Release Page](https://github.com/AstarNetwork/swanky-node/releases)

### Build Locally
First, complete the [basic Rust setup instructions](./docs/rust-setup.md).
After that, you can build node via
```bash
cargo build --release
```

### Embedded Docs :book:

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```bash
./target/release/swanky-node -h
```

## Usage
This command will start the single-node development chain with persistent state.
```bash
./target/release/swanky-node
```
If you want to run the node with non-persist mode, use tmp option.
```
./target/release/swanky-node --tmp
# or
./target/release/swanky-node --dev
```

Purge the development chain's state.
```bash
./target/release/swanky-node purge-chain
```

> Development **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/AstarNetwork/swanky-node/blob/main/node/src/chain_spec.rs#L44).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Charlie
> - Dave
> - Eve
> - Ferdie
> - Alice//stash
> - Bob//stash
> - Charlie//stash
> - Dave//stash
> - Eve//stash
> - Ferdie//stash

### Show only Errors and Contract Debug Output
To have only errors and contract debug output show up on the console you can supply
```
-lerror,runtime::contracts=debug
```
when starting the node.

Important: Debug output is only printed for RPC calls or off-chain tests ‒ not for transactions.

See ink! [FAQ](https://ink.substrate.io/faq/#how-do-i-print-something-to-the-console-from-the-runtime) for more details: How do I print something to the console from the runtime?.

### Connect with Polkadot-JS Apps Front-end

Once the swanky node is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local swanky node.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
mkdir .local # this is mounted by container
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/swanky-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/swanky-node --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/swanky-node purge-chain

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## Consensus (Manual Seal & Instant Seal)
Unlike other blockchains, Swanky node adopts block authioring and finalized gadget called Manual Seal and Instant Seal, consensus which is suitable for contracts development and testing.

Manual seal - Blocks are authored whenever RPC called.
Instant seal - Block are authored as soon as transactions get inside the pool, most often one transaction per block.

Swanky node enables both Manual seal and Instant seal.

### Manual Seal RPC calls
We can tell the node to author a block by calling the `engine_createBlock` RPC.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_createBlock",
      "params": [true, false, null]
    }'
```

#### Params
- **Create Empty**
`create_empty` is a Boolean value indicating whether empty blocks may be created. Setting `create-empty` to true does not mean that an empty block will necessarily be created. Rather it means that the engine should go ahead creating a block even if no transaction are present. If transactions are present in the queue, they will be included regardless of `create_empty`'s value.'

- **Finalize**
`finalize` is a Boolean indicating whether the block (and its ancestors, recursively) should be finalized after creation.

- **Parent Hash**
`parent_hash` is an optional hash of a block to use as a parent. To set the parent, use the format `"0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764"`. To omit the parent, use `null`. When the parent is omitted the block is built on the current best block. Manually specifying the parent is useful for constructing fork scenarios and demonstrating chain reorganizations.

#### Manually Finalizing Blocks
In addition to finalizing blocks while creating them, they can be finalized later by using the second provided RPC call, `engine_finalizeBlock`.

```bash
$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_finalizeBlock",
      "params": ["0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764", null]
    }'
```
