use std::collections::HashMap;
use crate::hasher::hash_key;
use crate::node::Node;

pub struct KVStore {
    pub(crate) store: HashMap<String, String>,
    nodes: Vec<Node>,
}

impl KVStore {
    pub(crate) fn new(nodes: Vec<Node>) -> Self {
        KVStore {
            store: HashMap::new(),
            nodes,
        }
    }
    pub(crate) fn put(&mut self, key: String, value: String){
        self.store.insert(key, value);
    }

    pub(crate) fn get(&self, key: &str) -> Option<&String>{
        self.store.get(key)
    }

    pub(crate) fn delete(&mut self, key: &str)-> Option<String>{
        self.store.remove(key)
    }

    pub(crate) fn get_node_for_key(&self, key: &str) -> Option<&Node>{
        let hashed_key = hash_key(key);
        let node_index = (hashed_key % self.nodes.len() as u64) as usize;
        self.nodes.get(node_index)
    }
}

