// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use core::panic;

use tezos_smart_rollup_host::{path::RefPath, runtime::Runtime};

const COUNTER_PATH: RefPath = RefPath::assert_from(b"/counter");

fn read_counter<R: Runtime>(rt: &mut R) -> u64 {
    match rt.store_read(&COUNTER_PATH, 0, 8) {
        Ok(bytes) => u64::from_be_bytes(bytes.try_into().unwrap()),
        Err(_) => 0,
    }
}

fn write_counter<R: Runtime>(rt: &mut R, counter: u64) -> u64 {
    match rt.store_write(&COUNTER_PATH, &counter.to_be_bytes(), 0) {
        Ok(_) => counter,
        Err(_) => panic!(),
    }
}

pub fn entry<R: Runtime>(rt: &mut R) {
    while let Ok(Some(message)) = rt.read_input() {
        if let [0x01, 0x88, ..] = message.as_ref() {
            let counter = read_counter(rt);
            let next_counter = counter + 1;
            write_counter(rt, next_counter);
        }
    }
}
