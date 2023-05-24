use tezos_smart_rollup_host::input::Message;

use super::{database::Database, kernel::Kernel, tezos_header::TezosHeader};

pub trait LowLatency<D>
where
    D: Database,
{
    fn on_message<K: Kernel>(&mut self, message: Message);
    fn on_tezos_header(&mut self, tezos_header: &TezosHeader);
}
