use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Value{
    pub(crate) value: String,
    pub(crate) insertion_time: u64,
    pub(crate) ttl: u64,
}

impl Value{
    pub(crate) fn new(value: String, ttl: u64) -> Self{
        let insertion_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Value{
            value,
            insertion_time,
            ttl,
        }
    }

    pub(crate) fn has_expired(&self) -> bool{
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        current_time - self.insertion_time > self.ttl
    }
}