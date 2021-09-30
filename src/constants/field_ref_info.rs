use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct FieldRefInfo
{
    tag: u8,
    class_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for FieldRefInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl FieldRefInfo
{
    pub fn new(data: &Vec<u8>) -> FieldRefInfo
    {
        let mut iter = data.iter();
        FieldRefInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            class_index: to_u16(&mut iter).unwrap(),
            name_and_type_index: to_u16(&mut iter).unwrap()
        }
    }
}