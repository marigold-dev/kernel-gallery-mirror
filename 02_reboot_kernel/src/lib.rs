// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use tezos_smart_rollup::{kernel_entry, prelude::*, storage::path::OwnedPath};

/// In this example, we'll begin to explore the control
/// flow between the kernel and the runtime.
///
/// Consider the debug kernel from the earlier example.
/// When called, it prints a message and immedaitely exits,
/// yielding to the underlying runtime, which loads inputs.
/// Any inputs that were not read by the kernel before exiting
/// are discarded, and in the case of the debug kernel, that's
/// includes all of them.
///
/// In this kernel, we will mark it for reboot for exiting. When
/// marked for reboot, the runtime will immediately reboot the kernel
/// after exit, preserving the inputs that were not read.
///
/// This is useful for kernels that cannot process all inputs in a single
/// call to the entry function. Remember that the runtime allows
/// a limited number of ticks per call to the entry function (and up
/// to 1000 reboots per Tezos block).
/// Be sure to read more at the official docs: https://tezos.gitlab.io/mumbai/smart_rollups.html#control-flow,
///
/// tl;dr - don't cram too more ticks than allowed in
/// a single call to the entry function; instead, mark
/// it for reboot, exit, and process the remaining inputs
/// in the next call.
pub fn entry(host: &mut impl Runtime) {
    debug_msg!(host, "Hello from kernel!\n");
    let greeting_path: OwnedPath = "/greeting".as_bytes().to_vec().try_into().unwrap();
    let _ = Runtime::store_write(host, &greeting_path, "hello world".as_bytes(), 0);
    host.mark_for_reboot().unwrap();

    // Understanding control flow for rollup kernels is critical,
    // but genuinely tricky. To reinforce the point, try running
    // the kernel with the debugger (see README in this folder for how).
    // Give the following inputs
    // > load inputs
    // > step result
    // > step result
    // > step result
    // ...
    // You can keep calling `step result` and seeing "Hello from kernel"
    // until you run out of reboots (a thousand iterations!).
    //
    // Now observe what happens when you comment out the `host.mark_for_reboot()`
    // and repeat the experiment.
    //
    // (Go on, try it!)
    //
    // You'll see that after the after the first step, the inbox messages are immediately dropped
    // (though, in our case, there are none), and the runtime is in a state of waiting for more
    // inputs (from new Tezos blocks). Continued calls to `step result` push the kernel into a
    // situation it could never reach in at runtime, and the debugger eventually complains about this.
    //
    // As a kernel developer, it's up to you to balance squeezing more throughput into each call
    // to the kernel entrypoint with marking the kernel for reboot and exiting cleanly before you
    // run out of ticks. Failure to do so in the correct sequence could drop messages or cause
    // you to run out of ticks. The rollup node won't notice if you run out of ticks. But if you
    // make a commitment to a state that violates the tick limit, you could get slashed!
}

kernel_entry!(entry);
