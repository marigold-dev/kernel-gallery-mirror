// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2023 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

extern crate alloc;

use tezos_smart_rollup::{kernel_entry, prelude::*, storage::path::OwnedPath};

mod counter;
use counter::*;

fn execute<Host: Runtime>(host: &mut Host, counter: Counter) -> Counter {
    // Read the input
    let input = host.read_input();

    match input {
        // If it's an error or no message then does nothing}
        Err(_) | Ok(None) => counter,
        Ok(Some(message)) => {
            // If there is a message let's process it.
            debug_msg!(host, "Hello message\n");
            let data = message.as_ref();
            match data {
                [0x00, ..] => {
                    debug_msg!(host, "Message from the kernel.\n");
                    execute(host, counter)
                }
                [0x01, ..] => {
                    debug_msg!(host, "Message from the user.\n");
                    // Let's skip the first byte of the data to get what the user has sent.
                    let user_message: Vec<&u8> = data.iter().skip(1).collect();
                    // We are parsing the message from the user.
                    // In the case of a good encoding we can process it.
                    let user_message = UserAction::try_from(user_message);
                    match user_message {
                        Ok(user_message) => {
                            let counter = transition(counter, user_message);
                            execute(host, counter)
                        }
                        Err(_) => execute(host, counter),
                    }
                }
                _ => execute(host, counter),
            }
        }
    }
}

pub fn entry(host: &mut impl Runtime) {
    let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
    let counter = Runtime::store_read(host, &counter_path, 0, 8)
        .map_err(|_| "Runtime error".to_string())
        .and_then(Counter::try_from)
        .unwrap_or_default();

    let counter = execute(host, counter);

    let counter: [u8; 8] = counter.into();
    let _ = Runtime::store_write(host, &counter_path, &counter, 0);
}

kernel_entry!(entry);

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown --features greeter
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/coutner_kernel.wasm --inputs ./counter_kernel/inputs.json
// 'load inputs'
// 'step result'
// 'show key /counter'
#[cfg(test)]
mod test {
    use tezos_smart_rollup::{host::Runtime, storage::path::OwnedPath};

    use crate::{
        counter::{Counter, UserAction},
        entry,
    };

    #[test]
    fn test_counter() {
        let mut host = tezos_smart_rollup::testing::prelude::MockHost::default();

        let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
        host.run_level(entry);

        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();

        assert_eq!(counter, Counter { counter: 0 });

        let action = UserAction::Increment;
        host.add_external(action);
        host.run_level(entry);
        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();
        assert_eq!(counter, Counter { counter: 1 });
    }
}
