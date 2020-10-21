
use crate::types::{MInteger, MFloat, MString, MDict, DictItem};

#[derive(Debug, Clone)]
pub enum Object {
    Integer(MInteger),
    Float(MFloat),
    String(MString),
    Dict(MDict)
}

pub fn object_to_dict_item(obj: Object) -> DictItem {
    match obj {
        Object::Integer(i) => DictItem::DictInteger(i),
        Object::Float(i)   => DictItem::DictFloat(i),
        Object::String(i)  => DictItem::DictString(i),
        Object::Dict(i)    => DictItem::DictDict(i)
    }
}

pub fn dict_item_to_object(di: DictItem) -> Object {
    match di {
        DictItem::DictInteger(i) => Object::Integer(i),
        DictItem::DictFloat(i)   => Object::Float(i),
        DictItem::DictString(i)  => Object::String(i),
        DictItem::DictDict(i)    => Object::Dict(i),
    }
}