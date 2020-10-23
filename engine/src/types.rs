
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

