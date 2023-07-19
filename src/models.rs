use std::collections::HashMap;

pub struct KVStore {
    pub(crate) store: HashMap<String, String>,
}

impl KVStore {
    pub(crate) fn new() -> Self {
        KVStore {
            store: HashMap::new(),
        }
    }
    pub(crate) fn put(&mut self, key: String, value: String){
        self.store.insert(key, value);
    }
}