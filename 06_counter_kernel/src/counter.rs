// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use tezos_data_encoding::enc::BinWriter;

#[derive(Debug, Default, PartialEq)]
pub struct Counter {
    pub(crate) counter: i64,
}

impl Counter {
    fn increment(self) -> Counter {
        Counter {
            counter: self.counter + 1,
        }
    }

    fn decrement(self) -> Counter {
        Counter {
            counter: self.counter - 1,
        }
    }
}

impl TryFrom<Vec<u8>> for Counter {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map_err(|_| "i64 is represented by 8 bytes".to_string())
            .map(i64::from_be_bytes)
            .map(|counter| Counter { counter })
    }
}

impl From<Counter> for [u8; 8] {
    fn from(value: Counter) -> Self {
        value.counter.to_be_bytes()
    }
}

#[derive(Debug, PartialEq, Eq, BinWriter)]
pub enum UserAction {
    Increment,
    Decrement,
    Reset,
}

pub fn transition(counter: Counter, action: UserAction) -> Counter {
    match action {
        UserAction::Increment => counter.increment(),
        UserAction::Decrement => counter.decrement(),
        UserAction::Reset => Counter::default(),
    }
}

impl TryFrom<Vec<&u8>> for UserAction {
    type Error = String;

    fn try_from(value: Vec<&u8>) -> Result<Self, Self::Error> {
        match value.as_slice() {
            [0x00] => Ok(UserAction::Increment),
            [0x01] => Ok(UserAction::Decrement),
            [0x02] => Ok(UserAction::Reset),
            _ => Err("Deserialization is not respected".to_string()),
        }
    }
}
