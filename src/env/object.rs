use crate::ast;

use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer(i64)
}