#[cfg(test)]
mod tests {
    use crate::models::KVStore;


    #[test]
    fn test_create_kv_store(){
        let kv_store = KVStore::new();
        assert_eq!(kv_store.store.len(), 0);
    }

    #[test]
    fn test_put(){
        let mut kvstore = KVStore::new();
        kvstore.put("key".to_string(), "value".to_string());
        assert_eq!(kvstore.store.len(), 1);
        assert_eq!(kvstore.store.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_get(){
        let mut kv_store = KVStore::new();
        kv_store.put("key".to_string(), "value".to_string());
        assert_eq!(kv_store.get("key"), Some(&"value".to_string()));
        assert_eq!(kv_store.get("key2"), None);

    }

    #[test]
    fn test_delete(){
        let mut kv_store = KVStore::new();
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
}