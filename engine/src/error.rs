
use derive_more::Display;

#[derive(Display)]
pub enum ExecutionError {

    StackError,
    UnknownVariable
}