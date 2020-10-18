
use crate::ast::{ Opcode, UnaryOpcode };
use crate::env::Object;

#[derive(Debug)]
pub enum EvalError {

    StackError,
    InvalidExpression(Object, Opcode, Object),
    InvalidUnaryExpression(Object, UnaryOpcode),
    UnknownVariable(String)
}