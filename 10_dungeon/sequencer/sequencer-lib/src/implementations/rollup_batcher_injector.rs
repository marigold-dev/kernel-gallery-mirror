use async_trait::async_trait;
use reqwest::{Client, StatusCode};

use crate::core::Injector;

pub struct RollupBatcherInjector {
    rollup_url: String,
    client: Client,
}

impl RollupBatcherInjector {
    pub fn new(rollup_url: String) -> Self {
        Self {
            rollup_url,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Injector for RollupBatcherInjector {
    async fn inject(&self, payload: Vec<Vec<u8>>) -> Result<(), ()> {
        if payload.is_empty() {
            return Ok(());
        }

        let batch = payload.iter().map(hex::encode).collect::<Vec<String>>();

        let str = serde_json::to_string(&batch).unwrap();

        let res = self
            .client
            .post(format!("{}/local/batcher/injection", &self.rollup_url))
            .body(str)
            .send()
            .await;

        match res {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}
