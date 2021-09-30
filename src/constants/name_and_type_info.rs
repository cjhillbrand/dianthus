use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct NameAndTypeInfo
{
    tag: u8,
    name_index: u16,
    descriptor_index: u16
}

impl ConstantInfo for NameAndTypeInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl NameAndTypeInfo
{
    pub fn new(data: &[u8]) -> NameAndTypeInfo
    {
        let mut iter = data.iter();
        NameAndTypeInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            name_index: to_u16(&mut iter).unwrap(),
            descriptor_index: to_u16(&mut iter).unwrap()
        }
    }
}