// 1. Life times should come up very infrequentyl
// 2. Writing good code = never thinking about the ownership system.

use crate::attributes::attribute_info::AttributeInfo;
use crate::constants::constant_info::ConstantInfo;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;
use crate::util::{to_u32, to_u16};
use crate::constants::constant_factory::get_constant;
use crate::attributes::attribute_factory::get_attribute;

#[derive(Default)]
struct ClassStruct {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<Box<dyn ConstantInfo>>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    field_info: Vec<FieldInfo>,
    methods_count: u16,
    method_info: Vec<MethodInfo>,
    attributes_count: u16,
    attribute_info: Vec<Box<dyn AttributeInfo>>
}

impl ClassStruct
{
    pub fn new(data: &[u8]) -> ClassStruct
    {
        let mut iter = data.iter();
        let mut result: ClassStruct = Default::default();
        result.magic = to_u32(&mut iter).unwrap();
        result.minor_version = to_u16(&mut iter).unwrap();
        result.major_version = to_u16(&mut iter).unwrap();
        result.constant_pool_count = to_u16(&mut iter).unwrap();
        result.constant_pool = Vec::new();
        for _i in 0..result.constant_pool_count.clone()
        {
            result.constant_pool.push(get_constant(&data));
        }

        result.access_flags = to_u16(&mut iter).unwrap();
        result.this_class = to_u16(&mut iter).unwrap();
        result.super_class = to_u16(&mut iter).unwrap();
        result.interfaces_count = to_u16(&mut iter).unwrap();
        result.interfaces = Vec::new();
        for _i in 0..result.interfaces_count.clone()
        {
            result.interfaces.push(to_u16(&mut iter).unwrap());
        }

        result.fields_count = to_u16(&mut iter).unwrap();
        result.field_info = Vec::new();
        for _i in 0..result.fields_count.clone()
        {
            result.field_info.push(FieldInfo::new(&data, &result.constant_pool));
        }

        result.methods_count = to_u16(&mut iter).unwrap();
        result.method_info = Veec::new();
        for _i in 0..result.methods_count.clone()
        {
            result.method_info.push(MethodInfo::new(&data, &result.constant_pool));
        }

        result.attributes_count = to_u16(&mut iter).unwrap();
        result.attribute_info = Vec::new();
        for _i in 0..result.attributes_count.clone()
        {
            result.attribute_info(get_attribute(&data, &result.constant_pool));
        }

        result
    }
}