// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use tezos_smart_rollup_host::runtime::Runtime;

pub trait Kernel {
    fn entry<Host: Runtime>(host: &mut Host);
}
