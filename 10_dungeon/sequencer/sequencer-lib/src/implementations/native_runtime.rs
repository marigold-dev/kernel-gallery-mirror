use std::collections::VecDeque;

use tezos_smart_rollup_host::{
    input::Message,
    path::Path,
    runtime::{Runtime, RuntimeError, ValueType},
    Error,
};

use crate::core::Database;

pub struct NativeRuntime<D>
where
    D: Database,
{
    inputs: VecDeque<Message>,
    db: D,
}

impl<D> NativeRuntime<D>
where
    D: Database,
{
    pub fn new(db: D) -> Self {
        NativeRuntime {
            inputs: VecDeque::default(),
            db,
        }
    }
}

/// Check the size of the data
///
/// The data should have a size greater than 2^31
pub fn check_data_size(data: &[u8]) -> Result<&[u8], RuntimeError> {
    i32::try_from(data.len())
        .map_err(|_| RuntimeError::HostErr(Error::StoreValueSizeExceeded))
        .map(|_| data)
}

pub trait Host<D: Database>: Runtime {
    fn add_message(&mut self, msg: Message);
}

impl<D> Host<D> for NativeRuntime<D>
where
    D: Database,
{
    fn add_message(&mut self, msg: Message) {
        self.inputs.push_back(msg);
    }
}

impl<D> Runtime for NativeRuntime<D>
where
    D: Database,
{
    fn write_output(&mut self, _from: &[u8]) -> Result<(), RuntimeError> {
        todo!()
    }

    fn write_debug(&self, msg: &str) {
        println!("Write_debug: {}", &msg);
    }

    fn read_input(&mut self) -> Result<Option<Message>, RuntimeError> {
        Ok(self.inputs.pop_front())
    }

    fn store_has<T: Path>(&self, path: &T) -> Result<Option<ValueType>, RuntimeError> {
        let path = std::str::from_utf8(path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;
        self.db
            .read_node(path)
            .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))
            .map(|node| match node {
                None => None,
                Some(node) => match (node.value(), node.children().is_empty()) {
                    (None, true) => None,
                    (None, false) => Some(ValueType::Subtree),
                    (Some(_), true) => Some(ValueType::Value),
                    (Some(_), false) => Some(ValueType::ValueWithSubtree),
                },
            })
    }

    fn store_read<T: Path>(
        &self,
        path: &T,
        from_offset: usize,
        max_bytes: usize,
    ) -> Result<Vec<u8>, RuntimeError> {
        let NativeRuntime { db, .. } = self;
        let path = std::str::from_utf8(path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        let res = db.read(path);
        match res {
            Ok(Some(vec)) => {
                let mut data = vec
                    .to_vec()
                    .iter()
                    .skip(from_offset)
                    .copied()
                    .collect::<Vec<u8>>();
                if data.len() > max_bytes {
                    data.resize(max_bytes, 0);
                }
                Ok(data)
            }
            Err(_) => Err(RuntimeError::HostErr(Error::GenericInvalidAccess)),
            Ok(None) => Err(RuntimeError::PathNotFound),
        }
    }

    fn store_read_slice<T: Path>(
        &self,
        path: &T,
        from_offset: usize,
        buffer: &mut [u8],
    ) -> Result<usize, RuntimeError> {
        let path = std::str::from_utf8(path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        let res = self
            .db
            .read(path)
            .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))?;

        match res {
            Some(res) => {
                let data = res.iter().skip(from_offset).copied().collect::<Vec<u8>>();
                buffer.copy_from_slice(&data);
                Ok(data.len())
            }
            None => Ok(0),
        }
    }

    fn store_write<T: Path>(
        &mut self,
        path: &T,
        src: &[u8],
        at_offset: usize,
    ) -> Result<(), RuntimeError> {
        let NativeRuntime { db, .. } = self;
        let path = std::str::from_utf8(path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        let src = check_data_size(src)?;
        let data = src.iter().skip(at_offset).copied().collect::<Vec<u8>>();
        let res = db.write(path, &data);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::HostErr(Error::GenericInvalidAccess)),
        }
    }

    fn store_delete<T: Path>(&mut self, path: &T) -> Result<(), RuntimeError> {
        let path = std::str::from_utf8(path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        self.db
            .delete(path)
            .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))
    }

    fn store_count_subkeys<T: Path>(&self, prefix: &T) -> Result<u64, RuntimeError> {
        let path = std::str::from_utf8(prefix.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        let node = self
            .db
            .read_node(path)
            .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))?;

        match node {
            None => Err(RuntimeError::HostErr(Error::StoreNotANode)),
            Some(node) => u64::try_from(node.children().len())
                .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess)),
        }
    }

    // fn store_get_subkey<T: Path>(
    //     &self,
    //     prefix: &T,
    //     index: i64,
    // ) -> Result<Option<OwnedPath>, RuntimeError> {
    //     let path = std::str::from_utf8(prefix.as_bytes())
    //         .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

    //     let node = self
    //         .db
    //         .read_node(path)
    //         .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))?;

    //     let index = usize::try_from(index)
    //         .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidSubkeyIndex))?;

    //     match node {
    //         None => Err(RuntimeError::HostErr(Error::StoreNotANode)),
    //         Some(node) => match node.children().get::<usize>(index) {
    //             Some(path) => OwnedPath::try_from(path.to_string())
    //                 .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidSubkeyIndex))
    //                 .map(Some),
    //             None => Err(RuntimeError::HostErr(Error::StoreInvalidSubkeyIndex)),
    //         },
    //     }
    // }

    fn store_move(
        &mut self,
        from_path: &impl Path,
        to_path: &impl Path,
    ) -> Result<(), RuntimeError> {
        self.store_copy(from_path, to_path)?;
        self.store_delete(from_path)?;
        Ok(())
    }

    fn store_copy(
        &mut self,
        from_path: &impl Path,
        to_path: &impl Path,
    ) -> Result<(), RuntimeError> {
        let from = std::str::from_utf8(from_path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        let to = std::str::from_utf8(to_path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        self.db
            .copy(from, to)
            .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))
    }

    fn reveal_preimage(
        &self,
        _hash: &[u8; tezos_smart_rollup_core::PREIMAGE_HASH_SIZE],
        _destination: &mut [u8],
    ) -> Result<usize, RuntimeError> {
        todo!()
    }

    fn store_value_size(&self, path: &impl Path) -> Result<usize, RuntimeError> {
        let path = std::str::from_utf8(path.as_bytes())
            .map_err(|_| RuntimeError::HostErr(Error::StoreInvalidKey))?;

        let data = self
            .db
            .read(path)
            .map_err(|_| RuntimeError::HostErr(Error::GenericInvalidAccess))?;

        Ok(data.map(|data| data.len()).unwrap_or_default())
    }

    fn mark_for_reboot(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn reveal_metadata(
        &self,
    ) -> Result<tezos_smart_rollup_host::metadata::RollupMetadata, RuntimeError> {
        todo!()
    }

    fn last_run_aborted(&self) -> Result<bool, RuntimeError> {
        todo!()
    }

    fn upgrade_failed(&self) -> Result<bool, RuntimeError> {
        todo!()
    }

    fn restart_forced(&self) -> Result<bool, RuntimeError> {
        todo!()
    }

    fn reboot_left(&self) -> Result<u32, RuntimeError> {
        Ok(1000)
    }

    fn runtime_version(&self) -> Result<String, RuntimeError> {
        todo!()
    }
}
