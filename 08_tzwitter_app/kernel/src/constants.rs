// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

pub const MAGIC_BYTE: u8 = 0x74;

#[cfg(not(debug_assertions))]
pub const L1_TOKEN_CONTRACT_ADDRESS: &str = env!("TZWITTER_L1_CONTRACT");
#[cfg(debug_assertions)]
pub const L1_TOKEN_CONTRACT_ADDRESS: &str = "KT1RycYvM4EVs6BAXWEsGXaAaRqiMP53KT4w";

pub const L1_TOKEN_CONTRACT_ENTRYPOINT: &str = "mint";
