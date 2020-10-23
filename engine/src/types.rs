
use std::{ cell::RefCell, rc::Rc };

use rug::{Integer, Float};
use std::collections::HashMap;

/// Record of data
#[derive(Debug, Clone)]
pub(crate) enum RecordData {
    Integer(Integer),
    Float(Float),
    String(String),
    Dict(Dictionary)
}

/// Helper functions for record data
impl RecordData {

    /// Update the value of the record data object to something else
    pub(crate) fn update_value(&mut self, other: RecordData) {
        *self = other.clone()
    }

    pub(crate) fn get_value(&self) -> RecordData {
        match &*self {
            RecordData::Integer(v) => RecordData::Integer(v.clone()),
            RecordData::Float(v)   => RecordData::Float(v.clone()),
            RecordData::String(v)  => RecordData::String(v.clone()),
            RecordData::Dict(v)    => RecordData::Dict(v.clone())
        }
    }
}

/// A dictionary of data
#[derive(Debug, Clone)]
pub(crate) struct Dictionary {
    data: HashMap<String, Rc<RefCell<RecordData>>>
}

impl Dictionary {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    /// Get a record
    pub(crate) fn get(&self, key: &String) -> Option<Rc<RefCell<RecordData>>> {
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

        self.data.insert(key.clone(), Rc::new(RefCell::new(value)));
    }
}

