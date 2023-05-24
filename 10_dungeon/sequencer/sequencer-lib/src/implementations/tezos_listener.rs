use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;

use crate::core::{ListenTezosHeader, TezosHeader};

pub struct TezosListener {
    url: String,
}

impl ListenTezosHeader for TezosListener {
    fn listen_tezos_header(&self, sender: Sender<TezosHeader>) {
        let url = format!("{}/monitor/heads/main", self.url);

        tokio::spawn(async move {
            let mut stream = reqwest::get(url).await.unwrap().bytes_stream();

            while let Some(Ok(item)) = stream.next().await {
                let bytes = item.to_vec();

                let header = serde_json::from_slice::<TezosHeader>(&bytes);
                match header {
                    Err(_) => {}
                    Ok(header) => {
                        let _ = sender.send(header).await;
                    }
                }
            }
        });
    }
}

impl TezosListener {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
