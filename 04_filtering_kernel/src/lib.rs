// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2023 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

use tezos_smart_rollup::{
    inbox::InboxMessage,
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

// The rollups inbox contains all messages addressed to all rollups.
// This kernel shows how to filter external messages in order to only handle
// those addressed to this rollup. For an example covering internal messages,
// see `outbox-kernel`.

// Unlike internal messages, external messages have no structure imposed on them.
// It is therefore the responsibility of each kernel to define their format and
// a way to identify which messages are addressed to it.
// Here, we will use a `MAGIC_BYTE`, which we will require as a prefix of any
// external message addressed to this rollup.
pub const MAGIC_BYTE: u8 = 0x1a;
// Another good strategy is to include the rollup address itself in the message,
// read the rollup address in the kernel via `host.read_metadata()`, and filter
// for just those messages that include the correct address.

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    loop {
        match host.read_input() {
            Ok(Some(message)) => {
                // Parse the payload of the message
                match InboxMessage::<Expr>::parse(message.as_ref()) {
                    Ok((remaining, InboxMessage::External([MAGIC_BYTE, data @ ..]))) => {
                        // Only process external messages that begin with the magic byte
                        // that we have defined for this rollup.
                        debug_assert!(remaining.is_empty());
                        let message = String::from_utf8_lossy(data);
                        debug_msg!(host, "External message: \"{}\"\n", message);
                    }
                    Ok(_) => {
                        // Ignore any other message
                        continue;
                    }
                    Err(_) =>
                    // Error parsing the message. This could happen when parsing a message
                    // sent to a different rollup, which might have a different Michelson type.
                    {
                        continue
                    }
                }
            }
            Ok(None) =>
            // Exit loop when there are no more messages to read.
            {
                break
            }
            Err(_) =>
            // An error here would most likely indicate a malformed message, ignore.
            {
                continue
            }
        }
    }
}

pub fn entry(host: &mut impl Runtime) {
    read_inbox_message::<MichelsonUnit>(host);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);
