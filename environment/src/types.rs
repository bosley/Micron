
use std::collections::HashMap;

use rug::{Integer, Float, Assign};

use crate::error::EnvError;

extern crate micron_ast;
use micron_ast::FLOAT_PRECISION;

/// RADIX
pub const STANDARD_RADIX:  i32 = 10;

/// Default precision for floats converted into strings
pub const DEFAULT_STRING_PRECISION: usize= 5;

/// Trait to cast data type from a type
pub trait FromRug {
    fn from_rug_integer(value: Integer) -> Self;
    fn from_rug_float(value: Float) -> Self;
}

/// Trait to cast any micron type to any other micron type
pub trait AsMicronType {
    fn as_micron_integer(&self) -> Result<MInteger, EnvError>;
    fn as_micron_float(&self) -> Result<MFloat, EnvError>;
    fn as_micron_string(&self) -> Result<MString, EnvError>;
}

/// Micron representation of an integer
#[derive(Debug, Clone, PartialEq)]
pub struct MInteger {

    value: Integer
}

/// Micron representation of a float
#[derive(Debug, Clone, PartialEq)]
pub struct MFloat {

    value: Float,
    precision: usize
}

/// Micron representation of a string
#[derive(Debug, Clone, PartialEq)]
pub struct MString {

    value: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum DictItem {
    DictInteger(MInteger),
    DictFloat(MFloat),
    DictString(MString)

    // DictFunc(statements) ???
}

/// Dictionary type
#[derive(Debug, Clone, PartialEq)]
pub struct MDict {

    items: HashMap<String, DictItem>
}

// ----------------------------------------------------------------------
//
//                   Micron Integer Implementations
//
// ----------------------------------------------------------------------

impl MInteger {
    pub fn new(value: i64) -> Self {
        Self {
            value: Integer::from(value)
        }
    }

    pub fn get_value(&self) -> Integer {
        self.value.clone()
    }
}

impl FromRug for MInteger {

    fn from_rug_integer(value: Integer) -> Self {

        Self{
            value: value
        }
    }

    fn from_rug_float(value: Float) -> Self {

        Self{
            value: value.to_integer().unwrap_or(Integer::from(0))
        }
    }
}

impl AsMicronType for MInteger {
    
    fn as_micron_integer(&self) -> Result<MInteger, EnvError>
    {
        Ok(MInteger::from_rug_integer(self.value.clone()))
    }

    fn as_micron_float(&self) -> Result<MFloat, EnvError>
    {
        Ok(MFloat::new(self.value.to_f64()))
    }

    fn as_micron_string(&self) -> Result<MString, EnvError>
    {
        let value = self.value.to_string_radix(STANDARD_RADIX);
        Ok(MString::new(value))
    }
}

// ----------------------------------------------------------------------
//
//                   Micron Float Implementations
//
// ----------------------------------------------------------------------


impl MFloat {
    pub fn new(value: f64) -> Self {
        let mut v = Float::new(FLOAT_PRECISION);
        v.assign(value);

        Self {
            value: v,
            precision: FLOAT_PRECISION as usize
        }
    }

    pub fn get_precesion(&self) -> usize {
        self.precision
    }

    pub fn get_value(&self) -> Float {
        self.value.clone()
    }
}

impl FromRug for MFloat {

    fn from_rug_integer(value: Integer) -> Self {

        Self{
            value: Float::with_val(FLOAT_PRECISION, value.to_f64()),
            precision: FLOAT_PRECISION as usize
        }
    }

    fn from_rug_float(value: Float) -> Self {

        Self{
            value: value,
            precision: FLOAT_PRECISION as usize
        }
    }
}

impl AsMicronType for MFloat {
    
    fn as_micron_integer(&self) -> Result<MInteger, EnvError>
    {
        Ok(MInteger::from_rug_float(self.value.clone()))
    }

    fn as_micron_float(&self) -> Result<MFloat, EnvError>
    {
        Ok(MFloat::from_rug_float(self.value.clone()))
    }

    fn as_micron_string(&self) -> Result<MString, EnvError>
    {
        let s = self.value.to_string_radix(STANDARD_RADIX, Some(self.precision));
        Ok(MString::new(s))
    }
}

// ----------------------------------------------------------------------
//
//                   Micron String Implementations
//
// ----------------------------------------------------------------------

impl MString {
    pub fn new(value: String) -> Self {
        Self {
            value: value.clone()
        }
    }

    pub fn get_index(&self, index: usize) -> Result<String, EnvError> {
        let mut chars = self.value.chars();
        match chars.nth(index) {
            Some(v) => {
                Ok(String::from(v))
            }
            None => {
                Err(EnvError::IndexError("Unable to retrieve index", index))
            }
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl FromRug for MString {

    fn from_rug_integer(value: Integer) -> Self {

        Self{
            value: value.to_string_radix(STANDARD_RADIX)
        }
    }

    fn from_rug_float(value: Float) -> Self {

        Self{
            value: value.to_string_radix(STANDARD_RADIX, Some(DEFAULT_STRING_PRECISION))
        }
    }
}

impl AsMicronType for MString {
    
    fn as_micron_integer(&self) -> Result<MInteger, EnvError>
    {
        let i_val = self.value.parse::<i64>();

        if i_val.is_err() {
            return Err(EnvError::ConversionError("Failed to convert value to integer type"));
        }

        return Ok(MInteger::new(i_val.unwrap()));
    }

    fn as_micron_float(&self) -> Result<MFloat, EnvError>
    {
        let f_val = self.value.parse::<f64>();

        if f_val.is_err() {
            return Err(EnvError::ConversionError("Failed to convert value to float type"));
        }

        return Ok(MFloat::new(f_val.unwrap()));
    }

    fn as_micron_string(&self) -> Result<MString, EnvError>
    {
        Ok(MString::new(self.value.clone()))
    }
}

// ----------------------------------------------------------------------
//
//                   Micron Dict Implementations
//
// ----------------------------------------------------------------------

impl MDict {
    pub fn new() -> Self {
        Self {
            items: HashMap::new()
        }
    }

    pub fn set_item(&mut self, key: MString, item: DictItem) {

        self.items.insert(key.get_value(), item);
    }

    pub fn get_item(&mut self, key: MString) -> Option<DictItem> {

        match self.items.get(&key.get_value()) {
            Some(v) => { return Some(v.clone()); }
            None    => { return None; }
        }
    }
}