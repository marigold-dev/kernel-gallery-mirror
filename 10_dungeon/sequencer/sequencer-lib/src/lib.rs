mod core;
mod implementations;

pub use crate::core::Kernel;
pub use crate::core::Node;
use crate::core::NodeBuilder;
use crate::core::NodeImpl;
use crate::implementations::*;
use async_trait::async_trait;
pub use tezos_smart_rollup_host::runtime::Runtime;

// struct NativeNodeBuilder {}

pub struct NativeNode(NodeImpl<SledDatabase>);

impl
    NodeBuilder<
        TezosListener,
        LowLatency<SledDatabase>,
        NativeBatcher,
        SledDatabase,
        RollupBatcherInjector,
    > for NativeNode
{
}

impl NativeNode {
    /// It will create a native sequencer/low latency
    /// This node will listen to tezos, it comes with its own database
    pub fn new<K: Kernel>(
        sled_database_uri: &str,
        tezos_node_uri: &str,
        rollup_node_uri: &str,
    ) -> impl Node {
        let database = SledDatabase::new(sled_database_uri);
        let tezos_listener = TezosListener::new(tezos_node_uri.to_string());
        let low_latency = LowLatency::new(database.clone());
        let injector = RollupBatcherInjector::new(rollup_node_uri.to_string());
        let sequencer = NativeBatcher::new();
        let comm =
            NativeNode::start::<K>(tezos_listener, low_latency, sequencer, database, injector);
        NativeNode(comm)
    }
}

#[async_trait]
impl Node for NativeNode {
    async fn submit_operation(&self, operation: Vec<u8>) -> () {
        self.0.submit_operation(operation).await
    }

    async fn get_value(&self, path: &str) -> Option<Vec<u8>> {
        self.0.get_value(path).await
    }

    async fn get_subkeys(&self, path: &str) -> Option<Vec<String>> {
        self.0.get_subkeys(path).await
    }
}
