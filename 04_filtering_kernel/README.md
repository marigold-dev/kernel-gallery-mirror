# Example: Message Filtering Kernel

## Running the example

Run the unit test with `cargo test`:

<!-- $MDX skip -->

```sh
$ cargo test
```

To run the kernel locally, compile the kernel to WASM with Cargo:

<!-- $MDX skip -->

```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel against the provided inputs (empty in this example) and commands:

```sh
$ octez-smart-rollup-wasm-debugger ../target/wasm32-unknown-unknown/release/filtering_kernel.wasm --inputs ./inputs.json <<< $(cat ./commands.txt)
Loaded 2 inputs at level 0
External message: "This message is for me"
Evaluation took 417738 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
```

Additionally, you can omit the `<<< $(cat ./commands.txt)` to enter a REPL mode and
explore the execution of the kernel interactively.
