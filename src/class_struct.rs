// 1. Life times should come up very infrequentyl
// 2. Writing good code = never thinking about the ownership system.

use crate::attributes::attribute_info::AttributeInfo;
use crate::constants::constant_info::ConstantInfo;
use crate::field_info::FieldInfo;
use crate::method_info::MethodInfo;

#[warn(dead_code)]
struct ClassStruct {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    cp_info: Vec<Box<dyn ConstantInfo>>,
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