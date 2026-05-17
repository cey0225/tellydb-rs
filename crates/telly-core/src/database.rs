use crate::kv::KVPair;
use crate::value::Value;
use std::collections::HashMap;

/// An in-memory key-value database.
pub struct Database {
    pub name: Vec<u8>,
    data: HashMap<Vec<u8>, KVPair>,
}

impl Database {
    pub fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            data: HashMap::new(),
        }
    }

    /// Gets a value by key. Returns None if missing or expired.
    pub fn get(&mut self, key: &[u8]) -> Option<&Value> {
        if self.data.get(key)?.is_expired() {
            self.data.remove(key);
            return None;
        }
        self.data.get(key).map(|kv| &kv.value)
    }

    pub fn set(&mut self, key: Vec<u8>, kv: KVPair) {
        self.data.insert(key, kv);
    }

    pub fn delete(&mut self, key: &[u8]) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn exists(&self, key: &[u8]) -> bool {
        self.data.get(key).map_or(false, |kv| !kv.is_expired())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Registry of all databases. Supports multiple named databases.
pub struct Databases {
    dbs: Vec<Database>,
}

impl Databases {
    pub fn new() -> Self {
        Self {
            dbs: vec![Database::new(b"main".to_vec())],
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&mut Database> {
        self.dbs.get_mut(index)
    }

    pub fn get_by_name(&mut self, name: &[u8]) -> Option<&mut Database> {
        self.dbs.iter_mut().find(|db| db.name == name)
    }

    pub fn create(&mut self, name: Vec<u8>) -> &mut Database {
        self.dbs.push(Database::new(name));
        self.dbs.last_mut().unwrap()
    }

    pub fn len(&self) -> usize {
        self.dbs.len()
    }
}
