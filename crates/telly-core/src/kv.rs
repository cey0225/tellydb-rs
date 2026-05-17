use crate::value::Value;
use std::time::Instant;

/// A key-value pair stored in the database.
///
/// Holds the value and an optional expiry time.
#[derive(Debug, Clone)]
pub struct KVPair {
    pub value: Value,
    pub expire_at: Option<Instant>,
}

impl KVPair {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            expire_at: None,
        }
    }

    pub fn with_expiry(value: Value, expire_at: Instant) -> Self {
        Self {
            value,
            expire_at: Some(expire_at),
        }
    }

    /// Returns true if the pair has expired.
    pub fn is_expired(&self) -> bool {
        self.expire_at.map_or(false, |t| Instant::now() >= t)
    }
}
