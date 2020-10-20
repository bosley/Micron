use derive_more::Display;

#[derive(Display)]
pub enum EnvError {
    ConversionError(&'static str),

    #[display(fmt = "{} : {}", _0, _1)]
    IndexError(&'static str, usize),
    UnknownVariable(String),
    UnknownScope(String)
}