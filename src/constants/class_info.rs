use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo
{
    tag: u8,
    name_index: u16
}

impl ConstantInfo for ClassInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl ClassInfo
{
    pub fn new(data: &Vec<u8>) -> ClassInfo
    {
        let mut iter = data.iter();
        ClassInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            name_index: to_u16(&mut iter).unwrap()
        }
    }
}