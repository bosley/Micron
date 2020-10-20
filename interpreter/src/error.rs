use micron_environment::EnvError;

use derive_more::Display;


#[derive(Display)]
pub enum InterpreterError {
    
    EnvironmentError(EnvError),
    StackError,
    InvalidUnaryOperation,
    InvalidExpression,
    InvalidStringExpression,
    InvalidAccessor
}
