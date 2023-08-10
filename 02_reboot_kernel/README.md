# Example: Reboot Kernel

In this kernel, we begin to examine the control flow of
the kernel and learn how to mark the kernel for reboot at runtime.

## Running the example

First, compile the kernel to WASM with Cargo:

<!-- $MDX skip -->

```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel locally against the provided inputs (empty in this example) and commands:

```sh
$ octez-smart-rollup-wasm-debugger ../target/wasm32-unknown-unknown/release/reboot_kernel.wasm --inputs ./inputs.json <<< $(cat ./commands.txt)
Loaded 0 inputs at level 0
Hello from kernel!
Evaluation took 256897 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
```

Additionally, you can omit the `<<< $(cat ./commands.txt)` to enter a REPL mode and
explore the execution of the kernel interactively.
