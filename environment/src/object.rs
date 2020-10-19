
use crate::types::{MInteger, MFloat, MString, MDict, DictItem, FromRug};

#[derive(Debug, Clone)]
pub enum Object {
    Integer(MInteger),
    Float(MFloat),
    String(MString),
    Dict(MDict)
}

pub fn get_object_from_dict_item(item: DictItem) -> Object {

    match item {
        DictItem::DictInteger(i) => {
            Object::Integer(MInteger::from_rug_integer(i.get_value()))
        }

        DictItem::DictFloat(f) => {
            Object::Float(MFloat::from_rug_float(f.get_value()))
        }

        DictItem::DictString(s) => {
            Object::String(MString::new(s.get_value()))
        }
    }
}