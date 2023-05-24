mod low_latency;
mod native_batcher;
mod native_runtime;
mod rollup_batcher_injector;
mod sled_database;
mod tezos_listener;

pub use low_latency::*;
pub use native_batcher::*;
pub use rollup_batcher_injector::*;
pub use sled_database::*;
pub use tezos_listener::*;

// TODO:
// implement host (but do not expose it, it will be part of the low latency module)

// Implement a prelude that implement give a way to instanciante the node
// Create three new crates:
// - this one
// - one dedicated fot the dummy counter which has to expose only the kernel_entry function
// - one dedicated for the http server
