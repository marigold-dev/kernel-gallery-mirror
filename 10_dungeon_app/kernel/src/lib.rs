use tezos_smart_rollup::{kernel_entry, prelude::*};

pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(_message)) => {
                todo!()
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
