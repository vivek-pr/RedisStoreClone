use std::collections::HashMap;
use std::fmt::Error;
use crate::expiration::Value;
use crate::hasher::hash_key;
use crate::node::Node;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum KVError{
    KeyNotFound,
    SizeLimitReached,
}

impl std::fmt::Display for KVError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
            KVError::KeyNotFound => write!(f, "Key not found"),
            KVError::SizeLimitReached => write!(f, "Size limit reached"),
        }
    }
}

impl std::error::Error for KVError{}


pub struct KVStore {
    pub(crate) store: HashMap<String, Value>,
    size: usize,
    limit: usize,
    replication_factor: usize,
    nodes: Vec<Node>,
}

impl KVStore {
    pub(crate) fn new(nodes: Vec<Node>, limit: usize, replication_factor: usize) -> Self {
        KVStore {
            store: HashMap::new(),
            size: 0,
            limit,
            replication_factor,
            nodes,
        }
    }
    pub(crate) fn put(&mut self, key: String, value: String, ttl: u64) -> Result<(), KVError>{
        if self.size >= self.limit {
            return Err(KVError::SizeLimitReached);
        }
        self.size += 1;
        let value = Value::new(value, ttl);
        self.store.insert(key.clone(), value.clone());
        let nodes = self.get_next_nodes(&key);
        for node in nodes{
            let _res = self.send_data_to_node(node, &key, &value.value, ttl);
        }

        Ok(())
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

    pub(crate) fn keys(&self) -> Vec<String>{
        self.store.keys().cloned().collect()
    }

    fn get_next_nodes(&self, key : &str) -> Vec<&Node>{
        let hashed_key = hash_key(key);
        let mut node_index = (hashed_key % self.nodes.len() as u64) as usize;
        let mut nodes = vec![];
        for _ in 0..self.replication_factor{
            nodes.push(self.nodes.get(node_index).unwrap());
            node_index += 1;
            if node_index >= self.nodes.len(){
                node_index = 0;
            }
        }
        nodes
    }

    fn send_data_to_node(&self, node: &Node, key: &str, value: &str, ttl: u64) -> Result<(), Error>{
        // let url = format!("http://{}:{}/data", node.ip, node.port);
        // let client = reqwest::Client::new();
        // let mut map = HashMap::new();
        // map.insert("key", key);
        // map.insert("value", value);
        // map.insert("ttl", &ttl.to_string());
        // let _res = client.post(&url)
        //     .json(&map)
        //     .send()?;
        Ok(())
    }
}

