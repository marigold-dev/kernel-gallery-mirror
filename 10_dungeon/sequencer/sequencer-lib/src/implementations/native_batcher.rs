use tezos_smart_rollup_host::input::Message;

use crate::core::TezosHeader;

pub struct NativeBatcher {
    tezos_level: u32,
    batch: Vec<Vec<u8>>,
}

impl NativeBatcher {
    pub fn new() -> Self {
        Self {
            tezos_level: 0,
            batch: Vec::default(),
        }
    }
}

impl crate::core::Batcher for NativeBatcher {
    fn on_operation(&mut self, operation: Vec<u8>) -> Message {
        let message_payload = {
            let mut data = vec![0x01];
            let mut payload = operation.clone();
            data.append(&mut payload);
            data
        };

        let index = self.batch.len().try_into().unwrap(); // TODO: should we increment the index by 2 ? (because of the SOL and IOL)
        let msg = Message::new(self.tezos_level, index, message_payload);

        // Add the message to the batch
        self.batch.push(operation);

        msg
    }

    fn on_tezos_header(&mut self, tezos_header: &TezosHeader) -> Vec<Vec<u8>> {
        self.tezos_level = tezos_header.level;
        let batch = self.batch.clone();
        self.batch = Vec::default();
        batch
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::Batcher, implementations::NativeBatcher};

    #[test]
    fn test_message_is_added() {
        let mut batcher = NativeBatcher::new();
        let payload = vec![0x02, 0x03, 0x04];
        let _ = batcher.on_operation(payload);

        assert_eq!(1, batcher.batch.len())
    }

    #[test]
    fn test_external_byte() {
        let mut batcher = NativeBatcher::new();
        let payload = vec![0x02, 0x03, 0x04];
        let msg = batcher.on_operation(payload.clone());

        let msg_payload = msg.as_ref();

        assert_eq!(msg_payload[0], 0x01);
        assert_eq!(
            msg_payload.iter().skip(1).copied().collect::<Vec<u8>>(),
            payload
        );
    }
}
