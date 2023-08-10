# Dungeon game POC

This is a rollup DApp for an interactive game. This "kernel" is a program
using the rollup kernel-SDK, sequencer prototype (low-latency node) and use the React App as its front-end.

The project is divided into three sub projects:

- The kernel, that is ran by the rollup
- The front, that can be uploaded anywhere
- The integration with the sequencer prototype

## The Dungeon-kernel

For the features we have:

- Light multi-players (2 players) interact with each other in real-time.
- Light connection with wallet (hardcode the secret keys).
- Exchange assets: where the player can drop, pick up, sell and buy item.

![](../10_dungeon_app/pics/sequencer%20workflow.png)

We have 5 main components:

- The DApp
- The Dungeon-kernel
- The Layer 1 - Tezos node
- The sequencer prototype
- The Rollup node

The Dungeon-kernel is known by both the Sequencer and the Rollup node.
The DApp will send a sequence of operations to the sequencer. The sequencer will then use it own runtime mechanism to compute the optimist durable state. The DApp will then fetch that state from the sequencer.

The sequencer will collect the sequence of operations in a batch, hash it and then send this hash to the Layer 1 (Tezos node).

Tezos node will receive this hash and send it to the rollup node. Rollup node will reconstruct the hash, process and verify the sequence of operations of this hash. If everything went well it will commit this hash.

Thanks to the sequencer, each action of the player is fast, the Dapp can fetch the new state without having to wait for the operations to be committed in the rollup node. We don't have to wait for 2 blocks for the rollup node to process and verify.

## Front-end

Display all the actors: player, map, floor, wall, item, inventory, gold and marketplace.

Add actions of the player: move, drop, pick up, buy, sell and switch to different player account.

Whenever player picks up the item, this item will be disappeared from the map and display in his inventory.

Whenever the player drops an item, this item will be disappeared from his inventory and appear on the map at the position that he dropped.

The player able to choose which item to drop that is available in his inventory.

Each player has certain amount of gold, this will be used for buying and selling the item.

Whenever the player choose to sell the item in his inventory, the item will be disappeared from his inventory and show in the marketplace with the option to sell.

When another player wants to buy the item that is listed in the marketplace. When he bought it, the item will be disappeared in the marketplace, display in his inventory and the amount of gold will be decreased by the value that the item is selled. The gold of the seller will increased by the amount that this item sells as well.

For the front-end, there is an interval that fetches the state of the sequencer every 0.5s. We have to fetch the state in an interval because the sequencer/node does not implement a push strategy (sending the state to the client when there is a new one).

The deserilisation of messges/state from the DApp is complicated, because the data are encoded in a binary format.

## Integration with the sequencer prototype

To use the sequencer you have to edit the sequencer-http/Cargo.toml, and add a dependency to your kernel:

```toml
# sequencer-http/Cargo.toml
kernel = {path = "../../10_dungeon/kernel"}
```

Then you can compile the sequencer-http crate and resolve any issues:

```rust
// sequencer-http/src/main.rs
impl Kernel for MyKernel {
    fn entry<R: Runtime>(host: &mut R) {
        kernel::entry(host)
    }
}
```

> You need to have a tezos node running.
> You can provide any node

```rust
let tezos_node_uri = "http://localhost:18731"; // Update this URI
```

Then you can start the sequencer:

```bash
cd sequencer/sequencer-http
cargo run
```

To submit an operation to the sequencer, you can use curl:
(Only external operation can added)

The content type of the request is a json.
The sequencer is exposing an post http endpoint "/operations" to submit an operation with the following payload:

```json
{ "data": "01" }
```

Where "01" refers to your hexadecimal operation.

```bash
curl -H "Content-Type: application/json" -X POST -d '{"data": "01" }' http://localhost:8080/operations
> Operation submitted
```

And the you can retrieve your state.
The sequencer is exposing two get endpoints to retrieve your optimist state:

- one to retrieve the value "/state/value?path=..."
- one to retrieve the list of sub keys of a path "/state/subkeys?path=..."

```bash
curl "http://127.0.0.1:8080/state/value?path=/state/player/y_pos"
> 0000000000000007
curl "http://127.0.0.1:8080/state/subkeys?path=/state/player"
> ["x_pos", "y_pos"]
```

## Dungeon POC on Mainnet

We have deployed our game on Mainnet in an [observer mode](https://tezos.gitlab.io/alpha/smart_rollups.html):

https://mainnet.dungeon.marigold.dev
