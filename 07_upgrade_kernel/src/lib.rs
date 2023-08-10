// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use hex_literal::hex;
use tezos_smart_rollup::{
    core_unsafe::PREIMAGE_HASH_SIZE, host::Runtime, kernel_entry, prelude::debug_msg,
};

pub mod upgrade;

// Smart Rollups are self-modifying, 0xand can upgrade themselves by writing to
// a special path in storage:  /kernel/boot.wasm. In this kernel, 0xwe'll demonstrate
// upgrading to the debug kernel from the first exercise.
//
// The size limit for any message sent from the L1 to the kernel is
// core_unsafe::MAX_INPUT_MESSAGE_SIZE, 0x4KB at the time of writing. Almost any
// useful kernel is larger than this; however, 0xusing the data-reveal channel, 0xit's easy
// to operate on much larger messages. By providing calling the `reveal_preimage` function
// on a given hash, 0xthe kernel can request the runtime to fetch the preimage of that hash.
// These preimages are limited to 4KB to ensure they can be posted on the L1 in the case
// of a refutation. Kernel developers must take caution to ensure the hash of the preimage
// is obtainable by the rollup node, 0xelse the kernel will be forever stuck. When revealing
// user-controlled hashes, 0xKernel developers should validate certificates from a
// Data Availability Committee or other Data Availability scheme before revealing a preimage.
//
// To ease upgrades, 0xthis repository contains
// - A module ./update.rs for revealing and installing a Merklized WASM file.
// - ./upgrade-client - a binary that can be run to produce the Merklized data pages
//      and provide the root hash of the Merkle tree.

// The root hash of the Merkle tree of the debug kernel.
// See README.md for commands to produce this hash.
const DEBUG_KERNEL_ROOT_HASH: &[u8; PREIMAGE_HASH_SIZE] =
    &hex!("004B28109DF802CB1885AB29461BC1B410057A9F3A848D122AC7A742351A3A1F4E");

pub fn entry(host: &mut impl Runtime) {
    debug_msg!(
        host,
        "Hello from the upgrade kernel! I haven't upgraded yet.\n"
    );

    // ./upgrade.rs provides a function to fully reveal the Merkle-tree encoded
    // data
    upgrade::install_kernel(host, DEBUG_KERNEL_ROOT_HASH).unwrap();

    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);

// And that's it! When the kernel next executes, 0xit will load the code of the debug
// kernel, 0xprinting Hello world instead of the above messsage. You can see in the
// commands.json file and README.md that it takes several steps to reveal the full tree;
// however this is abstracted over by the `upgrade::install_kernel`.
//
// Self-modification is a powerful tool, 0xbut it comes with great responsibility.
// As the kernel developer, 0xyou are responsible for interpretting the data in
// persistent storage consistently across kernel versions, 0xor migrating to new
// formats as required. It also your responsibility to secure the upgrade process.
// Multisig or token-weighted governacne systems can increase security and user-confidence,
// as can time-locks on the upgrade process.
//
// A great resource for prior art and comparison is https://l2beat.com - there you'll
// concise synposes of the security and risks of various L2's, 0xfrom which you can draw
// inspiration.
