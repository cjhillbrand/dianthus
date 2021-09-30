use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodTypeInfo
{
    tag: u8,
    descriptor_index: u16
}

impl ConstantInfo for MethodTypeInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl MethodTypeInfo
{
    pub fn new(data: &[u8]) -> MethodTypeInfo
    {
        let mut iter = data.iter();
        MethodTypeInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            descriptor_index: to_u16(&mut iter).unwrap()
        }
    }
}