use std::collections::HashMap;

use crate::core::{Database, DatabaseError, TreeNode};

/// Database using sled
#[derive(Clone)]
pub struct SledDatabase {
    inner: sled::Db,
}

impl SledDatabase {
    /// Open a connection to the sled database
    pub fn new(path: &str) -> Self {
        let inner = sled::open(path).unwrap();
        Self { inner }
    }

    fn write_node<'a>(&self, node: &'a TreeNode) -> Result<&'a TreeNode, DatabaseError> {
        let bytes = bincode::serialize(&node).map_err(|_| DatabaseError::EncodingError)?;
        let _ = self
            .inner
            .insert(node.key(), bytes)
            .map_err(|_| DatabaseError::IO)?;
        Ok(node)
    }

    /// Retrieve all the subkeys of a key
    ///
    /// For instance, let's assume we habe this path: "/a/b/c/d"
    /// Then the returned hashmap will look like:
    /// HashMap {
    ///    "/"       : ["a"],
    ///    "/a"      : ["b"],
    ///    "/a/b"    : ["c"],
    ///    "/a/b/c"  : ["d"],
    /// }
    fn get_all_subkeys(path: &str) -> HashMap<String, String> {
        let subkeys = HashMap::<String, String>::new();
        let mut splitted = path.split('/');
        let _ = splitted.next();

        let (_, subkeys) =
            splitted.fold(("/".to_string(), subkeys), |(path, mut subkeys), subkey| {
                let next_key = if path == "/" {
                    format!("/{}", subkey)
                } else {
                    format!("{}/{}", path, subkey)
                };

                subkeys.insert(path, subkey.to_string());

                (next_key, subkeys)
            });

        subkeys
    }

    fn add_subkey(&self, path: &str, subkey: &str) -> Result<(), DatabaseError> {
        let node = self.read_node(path)?;
        let node = match node {
            None => TreeNode {
                key: path.to_string(),
                value: None,
                children: vec![subkey.to_string()],
            },
            Some(node) => {
                let subkey = subkey.to_string();
                if !node.children().contains(&subkey) {
                    let mut children = node.children;
                    children.push(subkey);
                    TreeNode { children, ..node }
                } else {
                    node
                }
            }
        };
        let _ = self.write_node(&node)?;
        Ok(())
    }
}

