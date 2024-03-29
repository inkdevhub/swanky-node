# Swanky Node :sunglasses:

Swanky node is a Substrate based blockchain configured to enable `pallet-contracts` (a smart contract module) and more features to help WASM smart contract development locally.

## Features
- [pallet-contracts](https://github.com/paritytech/substrate/tree/master/frame/contracts) (polkadot-0.9.39).
- `grandpa` & `aura` consensus were removed. Instead, [`instant-seal`/`manual-seal`](https://github.com/AstarNetwork/swanky-node#consensus-manual-seal--instant-seal) & [`delayed-finalize`](https://github.com/AstarNetwork/swanky-node#consensus-delayed-finalize) are used.
  Blocks are sealed (1) as soon as a transaction get in the pool (2) when `engine_createBlock` RPC called. Blocks are finalized configured delay sec after blocks are sealed.
- Users' account Balance manipulation
- Block height manipulation. Developers can forward and revert blocks via RPC.
- [pallet-dapps-staking](https://github.com/AstarNetwork/Astar/tree/v5.15.0/pallets/dapps-staking) and ChainExtension to interact with it.
- [pallet-assets](https://github.com/paritytech/substrate/tree/polkadot-v0.9.43/frame/assets).
- Pallet-assets chain-extension
- dApps-staking chain-extension

Swanky Node is optimized for local development, while removing unnecessary components such as P2P. Additional features and pallets, such as to interact between (Contract <-> Runtime), will be added in the future.

## Compatible ink! version

Any ink! version from `v4.0.0` or `higher` is supported by pallet-contract polkadot-0.9.43 branch.

## Installation
### Download Binary
The easiest method of installation is by downloading and executing a precompiled binary from the [Release Page](https://github.com/AstarNetwork/swanky-node/releases)

### Build Locally
If you would like to build the source locally, you should first complete the [basic Rust setup instructions](https://github.com/AstarNetwork/swanky-node/blob/main/docs/rust-setup.md).
Once Rust is installed and configured, you will be able to build the node with:
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
This command will start the single-node development chain with a persistent state.
```bash
./target/release/swanky-node
```
If you would prefer to run the node in non-persistent mode, use tmp option.
```
./target/release/swanky-node --tmp
# or
./target/release/swanky-node --dev
```

Purge the development chain's state.
```bash
./target/release/swanky-node purge-chain
```

> The **alice** development account will be the authority and sudo account as declared in the
> [genesis state](https://github.com/AstarNetwork/swanky-node/blob/main/node/src/chain_spec.rs#L44).
> While at the same time, the following accounts will be pre-funded:
>
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
To print errors and contract debug output to the console log, supply `-lerror,runtime::contracts=debug` when starting the node.
```
-lerror,runtime::contracts=debug
```

Important: Debug output is only printed for RPC calls or off-chain tests ‒ not for transactions.

See the ink! [FAQ](https://ink.substrate.io/faq/#how-do-i-print-something-to-the-console-from-the-runtime) for more details: How do I print something to the console from the runtime?.

### Connect with Polkadot-JS Apps Front-end

Once the Swanky Node is running locally, you will be able to connect to it from the **Polkadot-JS Apps** front-end,
in order to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local Swanky Node.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
mkdir .local # this is mounted by container
./scripts/docker_run.sh
```

This command will compile the code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/swanky-node --dev --ws-external`)
by appending your own. A few useful commands are shown below:

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/swanky-node --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/swanky-node purge-chain

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## Consensus (Manual Seal & Instant Seal)
Unlike other blockchains, Swanky Node adopts block authoring and finality gadgets referred to as Manual Seal and Instant Seal, consensus mechanisms suitable for contract development and testing.

Manual seal - Blocks are authored whenever RPC is called.
Instant seal - Blocks are authored as soon as transactions enter the pool, most often resulting in one transaction per block.

Swanky Node enables both Manual seal and Instant seal.

### Manual Sealing via RPC call
We can tell the node to author a block by calling the `engine_createBlock` RPC.

```bash
$ curl http://127.0.0.1:9944 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_createBlock",
      "params": [true, false, null]
    }'
```

#### Params
- **Create Empty**
  `create_empty` is a Boolean value indicating whether empty blocks may be created. Setting `create-empty` to true does not mean that an empty block will necessarily be created. Rather, it means that the engine should go ahead creating a block even if no transactions are present. If transactions are present in the queue, they will be included regardless of the value of `create_empty`.

- **Finalize**
  `finalize` is a Boolean value indicating whether the block (and its ancestors, recursively) should be finalized after creation.

- **Parent Hash**
  `parent_hash` is an optional hash of a block to use as a parent. To set the parent, use the format `"0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764"`. To omit the parent, use `null`. When the parent is omitted the block will be built on the current best block. Manually specifying the parent is useful for constructing fork scenarios, and demonstrating chain reorganizations.

### Finalizing Blocks Manually
In addition to finalizing blocks at the time of creating them, they may also be finalized later by using the RPC call `engine_finalizeBlock`.

```bash
$ curl http://127.0.0.1:9944 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_finalizeBlock",
      "params": ["0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764", null]
    }'
```

## Consensus (Delayed Finalize)
By default, either manual or instant seal does not result in block finalization unless the `engine_finalizeBlock` RPC is executed. However, it is possible to configure the finalization of sealed blocks to occur after a certain amount of time by setting the `--finalize-delay-sec` option to a specific value, which specifies the number of seconds to delay before finalizing the blocks.

```bash
./target/release/swanky-node --finalize-delay-sec 5
```

In the above example, a setting of `5` seconds would result in the blocks being finalized five seconds after being sealed. In contrast, setting the value to `0` would lead to instant finalization, with the blocks being finalized immediately upon being sealed.

## Block height manipulation
Developers can forward blocks and revert blocks to requested block heights.

### Forward blocks via RPC
Forwarding blocks to requested block height by calling `engine_forwardBlocksTo`.

```bash
$ curl http://127.0.0.1:9944 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_forwardBlocksTo",
      "params": [120]  
    }'
```

#### Params
- **Height**
  `height` denotes an integral value that signifies the desired block height towards which the user intends to progress. If the value is lower than current height, RPC returns an error.

### Revert blocks via RPC
Reverting blocks to requested block height by calling `engine_revertBlocksTo`.

Note that reverting finalized blocks only works when node is launched with archive mode `--state-pruning archive` (or `--pruning archive`) since reverting blocks requires past blocks' states.
When blocks' states are pruned, **RPC won't revert finalized blocks**.

```bash
$ curl http://127.0.0.1:9944 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"engine_revertBlocksTo",
      "params": [50]
    }'
```

#### Params
- **Height**
  `height` denotes an integral value that represents the desired block height which the user intends to revert to. If the value is higher than current height, RPC returns an error.

## Account Balance manipulation
For local development purpose, developers can manipulate any users' account balance via RPC without requiring their accounts' signatures and transaction cost to pay.

### Get Account Balance
Getting users' account balance by `balance_getAccount` method.
```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"balance_getAccount",
      "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", null]
    }'
```

#### Params
- **Account ID**
  `account_id` is AccountID whose balance information you would like to check.

### Set Free Balance
Free balance is amount of unreserved token owner can freely spend. `balance_setFreeBalance` alters the amount of free token a specified account has.
```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"balance_setFreeBalance",
      "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 120000000000000000000, null]
    }'
```

#### Params
- **Account ID**
  `account_id` is `AccountID` whose balance you would like to modify.

- **Free Balance**
  `free_balance` is new Balance value you would like to set to accounts.
