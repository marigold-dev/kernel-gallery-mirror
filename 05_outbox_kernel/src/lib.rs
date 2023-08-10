// SPDX-FileCopyrightText: 2023 Marigold <contact@marigold.dev>
// SPDX-FileCopyrightText: 2023 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

use tezos_crypto_rs::hash::SmartRollupHash;
use tezos_data_encoding::enc::BinWriter;
use tezos_smart_rollup::{
    inbox::{InboxMessage, InternalInboxMessage},
    kernel_entry,
    michelson::{Michelson, MichelsonInt},
    outbox::{OutboxMessage, OutboxMessageTransaction, OutboxMessageTransactionBatch},
    prelude::*,
    types::{Contract, Entrypoint},
};

// Read inbox messages, only looking at internal transfer messages directed to
// this kernel's address. For each such message, write an outbox message
// addressed to a predetermined Layer 1 contract containing the same Michelson
// payload as the inbox message.
fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime, own_address: &SmartRollupHash) {
    loop {
        match host.read_input() {
            Ok(Some(message)) => {
                // Parse the payload of the message
                let parsed_message = InboxMessage::<Expr>::parse(message.as_ref());
                if let Ok((remaining, InboxMessage::Internal(msg))) = parsed_message {
                    debug_assert!(remaining.is_empty());
                    if let InternalInboxMessage::Transfer(m) = msg {
                        if m.destination.hash() == own_address {
                            // If the message is addressed to me, push a message
                            // to the outbox
                            debug_msg!(host, "Internal message: transfer for me\n");
                            write_outbox_message(host, m.payload);
                        } else {
                            debug_msg!(host, "Internal message: transfer not for me\n")
                        }
                    }
                }
            }
            Ok(None) => break,
            Err(_) => continue,
        }
    }
}

// Outbox messages are smart contract calls which can be executed once
// the commitment containing them has been cemented. For simplicity, we
// hardcode a contract address and an entrypoint which all the outbox
// messages will refer to.
const L1_CONTRACT_ADDRESS: &str = "KT1RycYvM4EVs6BAXWEsGXaAaRqiMP53KT4w";

// See `smart_contract/counter.jsligo` for a simple smart contract
// example implementing an entrypoint of the same type as this kernel (int).
const L1_CONTRACT_ENTRYPOINT: &str = "default";

fn write_outbox_message<Expr: Michelson>(host: &mut impl Runtime, payload: Expr) {
    let destination = Contract::from_b58check(L1_CONTRACT_ADDRESS).unwrap();
    let entrypoint = Entrypoint::try_from(L1_CONTRACT_ENTRYPOINT.to_string()).unwrap();
    let transaction = OutboxMessageTransaction {
        parameters: payload,
        destination,
        entrypoint,
    };
    // A batch groups transactions which need to succeed together
    let batch = OutboxMessageTransactionBatch::from(vec![transaction]);
    let message = OutboxMessage::AtomicTransactionBatch(batch);
    let mut output = Vec::default();
    message.bin_write(&mut output).unwrap();
    host.write_output(&output).unwrap();
}

pub fn entry(host: &mut impl Runtime) {
    // Get own address using the `reveal_metadata` host function
    // in order to only handle internal messages sent to this
    // kernel.
    let own_address = host.reveal_metadata().address();
    read_inbox_message::<MichelsonInt>(host, &own_address);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);

// Native unit tests can be written using `MockHost` and can then be
// run with `cargo test`.
#[cfg(test)]
mod test {
    use super::*;
    use tezos_crypto_rs::hash::HashType::ContractKt1Hash;
    use tezos_data_encoding::nom::NomReader;
    use tezos_smart_rollup::{
        testing::prelude::{MockHost, TransferMetadata},
        types::{PublicKeyHash, SmartRollupAddress},
    };

    const SENDER: &str = "KT1EfTusMLoeCAAGd9MZJn5yKzFr6kJU5U91";
    const SOURCE: &str = "tz1SodoUsWVe1Yey9eMFbqRUtNpBWfir5NRr";
    const OTHER_ADDR: &str = "sr1RYurGZtN8KNSpkMcCt9CgWeUaNkzsAfXf";

    // Check that if the inbox contains a transfer message addressed to
    // this rollup an outbox message with the same payload will be written
    #[test]
    fn transfer_outbox() {
        let mut host = MockHost::default();

        // Construct an internal message from a smart contract containing
        // data of type int and inject it into the test inbox using the
        // mock `add_transfer` function.
        let sender = ContractKt1Hash.b58check_to_hash(SENDER).unwrap();
        let source = PublicKeyHash::from_b58check(SOURCE).unwrap();
        let metadata = TransferMetadata::new(sender, source);
        let payload = MichelsonInt::from(32);
        host.add_transfer(payload, &metadata);

        // Execute the kernel, read the outbox message, and check its payload.
        entry(&mut host);
        let (_, f) = OutboxMessageTransaction::<MichelsonInt>::nom_read(
            &host.outbox_at(host.level())[0].as_slice()[5..],
        )
        .unwrap();
        assert!(f.parameters == MichelsonInt::from(32));
    }

    #[test]
    // Check that if the inbox only contains a transfer message addressed to
    // a different rollup no outbox message is written
    fn transfer_ignore() {
        let mut host = MockHost::default();

        let sender = ContractKt1Hash.b58check_to_hash(SENDER).unwrap();
        let source = PublicKeyHash::from_b58check(SOURCE).unwrap();
        let mut metadata = TransferMetadata::new(sender, source);
        let destination = SmartRollupAddress::from_b58check(OTHER_ADDR).unwrap();
        metadata.override_destination(destination);
        let payload = MichelsonInt::from(32);
        host.add_transfer(payload, &metadata);

        entry(&mut host);
        assert!(host.outbox_at(host.level()).is_empty());
    }
}
