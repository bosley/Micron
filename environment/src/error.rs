use derive_more::Display;

#[derive(Display)]
pub enum EnvError {

    #[display(fmt = "Failed to convert : {}", _0)]
    ConversionError(&'static str),

    #[display(fmt = "{} : {}", _0, _1)]
    IndexError(&'static str, usize),

    #[display(fmt = "Unknown variable : {}", _0)]
    UnknownVariable(String),

    #[display(fmt = "Unknown scope : {}", _0)]
    UnknownScope(String),

    #[display(fmt = "Failed to convert object : {}", _0)]
    ObjectConversionError(&'static str),

    #[display(fmt = "Unknown Method : {}", _0)]
    UnknownMethod(String),

    #[display(fmt = "Invalid Number of parameters : Expected : {}, got : {}", _0, _1)]
    InvalidNumberOfParameters(i32, i32),

    #[display(fmt = "Invalid Parameter: {}", _0)]
    InvalidParameter(&'static str),

    #[display(fmt = "Type {} has no method named {} associated with it", _0, _1)]
    NoMethodForType(&'static str, &'static str),

    #[display(fmt = "Dictionary does not contain key {}", _0)]
    UnknownKeyForDict(String),

    #[display(fmt = "Incorrect type : {}", _0)]
    IncorrectType(&'static str),
}