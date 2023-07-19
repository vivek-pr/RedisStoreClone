#[cfg(test)]
mod tests {
    use crate::models::KVStore;
    use super::*;
    #[test]
    fn test_create_kv_store(){
        let kv_store = KVStore::new();
        assert_eq!(kv_store.store.len(), 0);
    }
}