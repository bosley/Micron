use rug::Integer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    Integer(Integer)
}