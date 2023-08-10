# Example: Inbox Kernel

## Running the example

To run the kernel locally, compile the kernel to WASM with Cargo:

<!-- $MDX skip -->

```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel against the provided inputs and commands:

```sh
$ octez-smart-rollup-wasm-debugger ../target/wasm32-unknown-unknown/release/inbox_kernel.wasm --inputs ./inputs.json <<< $(cat ./commands.txt)
Loaded 2 inputs at level 0
Inbox level: 0 Internal message: start of level
Inbox level: 0 Internal message: level info (block predecessor: BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M, predecessor_timestamp: 1970-01-01T00:00:00Z
Inbox level: 0 External message: "This is an external message"
Inbox level: 0 External message: "And here's another one"
Inbox level: 0 Internal message: end of level
Evaluation took 1093784 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
```

Additionally, you can omit the `<<< $(cat ./commands.txt)` to enter a REPL mode and
explore the execution of the kernel interactively.
