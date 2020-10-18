use rug::{ Integer, Float };

#[derive(Clone, Debug)]
pub enum Object {
    Integer(Integer),
    Float(Float)
}