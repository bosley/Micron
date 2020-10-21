
use rug::{ Integer, Float };

/// Precision of floats read into Micron
pub const FLOAT_PRECISION: u32 = 53;

#[derive(Debug)]
pub enum Statement {

    Assignment(String, Box<Expr>),
    BareExpression(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Integer),
    Real(Float),
    String(String),
    Variable(String),
    Modifier(String, Vec<String>),
    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Box<Expr>, UnaryOpcode),
    Access(Box<Expr>, Accessors, Box<MemberMethod>),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOpcode {
    Negate,
    BwNot
}

#[derive(Debug, Clone)]
pub enum Accessors {
    Dot,
}

#[derive(Debug, Clone)]
pub struct MemberMethod {
    pub method: String,
    pub params: Vec<Box<Expr>>
}
