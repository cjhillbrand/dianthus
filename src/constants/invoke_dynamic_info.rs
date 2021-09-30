use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct InvokeDynamicInfo
{
    tag: u8,
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for InvokeDynamicInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl InvokeDynamicInfo
{
    pub fn new(data: &[u8]) -> InvokeDynamicInfo
    {
        let mut iter = data.iter();
        InvokeDynamicInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            bootstrap_method_attr_index: to_u16(&mut iter).unwrap(),
            name_and_type_index: to_u16(&mut iter).unwrap()
        }
    }
}