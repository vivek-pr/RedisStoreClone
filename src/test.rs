#[cfg(test)]
mod tests {
    use crate::models::{KVError, KVStore};
    use crate::node::Node;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_create_kv_store(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let kv_store = KVStore::new(nodes, 100, 3);
        assert_eq!(kv_store.store.len(), 0);
    }

    #[test]
    fn test_put(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kvstore = KVStore::new(nodes, 100, 3);
        kvstore.put("key".to_string(), "value".to_string(), 200);
        assert_eq!(kvstore.store.len(), 1);
        assert_eq!(kvstore.store.get("key").unwrap().value, "value".to_string());
    }

    #[test]
    fn test_get(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kv_store = KVStore::new(nodes, 100, 3);
        kv_store.put("key".to_string(), "value".to_string(), 200);
        assert_eq!(kv_store.get("key"), Ok(&"value".to_string()));
        assert_eq!(kv_store.get("key2"), Err(KVError::KeyNotFound));

    }

    #[test]
    fn test_delete(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kv_store = KVStore::new(nodes,100, 3);
        kv_store.put("key".to_string(), "value".to_string(), 200);
        assert_eq!(kv_store.get("key"), Ok(&"value".to_string()));
        assert_eq!(kv_store.get("key2"), Err(KVError::KeyNotFound));
    }

    #[test]
    fn test_hash_key(){
        let key = "key";
        let hashed_key = crate::hasher::hash_key(key);
        assert!(hashed_key > 0);

    }

    #[test]
    fn test_get_node_for_key(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let kv_store = KVStore::new(nodes, 100, 3);
        let node = kv_store.get_node_for_key("key");
        assert!(node.is_some());
    }

    #[test]
    fn test_ttl(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kv_store = KVStore::new(nodes, 100, 3);
        kv_store.put("key".to_string(), "value".to_string(), 1);
        assert_eq!(kv_store.get("key"), Ok(&"value".to_string()));
        thread::sleep(Duration::from_secs(2));
        assert_eq!(kv_store.get("key"), Err(KVError::KeyNotFound));
    }

    #[test]
    fn test_get_nonexistent_key(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let kv_store = KVStore::new(nodes, 100, 3);
        assert_eq!(kv_store.get("key"), Err(KVError::KeyNotFound));
    }

    #[test]
    fn test_delete_nonexistent_key(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kv_store = KVStore::new(nodes, 100, 3);
        assert_eq!(kv_store.delete("key"), Err(KVError::KeyNotFound));
    }

    #[test]
    fn test_size_limit() {
        let mut kvstore = KVStore::new(vec![], 1, 3);
        assert_eq!(kvstore.put("key1".to_string(), "value1".to_string(), 1), Ok(()));
        assert_eq!(kvstore.put("key2".to_string(), "value2".to_string(), 1), Err(KVError::SizeLimitReached));
    }

    #[test]
    fn test_keys(){
        let mut kvstore = KVStore::new(vec![], 4, 3);
        kvstore.put("key1".to_string(), "value1".to_string(), 1).unwrap();
        kvstore.put("key2".to_string(), "value2".to_string(), 1).unwrap();
        kvstore.put("key3".to_string(), "value3".to_string(), 1).unwrap();
        let keys = kvstore.keys();
        assert_eq!(keys.contains(&"key1".to_string()), true);
        assert_eq!(keys.contains(&"key2".to_string()), true);
        assert_eq!(keys.contains(&"key3".to_string()), true);
    }
}