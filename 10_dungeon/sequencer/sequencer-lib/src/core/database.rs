use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DatabaseError {
    EncodingError,
    IO,
}

#[derive(Serialize, Deserialize)]
pub struct TreeNode {
    pub key: String,            // More convenient
    pub value: Option<Vec<u8>>, // Sometime a node does not have any data
    pub children: Vec<String>,  // Array of subkeys
}

impl TreeNode {
    /// Get the children of the node
    pub fn children(&self) -> Vec<String> {
        self.children.to_vec()
    }

    /// Get the value of the node
    pub fn value(&self) -> Option<Vec<u8>> {
        self.value.as_ref().cloned()
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}

pub trait Database: Clone {
    /// Write the given data to the given path
    fn write<'a>(&self, path: &str, data: &'a [u8]) -> Result<&'a [u8], DatabaseError>;

    /// Read the data from the given path
    fn read(&self, path: &str) -> Result<Option<Vec<u8>>, DatabaseError>;

    /// Returns the subkeys of at the given path
    fn get_subkeys(&self, path: &str) -> Result<Vec<String>, DatabaseError>;

    /// Deletes the data at a given path
    ///
    /// It also deletes all the subkeys
    fn delete(&self, path: &str) -> Result<(), DatabaseError>;

    /// Read a node
    fn read_node(&self, path: &str) -> Result<Option<TreeNode>, DatabaseError>;

    /// Copy a node to a new path
    fn copy(&self, from: &str, to: &str) -> Result<(), DatabaseError>;
}
