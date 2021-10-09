use crate::constants::constant_info::ConstantInfo;
use crate::read_bytes::ReadBytes;
use crate::attributes::attribute_factory::{get_attribute_container};
use serde_json::de::Read;
use crate::attributes::attribute_container::AttributeContainer;
use crate::constants::constant_container::ConstantContainer;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodInfo
{
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeContainer>
}

impl MethodInfo
{
    pub fn new<T: ReadBytes>(mut data: &mut T, constant_pool: &[ConstantContainer]) -> MethodInfo
    {
        let mut result: MethodInfo = Default::default();
        result.access_flags = data.pop_u16();
        result.name_index = data.pop_u16();
        result.descriptor_index = data.pop_u16();
        result.attributes_count = data.pop_u16();

        result.attributes = Vec::new();
        for _i in 0..result.attributes_count.clone()
        {
            result.attributes.push(get_attribute_container(data, &constant_pool));
        }

        result
    }
}