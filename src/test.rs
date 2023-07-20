#[cfg(test)]
mod tests {
    use crate::models::KVStore;
    use crate::node::Node;

    #[test]
    fn test_create_kv_store(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let kv_store = KVStore::new(nodes);
        assert_eq!(kv_store.store.len(), 0);
    }

    #[test]
    fn test_put(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kvstore = KVStore::new(nodes);
        kvstore.put("key".to_string(), "value".to_string());
        assert_eq!(kvstore.store.len(), 1);
        assert_eq!(kvstore.store.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_get(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kv_store = KVStore::new(nodes);
        kv_store.put("key".to_string(), "value".to_string());
        assert_eq!(kv_store.get("key"), Some(&"value".to_string()));
        assert_eq!(kv_store.get("key2"), None);

    }

    #[test]
    fn test_delete(){
        let nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let mut kv_store = KVStore::new(nodes);
        kv_store.put("key".to_string(), "value".to_string());
        assert_eq!(kv_store.delete("key"), Some("value".to_string()));
        assert_eq!(kv_store.delete("key"), None);
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
        let kv_store = KVStore::new(nodes);
        let node = kv_store.get_node_for_key("key");
        assert!(node.is_some());
    }
}