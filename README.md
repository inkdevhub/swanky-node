# Swanky Node

Swanky node is the Substrate based blockchain configured to enable `pallet-contracts` and more features to help Wasm Smart Contract development.

## Getting Started

Follow the steps below to get started with the swanky node :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the swanky node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/swanky-node -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/swanky-node --dev
```

Purge the development chain's state:

```bash
./target/release/swanky-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/swanky-node -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-swanky-node/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/swanky-node --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Show only Errors and Contract Debug Output
To have only errors and contract debug output show up on the console you can supply `-lerror,runtime::contracts=debug` when starting the node.

Important: Debug output is only printed for RPC calls or off-chain tests ‒ not for transactions!

See our FAQ for more details: How do I print something to the console from the runtime?.

### Connect with Polkadot-JS Apps Front-end

Once the swanky node is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local swanky.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/swanky-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/swanky-node --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/swanky-node purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## Consensus (Manual Seal & Instant Seal)
Unlike other blockchains, Swanky node adopts block authioring and finalized gadget called Manual (& Instant) Seal, consensus which is suitable for contract development and testing.

Manual seal: Where there is one author and it authors a block whenever you tell it via an RPC call.
Instant seal: Where there is one author and it attempts to author a block as soon as it sees a transaction in the pool, most often leading to one transaction per block

By default, instant seal is used.

### Using Instant Seal
```bash
./target/release/swanky-node --dev
```

As soon as transaction gets pooled, blocks are instantly created.


### Using Manual Seal
```bash
./target/release/swanky-node --dev --manual-seal
```

Once your node is running, you will see that it just sits there idly. It will accept transactions to the pool, but it will not author blocks on its own. In manual seal, the node does not author a block until we explicitly tell it to. We can tell it to author a block by calling the `engine_createBlock` RPC.

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
`finalize` is a Boolean indicating whether the block (and its ancestors, recursively) should be finalized after creation. Manually controlling finality is interesting, but also dangerous. If you attempt to author and finalize a block that does not build on the best finalized chain, the block will not be imported. If you finalize one block in one node, and a conflicting block in another node, you will cause a safety violation when the nodes synchronize.

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
