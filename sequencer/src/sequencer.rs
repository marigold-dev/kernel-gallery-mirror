use tezos_smart_rollup_host::input::Message;

pub struct Sequencer {
    tezos_level: u32,
    batch: Vec<Vec<u8>>,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            tezos_level: 0,
            batch: Vec::default(),
        }
    }

    /// Add the operations to the current batch
    pub fn on_operation(&mut self, payload: Vec<u8>) -> Message {
        let index = self.batch.len().try_into().unwrap(); // TODO: should we increment the index by 2 ? (because of the SOL and IOL)
        let msg = Message::new(self.tezos_level, index, payload.clone());

        // Add the message to the batch
        self.batch.push(payload);

        msg
    }
}
