use crate::attributes::attribute_info::AttributeInfo;
use crate::constants::constant_info::ConstantInfo;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;
use crate::read_bytes::ReadBytes;
use crate::constants::constant_factory::get_constant;
use crate::attributes::attribute_factory::get_attribute;
use std::collections::VecDeque;

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
    pub fn new(mut data: &mut VecDeque<u8>) -> ClassStruct
    {
        let mut result: ClassStruct = Default::default();
        result.magic = data.pop_u32();
        result.minor_version = data.pop_u16();
        result.major_version = data.pop_u16();
        result.constant_pool_count = data.pop_u16();
        result.constant_pool = Vec::new();
        for _i in 0..result.constant_pool_count.clone()
        {
            result.constant_pool.push(get_constant(&mut data));
        }

        result.access_flags = data.pop_u16();
        result.this_class = data.pop_u16();
        result.super_class = data.pop_u16();
        result.interfaces_count = data.pop_u16();
        result.interfaces = Vec::new();
        for _i in 0..result.interfaces_count.clone()
        {
            result.interfaces.push(data.pop_u16());
        }

        result.fields_count = data.pop_u16();
        result.field_info = Vec::new();
        for _i in 0..result.fields_count.clone()
        {
            result.field_info.push(FieldInfo::new(&mut data, &result.constant_pool));
        }

        result.methods_count = data.pop_u16();
        result.method_info = Vec::new();
        for _i in 0..result.methods_count.clone()
        {
            result.method_info.push(MethodInfo::new(&mut data, &result.constant_pool));
        }

        result.attributes_count = data.pop_u16();
        result.attribute_info = Vec::new();
        for _i in 0..result.attributes_count.clone()
        {
            result.attribute_info.push(get_attribute(&mut data, &result.constant_pool));
        }

        result
    }
}