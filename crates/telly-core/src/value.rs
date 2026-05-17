use std::collections::HashMap;

/// A value stored in the database.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i64),
    Double(f64),
    String(Vec<u8>),
    Boolean(bool),
    List(Vec<Value>),
    Hash(HashMap<Vec<u8>, Value>),
}

impl Value {
    /// Returns the type name as a string (used for TYPE command).
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Integer(_) => "int",
            Value::Double(_) => "double",
            Value::String(_) => "string",
            Value::Boolean(_) => "bool",
            Value::List(_) => "list",
            Value::Hash(_) => "hashtable",
        }
    }
}
