## How to build and run Swanky-node to support pallet_uniques


1. Clone this repo

```
git clone git@github.com:AstarNetwork/swanky-node.git
```
2. checkout working branch
```
git checkout feature/uniques-ce
```
3. build the node
````
cargo build --release
````
4. run the node
````
./target/release/swanky-node --dev --tmp -lerror,runtime::contracts=debug 
````

