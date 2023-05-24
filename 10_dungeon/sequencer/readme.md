# Sequencer / Low latency

This first design of the sequencer and low latency node has been designed to be kernel agnostic, that means this sequencer can work for any kernels.

Under this folder you will find 3 different folders:
 - sequencer-lib: the library of the sequencer
 - sequencer-http: an http server that uses the sequencer-lib and exposes endpoint to submit operation and retrieve the optimist state
 - counter-kernel: the sequencer-http is running a simple kernel that counts the number of external messages in the inbox.

 # Sequencer-lib

The sequencer is also a low latency node. That means it will compute an optimist state for every received operation and it will expose an optimist state.

What is not implemented in this first version:
 - a delayed inbox: people can still directly post operations to the inbox and bypass the sequencer, if so, the state of the rollup and the sequencer will diverge
 - using the DAC: the operations sent to the sequencer has to be smaller than 4kb, by using the DAC we can ignore this limitation
 - a solution to get the state from the rollup: it would fix any state divergence issue.

 What have been implemented:
 - a native runtime: to execute natively the kernel
 - a batcher: to batch the operations in a sequence
 - a tezos listener: to listen for tezos blocks
 - a tezos injector: to inject the operation to the rollup

# Sequencer http

Because this PoC is not (yet) a framework, you should write your own sequencer-http binary to run your kernel with low latency.
Let me explain how to modify the `sequencer-http` crate to run your own kernel.

First you should add your kernel as a dependency in the `Cargo.toml` by replacing the line `counter-kernel = {path = "../counter-kernel"}` by yours.

Then you should call the entry function of your kernel in the `main.rs`:

```rust
struct MyKernel {}

impl Kernel for MyKernel {
    fn entry<R: Runtime>(host: &mut R) {
        counter_kernel::entry(host); // Change this line to call: my_kernel::entry(host)
    }
}
```

Finnaly, you have to update these 2 variables:

```rust
let tezos_node_uri = "http://localhost:18731";
let rollup_node_uri = "http://localhost:8932";
```

Unfortunately the sequencer needs a `tezos_node_uri` pointing to a working node, otherwise it will crash...

> If you only want to start the sequencer for debug purpose, you can put any node from any network
> And when you want to deploy your sequencer, use a node on the same network as your rollup

Then you are ready to execute the sequencer:

```bash
cd sequencer-http
cargo run
```