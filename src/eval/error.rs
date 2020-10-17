
use crate::ast::Opcode;
use crate::env::Object;

#[derive(Debug)]
pub enum EvalError {

    StackError,
    InvalidExpression(Object, Opcode, Object)
}