use std::collections::HashMap;
use std::fmt::Error;
use crate::expiration::Value;
use crate::hasher::hash_key;
use crate::node::Node;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum KVError{
    KeyNotFound,
}

impl std::fmt::Display for KVError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
            KVError::KeyNotFound => write!(f, "Key not found"),
        }
    }
}

impl std::error::Error for KVError{}


pub struct KVStore {
    pub(crate) store: HashMap<String, Value>,
    nodes: Vec<Node>,
}

impl KVStore {
    pub(crate) fn new(nodes: Vec<Node>) -> Self {
        KVStore {
            store: HashMap::new(),
            nodes,
        }
    }
    pub(crate) fn put(&mut self, key: String, value: String, ttl: u64){
        let value = Value::new(value, ttl);
        self.store.insert(key, value);
    }

    pub(crate) fn get(&self, key: &str) -> Result<&String, KVError>{
        match self.store.get(key){
            Some(value) if !value.has_expired() => Ok(&value.value),
            _ => Err(KVError::KeyNotFound),
        }
    }

    pub(crate) fn delete(&mut self, key: &str)-> Result<String, KVError>{
        match self.store.remove(key){
            Some(value) => Ok(value.value),
            None => Err(KVError::KeyNotFound),
        }
    }

    pub(crate) fn get_node_for_key(&self, key: &str) -> Option<&Node>{
        let hashed_key = hash_key(key);
        let node_index = (hashed_key % self.nodes.len() as u64) as usize;
        self.nodes.get(node_index)
    }
}

