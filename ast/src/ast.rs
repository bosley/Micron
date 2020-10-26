use rug::{ Integer, Float };

/// Precision of floats read into Micron
pub const FLOAT_PRECISION: u32 = 53;
pub const RADIX: i32 = 10;

#[derive(Debug, Clone)]
pub enum VariableType {
   Singular(String),
   Nested(String, Vec<DictAccessType>),
}

#[derive(Debug)]
pub enum Statement {

    Assignment(VariableType, Box<Expr>),
    BareExpression(Box<Expr>)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Integer),
    Real(Float),
    String(String),
    Variable(VariableType),

    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Box<Expr>, UnaryOpcode),


    BuiltInModifierCall(String, Vec<VariableType>),

    Access(Box<Expr>, Accessors, Box<MemberMethod>),

    // { .. }
    Dict(Vec<Box<DictEntry>>),
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
pub enum DictAccessType {
    RawValue(String),
    Variable(String)
}

#[derive(Debug, Clone)]
pub enum Accessors {
    Dot
}

#[derive(Debug, Clone)]
pub struct MemberMethod {
    pub method: String,
    pub params: Vec<Box<Expr>>
}

#[derive(Debug, Clone)]
pub struct DictEntry {
    pub key: String,
    pub value: Box<Expr>
}