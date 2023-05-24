use super::tezos_header::TezosHeader;
use tokio::sync::mpsc::Sender;

pub trait ListenTezosHeader {
    fn listen_tezos_header(&self, sender: Sender<TezosHeader>);
}
