
use std::{ cell::RefCell, rc::Rc };
use rug::{Integer, Float, Assign};
use std::collections::HashMap;

extern crate micron_ast;
use micron_ast::{ FLOAT_PRECISION, RADIX };

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

    pub(crate) fn to_string(&mut self) -> Option<RecordData> {
        match &*self {
            RecordData::Integer(v) => {
                Some(RecordData::String(String::from(v.to_string_radix(RADIX))))
            }

            RecordData::Float(v)   => {

                Some(RecordData::String(v.to_string_radix(RADIX, Some(v.prec() as usize))))
            }

            RecordData::String(v)  => {
                 Some(RecordData::String(v.clone()))
            }

            RecordData::Dict(v)    => {

                Some(RecordData::String(format!("{:?}", v)))
            }
        }
    }

    pub(crate) fn to_int(&mut self) -> Option<RecordData> {

        match &*self {
            RecordData::Integer(v) => {
                Some(RecordData::Integer(v.clone()))
            }

            RecordData::Float(v)   => {

                match v.to_integer() {
                    Some(v_int) => { Some(RecordData::Integer(v_int)) }
                    None => None 
                }
            }

            RecordData::String(v)  => {

                let i_val = v.parse::<i64>();

                if i_val.is_err() {
                    return None;
                }

                Some(RecordData::Integer(Integer::from(i_val.unwrap())))
            }

            RecordData::Dict(_)    => {

                None
            }
        }
    }

    pub(crate) fn to_float(&mut self) -> Option<RecordData> {

        match &*self {
            RecordData::Integer(v) => {

                let mut f_v = Float::new(FLOAT_PRECISION);
                f_v.assign(v);

                Some(RecordData::Float( f_v ))
            }

            RecordData::Float(v)   => {

                Some(RecordData::Float(v.clone()))
            }

            RecordData::String(v)  => {

                let i_val = v.parse::<i64>();

                if i_val.is_err() {
                    return None;
                }

                Some(RecordData::Integer(Integer::from(i_val.unwrap())))
            }

            RecordData::Dict(_)    => {

                None
            }
        }
    }

    pub(crate) fn set_precision(&mut self, precision: u32) -> Option<RecordData> {
        match &*self {
            RecordData::Float(v)   => {

                let mut f_v = Float::new(precision);
                f_v.assign(v);

                Some(RecordData::Float(v.clone()))
            }

            _ => {
                None
            }
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

