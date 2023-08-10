// SPDX-FileCopyrightText: 2022 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2022 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

extern crate alloc;

use tezos_smart_rollup::{kernel_entry, prelude::*};

/// The main entrypoint of the kernel.
/// This function is called by the runtime in a loop, and is
/// responsible for processing inputs (e.g. reading messages from the shared inbox
/// or revealing preimages of hashes) and writing to persistent storage and the shared
/// outbox.
///
/// Special care must be taken to ensure that the kernel does not run out of ticks
/// and that inputs are handled appropriately. We'll cover some of these topics
/// in coming examples, but we suggest having a look at the documentation as well:
/// https://tezos.gitlab.io/mumbai/smart_rollups.html#developing-wasm-kernels
pub fn entry(host: &mut impl Runtime) {
    // The `debug_msg!` macro prints messages that can be observed
    // when executing with the octez-smart-rollup-wasm-debugger binary.
    debug_msg!(host, "Hello from kernel!\n");
}

// Registers our `entry` function as the `kernel_run` function in the WASM output.
kernel_entry!(entry);
