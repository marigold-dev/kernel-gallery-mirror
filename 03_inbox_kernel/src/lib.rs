// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2023 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

use tezos_smart_rollup::{
    inbox::{InboxMessage, InternalInboxMessage},
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

// This kernel demonstrates how to parse the different kinds of inbox messages.
// The rollups inbox is the mechanism by which the Layer 1 can send messages to
// rollups. These can be:
// - Internal messages: sent by the Layer 1 itself, either:
//   - as a result of a smart contract making a transfer to a rollup address, or
//   - status messages from the protocol (such as the start or end of a new block level)
// - External messages: sent by anyone via a new kind of Layer 1 operation. These can
// include any kind of data, as defined by the kernel.

// The `read_inbox_message` function shows how to parse all of these messages,
// but more detailed handling of internal transfer messages and external
// messages is shown in the `filtering-kernel` and `outbox-kernel` examples.
// It is also important to keep in mind that the rollups inbox is shared among
// all deployed rollups, meaning that we should check whether a message is
// intended for us before processing it. For simplicity, this kernel skips over
// these checks, but they are also demonstrated in the aforementioned examples.

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    // In this example, we are using an infinite loop, breaking only when
    // we reach the end of the inbox. In practice, we would need to be
    // mindful of how many ticks the processing of each kind of message takes.
    // We must be careful to benchmark the execution of our kernel and set a conservative
    // limit on how many messages the kernel can process before marking it for
    // reboot. See [https://tezos.gitlab.io/alpha/smart_rollups.html#developing-wasm-kernels]
    // for more details.
    loop {
        match host.read_input() {
            Ok(Some(message)) => {
                // Show the inbox level of the message
                debug_msg!(host, "Inbox level: {} ", message.level.to_string());
                // Parse the payload of the message
                match InboxMessage::<Expr>::parse(message.as_ref()) {
                    Ok(parsed_msg) => match parsed_msg {
                        (remaining, InboxMessage::Internal(msg)) => {
                            debug_assert!(remaining.is_empty());
                            match msg {
                                InternalInboxMessage::StartOfLevel => {
                                    // The "Start of level" message is pushed by the Layer 1
                                    // at the beginning of each level. It carries no additional
                                    // payload. The actual level number is recorded as metadata
                                    // for every single message, as shown above.
                                    debug_msg!(host, "Internal message: start of level\n")
                                }
                                InternalInboxMessage::InfoPerLevel(info) => {
                                    // The "Info per level" messages follows the "Start of level"
                                    // message and contains information on the previous Layer 1 block.
                                    debug_msg!(
                                        host,
                                        "Internal message: level info \
                                             (block predecessor: {}, predecessor_timestamp: {}\n",
                                        info.predecessor,
                                        info.predecessor_timestamp
                                    );
                                }
                                InternalInboxMessage::EndOfLevel => {
                                    // The "End of level" message is pushed by the Layer 1
                                    // at the end of each level.
                                    debug_msg!(host, "Internal message: end of level\n")
                                }
                                InternalInboxMessage::Transfer(_) => {
                                    // See `outbox-kernel` for a more detailed explanation
                                    // of transfer messages and a simple example of how
                                    // to handle them.
                                    debug_msg!(host, "Internal message: transfer\n")
                                }
                            }
                        }
                        (remaining, InboxMessage::External(msg)) => {
                            // External messages can encode any kind of data.
                            // Defining their format and parsing them is up to each kernel.
                            // For a simple practical example, see `counter-kernel`, where
                            // external messages are used to encode the state transitions
                            // of a counter.
                            debug_assert!(remaining.is_empty());
                            let message = String::from_utf8_lossy(msg);
                            debug_msg!(host, "External message: \"{}\"\n", message);
                        }
                    },
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
            // An error here would most likely indicate a violation of the protocol between
            // the Layer 1 and the rollup node or kernel, e.g. as a result of a protocol
            // upgrade that was not handled by the kernel appropriately.
            {
                continue
            }
        }
    }
}

pub fn entry(host: &mut impl Runtime) {
    // Every rollup has a Michelson type, declared at origination, which
    // represents the kind of data it can receive via internal transfer
    // messages. We won't delve into this here - for simplicity, the type
    // of this rollup is unit.
    read_inbox_message::<MichelsonUnit>(host);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);
