
use crate::types::{MInteger, MFloat, MString, MDict,AsMicronType, DictItem, FromRug};
use crate::object::Object;
use crate::error::EnvError;

/// Convert an object to a string (built in function call)
pub fn as_string(item: Object) -> Result<Object, EnvError> {

    match item {
        Object::String(s) => {
            return Ok(Object::String(s))
        }
        Object::Integer(i) => {

            match i.as_micron_string() {
                Ok(s) => {
                    return Ok(Object::String(s));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Object::Float(f) => {

            match f.as_micron_string() {
                Ok(s) => {
                    return Ok(Object::String(s));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        _ => {
            return Err(EnvError::ObjectConversionError("Can not convert item to string"))
        }
    }
}

/// Convert an object to an integer
pub fn as_int(item: Object) -> Result<Object, EnvError> {

    match item {
        Object::String(s) => {
            match s.as_micron_integer() {
                Ok(s) => {
                    return Ok(Object::Integer(s));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Object::Integer(i) => {
            return Ok(Object::Integer(i))
        }
        Object::Float(f) => {
            match f.as_micron_integer() {
                Ok(s) => {
                    return Ok(Object::Integer(s));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        _ => {
            return Err(EnvError::ObjectConversionError("Can not convert item to integer"))
        }
    }
}

/// Convert an object to an float
pub fn as_float(item: Object) -> Result<Object, EnvError> {
    match item {
        Object::String(s) => {
            match s.as_micron_float() {
                Ok(s) => {
                    return Ok(Object::Float(s));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Object::Integer(i) => {
            match i.as_micron_float() {
                Ok(s) => {
                    return Ok(Object::Float(s));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Object::Float(f) => {
            return Ok(Object::Float(f))
        }
        _ => {
            return Err(EnvError::ObjectConversionError("Can not convert item to float"))
        }
    }
}

/// Set the precision of a suspected float
pub fn with_precision(item: Object, precision: MInteger)-> Result<Object, EnvError> {
    
    let converted_prec = precision.get_value().to_i64();

    if converted_prec.is_none() {
        return Err(EnvError::ObjectConversionError("Precision for float"));
    }

    match item {
        Object::Float(f) => {
            let mut n = MFloat::from_rug_float(f.get_value());
            n.set_precision(converted_prec.unwrap() as usize);
            return Ok(Object::Float(n));
        }
        Object::String(_) => {
            return Err(EnvError::NoMethodForType("String", "set_precision"));
        }
        Object::Integer(_) => {
            return Err(EnvError::NoMethodForType("Integer", "set_precision"));
        }
        Object::Dict(_) => {
            return Err(EnvError::NoMethodForType("Dict", "set_precision"));

        }
    }
}