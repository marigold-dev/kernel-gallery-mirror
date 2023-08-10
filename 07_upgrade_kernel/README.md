# Example: Upgrade Kernel

In this example we will demonstrate how to upgrade a kernel.

Let's upgrade this kernel to the debug kernel one.

## Running the example

To run the kernel locally, compile the kernel to WASM with Cargo:

```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel against the provided inputs and commands:

```sh
$ wasm-strip ../target/wasm32-unknown-unknown/release/debug_kernel.wasm -o stripped_debug_kernel.wasm
$ cargo run --manifest-path ./upgrade-client/Cargo.toml -- get-reveal-installer --kernel stripped_debug_kernel.wasm -P ./preimage
Root hash: 004B28109DF802CB1885AB29461BC1B410057A9F3A848D122AC7A742351A3A1F4E
$ octez-smart-rollup-wasm-debugger ../target/wasm32-unknown-unknown/release/upgrade_kernel.wasm --inputs ./inputs.json  <<< $(cat ./commands.txt)
Loaded 0 inputs at level 0
Hello from the upgrade kernel! I haven't upgraded yet.
Evaluation took 3357271 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Error: Not in a reveal step
%!> Hello from kernel!
Evaluation took 10996865685 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Evaluation took 10999777044 ticks so far
Status: Waiting for input
Internal_status: Collect
```
