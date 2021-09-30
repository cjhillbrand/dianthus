use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StringInfo
{
    tag: u8,
    string_index: u16
}

impl ConstantInfo for StringInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl StringInfo
{
    pub fn new(data: &Vec<u8>) -> StringInfo
    {
        let mut iter = data.iter();
        StringInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            string_index: to_u16(&mut iter).unwrap()
        }
    }
}