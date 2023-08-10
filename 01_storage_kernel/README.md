# Example: Storage Kernel

In this example, we take a first look writing and reading to persistent storage.

Additionally, we introduce the Mock Host to enable a simple unit test of our kernel.

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
$ octez-smart-rollup-wasm-debugger ../target/wasm32-unknown-unknown/release/storage_kernel.wasm --inputs ./inputs.json <<< $(cat ./commands.txt)
68656c6c6f20776f726c64
Loaded 0 inputs at level 0
Evaluation took 235916 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
```

Additionally, you can omit the `<<< $(cat ./commands.txt)` to enter a REPL mode and
explore the execution of the kernel interactively.
