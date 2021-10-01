// sorry rust folks

use crate::constants::constant_info::ConstantInfo;
use crate::constants::utf8_info::Utf8Info;
use crate::attributes::attribute_info::{AttributeInfo};
use crate::util::to_u16;
use crate::attributes::code_attribute::CodeAttribute;

pub fn get_attribute(data: &[u8], constant_pool: &[Box<dyn ConstantInfo>]) -> Box<dyn AttributeInfo>
{
    let attr_index_vec = vec![data[0].clone(), data[1].clone()];
    let mut iter = attr_index_vec.iter();
    let attr_index: usize = to_u16(&mut iter).unwrap() as usize;
    let constant_info = &constant_pool[attr_index];
    let utf8_info: &Utf8Info = match constant_info.as_any().downcast_ref::<Utf8Info>() {
        Some(info) => info,
        None => panic!("The index into the constant pool isn't a utf8 into constant info"),
    };

    let attribute_type = utf8_info.get_string();
    match attribute_type
    {
        &_ => { Box::new(CodeAttribute::new(&data)) }
    }
}