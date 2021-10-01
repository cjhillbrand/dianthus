use crate::constants::constant_info::ConstantInfo;
use crate::constants::utf8_info::Utf8Info;
use crate::attributes::attribute_info::{AttributeInfo};
use crate::read_bytes::ReadBytes;
use crate::attributes::code_attribute::CodeAttribute;
use crate::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::attributes::signature_attribute::SignatureAttribute;
use crate::attributes::exception_attribute::ExceptionAttribute;
use std::collections::VecDeque;
use crate::vecdeque;

const CONSTANT_VALUE: &str = "ConstantValue";
const CODE: &str = "Code";
const DEPRECATED: &str = "Deprecated";
const SIGNATURE: &str = "Signature";
const EXCEPTION: &str = "Exception";

pub fn get_attribute(mut data: &mut VecDeque<u8>, constant_pool: &[Box<dyn ConstantInfo>]) -> Box<dyn AttributeInfo>
{
    let mut attr_index_vec = vecdeque![data[0].clone(), data[1].clone()];
    let attr_index: usize = attr_index_vec.pop_u16() as usize;
    let constant_info = &constant_pool[attr_index];
    let utf8_info: &Utf8Info = match constant_info.as_any().downcast_ref::<Utf8Info>() {
        Some(info) => info,
        None => panic!("The index into the constant pool isn't a utf8 into constant info"),
    };

    let attribute_type = utf8_info.get_string();
    match attribute_type
    {
        CONSTANT_VALUE =>   { Box::new(ConstantValueAttribute::new(&mut data)) },
        CODE =>             { Box::new(CodeAttribute::new(&mut data, &constant_pool)) },
        DEPRECATED =>       { Box::new(DeprecatedAttribute::new(&mut data)) },
        SIGNATURE =>        { Box::new(SignatureAttribute::new(&mut data)) },
        EXCEPTION =>        { Box::new(ExceptionAttribute::new(&mut data)) },
        &_ => panic!("Unidentified attribute")
    }
}