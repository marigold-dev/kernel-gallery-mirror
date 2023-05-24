use super::{
    batcher::Batcher, database::Database, injector::Injector, kernel::Kernel,
    listen_tezos_header::ListenTezosHeader, low_latency::LowLatency, tezos_header::TezosHeader,
};
use async_trait::async_trait;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

#[async_trait]
pub trait Node {
    async fn submit_operation(&self, operation: Vec<u8>);
    async fn get_value(&self, path: &str) -> Option<Vec<u8>>;
    async fn get_subkeys(&self, path: &str) -> Option<Vec<String>>;
}

enum QueueContent {
    Message(Vec<u8>),
    TezosHeader(TezosHeader),
}

struct QueueMsg {
    promise: Option<oneshot::Sender<()>>,
    content: QueueContent,
}

pub struct NodeImpl<D: Database> {
    sender: Sender<QueueMsg>,
    database: D,
}

#[async_trait]
impl<D: Database + Send + Sync> Node for NodeImpl<D> {
    async fn submit_operation(&self, operation: Vec<u8>) {
        let (tx, rx) = oneshot::channel::<()>();
        let msg = QueueMsg {
            promise: Some(tx),
            content: QueueContent::Message(operation),
        };
        let _ = self.sender.send(msg).await;
        let _ = rx.await;
        println!("Message submitted")
    }

    async fn get_value(&self, path: &str) -> Option<Vec<u8>> {
        match self.database.read(path) {
            Ok(Some(value)) => Some(value),
            _ => None,
        }
    }

    async fn get_subkeys(&self, path: &str) -> Option<Vec<String>> {
        if path == "/" {
            return None;
        }
        match self.database.get_subkeys(path) {
            Ok(value) => Some(value),
            _ => None,
        }
    }
}

pub trait NodeBuilder<A, B, C, D, E>
where
    A: ListenTezosHeader + Send + Sync + 'static,
    B: LowLatency<D> + Send + Sync + 'static,
    C: Batcher + Send + Sync + 'static,
    D: Database,
    E: Injector + Send + Sync + 'static,
    Self: Sized,
{
    fn start<K>(
        tezos_listener: A,
        mut low_latency: B,
        mut batcher: C,
        database: D,
        injector: E,
    ) -> NodeImpl<D>
    where
        K: Kernel,
    {
        let (tx, mut rx) = mpsc::channel::<QueueMsg>(1024);

        let _ = tokio::spawn(async move {
            let mut running = true;
            while running {
                match rx.recv().await {
                    None => running = false,
                    Some(QueueMsg { promise, content }) => {
                        if let Some(promise) = promise {
                            let _ = promise.send(());
                        }

                        match content {
                            QueueContent::Message(msg) => {
                                let msg = batcher.on_operation(msg);
                                low_latency.on_message::<K>(msg);
                            }
                            QueueContent::TezosHeader(tezos_header) => {
                                let batch = batcher.on_tezos_header(&tezos_header);
                                low_latency.on_tezos_header(&tezos_header);
                                let _ = injector.inject(batch).await;
                            }
                        }
                    }
                }
            }
        });

        let listener_sender = tx.clone();

        let _ = tokio::spawn(async move {
            let (sender, mut receiver) = mpsc::channel(32);
            tezos_listener.listen_tezos_header(sender);
            while let Some(tezos_header) = receiver.recv().await {
                let msg = QueueMsg {
                    promise: None,
                    content: QueueContent::TezosHeader(tezos_header),
                };
                let _ = listener_sender.send(msg).await;
            }
        });

        NodeImpl {
            sender: tx,
            database,
        }
    }
}
