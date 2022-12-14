/*****************************************************************************/
/*                                                                           */
/* Open Source License                                                       */
/* Copyright (c) 2022 Nomadic Labs <contact@nomadic-labs.com>                */
/*                                                                           */
/* Permission is hereby granted, free of charge, to any person obtaining a   */
/* copy of this software and associated documentation files (the "Software"),*/
/* to deal in the Software without restriction, including without limitation */
/* the rights to use, copy, modify, merge, publish, distribute, sublicense,  */
/* and/or sell copies of the Software, and to permit persons to whom the     */
/* Software is furnished to do so, subject to the following conditions:      */
/*                                                                           */
/* The above copyright notice and this permission notice shall be included   */
/* in all copies or substantial portions of the Software.                    */
/*                                                                           */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR*/
/* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,  */
/* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL   */
/* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER*/
/* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING   */
/* FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER       */
/* DEALINGS IN THE SOFTWARE.                                                 */
/*                                                                           */
/*****************************************************************************/
extern crate kernel;
extern crate alloc;
extern crate debug;
extern crate kernel_core;
extern crate anyhow;

use anyhow::{ ensure, Result };
use host::{
    input::{ Input, MessageData },
    path::OwnedPath,
    rollup_core::{ RawRollupCore, MAX_INPUT_MESSAGE_SIZE, MAX_INPUT_SLOT_DATA_CHUNK_SIZE },
    runtime::Runtime,
};
use kernel_core::{ inbox::{ InboxMessage, InternalInboxMessage, Transfer }, memory::Memory };
use debug::debug_msg;

const MAX_READ_INPUT_SIZE: usize = if MAX_INPUT_MESSAGE_SIZE > MAX_INPUT_SLOT_DATA_CHUNK_SIZE {
    MAX_INPUT_MESSAGE_SIZE
} else {
    MAX_INPUT_SLOT_DATA_CHUNK_SIZE
};

/// Counter
#[derive(Debug, Clone, Copy)]
pub struct Counter {
    val: i8,
}

impl Counter {
    /// Create a new counter
    pub fn new(val: i8) -> Self {
        Self { val }
    }

    /// Public read-only method: Returns the counter value
    pub fn get_num(&self) -> i8 {
        self.val
    }

    /// Increment the counter
    pub fn increment(&mut self) -> u64 {
        self.val += 1;
        return self.val as u64;
    }

    pub fn decrement(&mut self) {
        self.val -= 1;
    }

    /// Reset the counter to 0
    pub fn reset(&mut self) {
        self.val = 0;
    }

    // convert from i8 to u64 use in string ticket
    pub fn convert(&mut self) -> u64 {
        self.val as u64
    }
}

fn process_counter<'a, Host: RawRollupCore>(
    host: &mut Host,
    memory: &mut Memory,
    counter: &mut Counter,
    payload: &'a [u8]
) {
    // parsing the Input message with the payload
    let (_remaining, message) = InboxMessage::parse(payload).expect("Failed on parse payload");

    // Input message is either external or internal message
    match message {
        // External message, it is hex
        InboxMessage::External(message) => {
            debug_msg!(Host, "Received an external message {:?}", message);

            let incr_counter = counter.increment();
            debug_msg!(Host, "Counter {:#?}", incr_counter);
        }

        // Internal message via contract call
        InboxMessage::Internal(message) =>
            match message {
                InternalInboxMessage::Transfer(Transfer { payload, .. }) => {
                    debug_msg!(Host, "Received an internal message {:?}", payload);

                    counter.increment();
                    debug_msg!(Host, "Counter {:#?}", counter);
                }
                InternalInboxMessage::StartOfLevel => {
                    debug_msg!(Host, "Start of level");
                }
                InternalInboxMessage::EndOfLevel => {
                    debug_msg!(Host, "End of level");
                }
            }
    }

    // call memory snapshot
    memory.snapshot(host);
}

pub fn handle_input_message<H: RawRollupCore>(
    host: &mut H,
    memory: &mut Memory,
    message: MessageData
) -> Result<()> {
    // processing counter function
    let path: OwnedPath = "/counter"
        .as_bytes()
        .to_vec()
        .try_into()
        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
    let counter = match Runtime::store_read(host, &path, 0, 1) {
        Ok(counter) => counter,
        Err(_) => vec![0],
    };
    ensure!(counter.len() == 1, "counter is not a byte");
    let mut counter = Counter {
        val: i8::from_le_bytes(counter[..].try_into().unwrap()),
    };

    process_counter(host, memory, &mut counter, message.as_ref());
    // We need to persist the effect in `counter`
    Runtime::store_write(host, &path, &counter.val.to_le_bytes(), 0).map_err(|e|
        anyhow::Error::msg(format!("{e:?}"))
    )?;
    Ok(())
}

pub fn counter_run<Host: RawRollupCore>(host: &mut Host) {
    let mut memory = Memory::load_memory(host);

    // Reading the input from host
    match host.read_input(MAX_READ_INPUT_SIZE) {
        Ok(Some(Input::Message(message))) => {
            debug_msg!(Host, "message data at level:{} - id:{}", message.level, message.id);
            if let Err(_) = handle_input_message(host, &mut memory, message) {
                // log and gracefully exit
            }
        }
        Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
        Ok(None) => debug_msg!(Host, "no input"),
        Err(_) => todo!("handle errors"),
    }

    // Memory is like a database, make a snapshot
    memory.snapshot(host)
}

/// Define the `kernel-run` for the counter kernel.
#[cfg(feature = "counter-kernel")]
pub mod counter_kernel {
    use kernel::kernel_entry;
    kernel_entry!(counter_run);
}