impl Database for SledDatabase {
    fn write<'a>(&self, path: &str, data: &'a [u8]) -> Result<&'a [u8], DatabaseError> {
        println!("write node {}", path);
        let subkeys = SledDatabase::get_all_subkeys(path);
        println!("{:?}", &subkeys);

        // Creates/Update node's subkeys
        for (path, subkey) in subkeys {
            println!("add subkey {} to path {}", subkey, path);
            self.add_subkey(&path, &subkey)?;
        }

        let node = TreeNode {
            key: path.to_string(),
            value: Some(data.to_vec()),
            children: Vec::new(),
        };

        let _ = self.write_node(&node)?;
        Ok(data)
    }

    fn read(&self, path: &str) -> Result<Option<Vec<u8>>, DatabaseError> {
        let node = self.read_node(path)?;
        match node {
            None => Ok(None),
            Some(TreeNode { value, .. }) => Ok(value),
        }
    }

    fn get_subkeys(&self, path: &str) -> Result<Vec<String>, DatabaseError> {
        let node = self.read_node(path)?;
        match node {
            None => Ok(Vec::default()),
            Some(TreeNode { children, .. }) => {
                println!("children of the node {}: {:?}", path, children);
                Ok(children)
            }
        }
    }

    fn delete(&self, path: &str) -> Result<(), DatabaseError> {
        let node = self.read_node(path)?;
        match node {
            None => Ok(()),
            Some(node) => {
                for child in node.children() {
                    let path = if path == "/" {
                        format!("/{}", child)
                    } else {
                        format!("{}/{}", path, child)
                    };
                    println!("deleting: {path}");
                    self.delete(&path)?;
                }
                self.inner.remove(path).map_err(|_| DatabaseError::IO)?;
                Ok(())
            }
        }
    }

    fn read_node(&self, path: &str) -> Result<Option<TreeNode>, DatabaseError> {
        let res = self.inner.get(path).map_err(|_| DatabaseError::IO)?;
        match res {
            None => Ok(None),
            Some(bytes) => {
                let node =
                    bincode::deserialize(&bytes).map_err(|_| DatabaseError::EncodingError)?;
                Ok(Some(node))
            }
        }
    }

    /// Copy a node to a new path
    ///
    /// TODO: this is really not optimized
    fn copy(&self, from: &str, to: &str) -> Result<(), DatabaseError> {
        let node = self.read_node(from)?;
        match node {
            None => Ok(()),
            Some(node) => {
                for child in &node.children() {
                    let from_path = if from == "/" {
                        format!("/{}", child)
                    } else {
                        format!("{}/{}", from, child)
                    };
                    let to_path = if from == "/" {
                        format!("/{}", child)
                    } else {
                        format!("{}/{}", to, child)
                    };
                    self.copy(&from_path, &to_path)?;
                }
                let copied = TreeNode {
                    key: to.to_string(),
                    value: node.value(),
                    children: node.children(),
                };
                self.write_node(&copied)?;

                let subkeys = SledDatabase::get_all_subkeys(to);
                // Creates/Update node's subkeys
                for (path, subkey) in subkeys {
                    self.add_subkey(&path, &subkey)?;
                }
                Ok(())
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::fs;

//     use crate::_database::Database;

//     use super::SledDatabase;

//     /// Wrapper of the SledDatabase to wipe it at then end of the tests
//     struct Db {
//         inner: SledDatabase,
//         path: String,
//     }

//     impl AsRef<SledDatabase> for Db {
//         fn as_ref(&self) -> &SledDatabase {
//             &self.inner
//         }
//     }

//     impl Drop for Db {
//         fn drop(&mut self) {
//             let _ = fs::remove_dir(&self.path);
//         }
//     }

//     impl Default for Db {
//         fn default() -> Self {
//             let path = format!("/tmp/{}", uuid::Uuid::new_v4());
//             let inner = SledDatabase::new(&path);
//             Self { inner, path }
//         }
//     }

//     #[test]
//     fn test_write() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let data = [0x01, 0x02, 0x03, 0x04];
//         let res = database.write("/path", &data);
//         assert!(res.is_ok())
//     }

//     #[test]
//     fn test_read() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let data = [0x01, 0x02, 0x03, 0x04];

//         let _ = database.write("/path", &data).unwrap();
//         let res = database.read("/path").unwrap().unwrap();

//         assert_eq!(res, data.to_vec());
//     }

//     #[test]
//     fn test_read_unknown() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let res = database.read("/path");

//         assert!(res.is_ok());
//         assert!(res.unwrap().is_none());
//     }

//     #[test]
//     fn test_delete() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let data = [0x01, 0x02, 0x03, 0x04];

//         let _ = database.write("/path", &data).unwrap();
//         let () = database.delete("/path").unwrap();
//         let res = database.read("/path").unwrap();

//         assert!(res.is_none());
//     }

//     #[test]
//     fn test_get_subkeys() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let data = [0x01, 0x02, 0x03, 0x04];

//         let _ = database.write("/path/sub", &data).unwrap();
//         let root_res = database.get_subkeys("/").unwrap();
//         let path_res = database.get_subkeys("/path").unwrap();
//         let sub_res = database.get_subkeys("/path/sub").unwrap();

//         assert_eq!(root_res, vec!["path"]);
//         assert_eq!(path_res, vec!["sub"]);
//         assert!(sub_res.is_empty());
//     }

//     #[test]
//     fn test_get_delete() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let data = [0x01, 0x02, 0x03, 0x04];

//         let _ = database.write("/path/sub", &data).unwrap();
//         let _ = database.delete("/").unwrap();

//         let root_res = database.get_subkeys("/").unwrap();
//         let path_res = database.get_subkeys("/path").unwrap();
//         let sub_res = database.get_subkeys("/path/sub").unwrap();

//         let empty: Vec<String> = Vec::default();
//         assert_eq!(root_res, empty);
//         assert_eq!(path_res, empty);
//         assert_eq!(sub_res, empty);
//     }

//     #[test]
//     fn test_copy() {
//         let database = Db::default();
//         let database = database.as_ref();
//         let data = [0x01, 0x02, 0x03, 0x04];

//         let _ = database.write("/path/a/b", &data).unwrap();
//         let _ = database.copy("/path", "/c").unwrap();

//         let c_res = database.get_subkeys("/c").unwrap();
//         let a_res = database.get_subkeys("/c/a").unwrap();
//         let b_res = database.get_subkeys("/c/a/b").unwrap();
//         let root_res = database.get_subkeys("/").unwrap();
//         let copied_res = database.read("/c/a/b").unwrap().unwrap();

//         let empty: Vec<String> = Vec::default();
//         assert_eq!(c_res, vec!["a"]);
//         assert_eq!(a_res, vec!["b"]);
//         assert_eq!(b_res, empty);
//         assert_eq!(root_res, vec!["path", "c"]);
//         assert_eq!(copied_res, data)
//     }
// }
