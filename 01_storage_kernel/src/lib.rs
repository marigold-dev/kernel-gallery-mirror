// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2023 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

extern crate alloc;

use tezos_smart_rollup::{kernel_entry, prelude::*, storage::path::OwnedPath};

// In this kernel, we'll demonstrate how to write from storage.
// Additionally, we'll write a unit test that executes against the Mock Host
// test fixture.

/// In this kernel, we'll just store a single value in persistent storage.
pub fn entry(host: &mut impl Runtime) {
    // Kernel's can specify paths to store data in. We'll use the path "/greeting"
    let greeting_path: OwnedPath = "/greeting".as_bytes().to_vec().try_into().unwrap();
    let _ = Runtime::store_write(host, &greeting_path, "hello world".as_bytes(), 0);
}

kernel_entry!(entry);

// Let's test our kernel!
// With the Mock Host, we can execute the kernel natively (i.e without WASM)
// to simulate interactions and assert on the results.
#[cfg(test)]
mod test {

    #[test]
    fn test_storage() {
        use super::*;

        let mut host = tezos_smart_rollup::testing::prelude::MockHost::default();

        let greeting_path: OwnedPath = "/greeting".as_bytes().to_vec().try_into().unwrap();
        host.run_level(entry);

        let expected_message = "hello world";

        let greeting = Runtime::store_read(&mut host, &greeting_path, 0, expected_message.len())
            .map_err(|_| "Error reading from storage".to_string())
            .unwrap_or_default();

        assert_eq!(greeting.as_slice(), expected_message.as_bytes());
    }
}
