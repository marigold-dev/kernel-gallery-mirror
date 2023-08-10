// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    Sword,
    Potion,
}

#[allow(dead_code)]
impl Item {
    pub fn new_sword() -> Self {
        Self::Sword
    }

    pub fn new_potion() -> Self {
        Self::Potion
    }
}
