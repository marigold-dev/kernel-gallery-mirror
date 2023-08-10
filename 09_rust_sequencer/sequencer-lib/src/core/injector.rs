// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use async_trait::async_trait;

#[async_trait]
pub trait Injector {
    async fn inject(&self, payload: Vec<Vec<u8>>) -> Result<(), ()>;
}
