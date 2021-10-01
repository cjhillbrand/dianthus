use crate::attributes::attribute_info::AttributeInfo;
use crate::constants::constant_info::ConstantInfo;
use crate::util::to_u16;
use crate::attributes::attribute_factory::get_attribute;

#[derive(Default)]
pub struct MethodInfo
{
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Box<dyn AttributeInfo>>
}

impl MethodInfo
{
    pub fn new(data: &[u8], constant_pool: &[Box<dyn ConstantInfo>]) -> MethodInfo
    {
        let mut iter = data.iter();
        let mut result: MethodInfo = Default::default();
        result.access_flags = to_u16(&mut iter).unwrap();
        result.name_index = to_u16(&mut iter).unwrap();
        result.descriptor_index = to_u16(&mut iter).unwrap();
        result.attributes_count = to_u16(&mut iter).unwrap();

        result.attributes = Vec::new();
        for _i in 0..result.attributes_count.clone()
        {
            result.attributes.push(get_attribute(&data, &constant_pool));
        }

        result
    }
}