use std::collections::HashMap;
use std::sync::Mutex;

pub struct QueryCache {
    cache: Mutex<HashMap<String, serde_json::Value>>,
}

impl QueryCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, query: &str) -> Option<serde_json::Value> {
        let cache = self.cache.lock().unwrap();
        cache.get(query).cloned()
    }

    pub fn insert(&self, query: &str, value: serde_json::Value) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(query.to_string(), value);
    }
}

impl Default for QueryCache {
    fn default() -> Self {
        Self::new()
    }
}
