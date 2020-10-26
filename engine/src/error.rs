
use derive_more::Display;

#[derive(Display)]
pub enum ExecutionError {

    StackError,
    UnknownVariable,

    #[display(fmt = "Unknown built in function {} ", _0)]
    UnknownBuiltInFunction(String),

    #[display(fmt = "Conversion failure arose from {} | {}", _0, _1)]
    ConversionFailure(String, String),


    #[display(fmt = "Unknown method '{}' attempted to be accessed by accessor '{}'", _1, _0)]
    UnknownVariableMethod(&'static str, String),


    #[display(fmt = "Invalid Operation: {}", _0)]
    InvalidOperation(&'static str),

    InvalidParameters,

    IndexError
}