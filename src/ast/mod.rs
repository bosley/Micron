use rug::{ Integer, Float };

/// Precision of floats in Micron
pub const FLOAT_PRECISION: u32 = 53;

#[derive(Debug)]
pub enum Statement {

    Assignment(String, Box<Expr>),
    BareExpression(Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Number(Integer),
    Real(Float),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Box<Expr>, UnaryOpcode)
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Lte,
    Gte,
    Gt,
    Lt,
    Equal,
    Ne,
    Pow,
    Mod,
    Lsh,
    Rsh,
    BwXor,
    BwOr,
    BwAnd,
    Or,
    And
}

#[derive(Debug)]
pub enum UnaryOpcode {
    Negate,
    BwNot
}