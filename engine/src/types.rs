use rug::{Integer, Float, Assign};
use std::collections::HashMap;

/// Record of data
#[derive(Debug, Clone)]
pub(crate) enum RecordData {
    Integer(Integer),
    Float(Float),
    String(String),
    Dict(Dictionary)
}

/// A single record
#[derive(Debug, Clone)]
pub(crate) struct Record {
    name: String,
    data: RecordData
}

impl Record {

    /// Create a new record
    pub(crate) fn new(name: String, record: RecordData) -> Self {
        Self {
            name: name,
            data: record
        }
    }

    /// Get record data
    pub(crate) fn get_data(&self) -> RecordData {
        return self.data.clone();
    }

    /// Get record name
    pub(crate) fn get_name(&self) -> String {
        return self.name.clone();
    }
}

/// A dictionary of data
#[derive(Debug, Clone)]
pub(crate) struct Dictionary {
    data: HashMap<String, Record>
}

impl Dictionary {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    /// Get a record
    pub(crate) fn get(&self, key: &String) -> Option<Record> {
        match self.data.get(key) {
            Some(record) => {
                return Some(record.clone());
            }
            None => {
                return None;
            }
        }
    }

    /// Set a record to record data
    pub(crate) fn set(&mut self, key: &String, value: RecordData) {

        self.data.insert(key.clone(), Record::new(key.clone(), value));
    }
}

