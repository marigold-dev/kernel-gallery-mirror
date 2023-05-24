use super::native_runtime::{Host, NativeRuntime};
use crate::core::{Database, Kernel, TezosHeader};
use tezos_smart_rollup_host::input::Message;

pub struct LowLatency<D>
where
    D: Database,
{
    native_runtime: NativeRuntime<D>,
}

impl<D> LowLatency<D>
where
    D: Database,
{
    pub fn new(database: D) -> Self {
        Self {
            native_runtime: NativeRuntime::new(database),
        }
    }
}

impl<D> crate::core::LowLatency<D> for LowLatency<D>
where
    D: Database,
{
    fn on_message<K: Kernel>(&mut self, message: Message) {
        // TODO Add the message to the runtime
        self.native_runtime.add_message(message);

        // Call the kernel function
        K::entry(&mut self.native_runtime);
    }

    fn on_tezos_header(&mut self, _tezos_header: &TezosHeader) {
        println!("TODO: simulate the end of level and the two first message of the inbox")
    }
}
