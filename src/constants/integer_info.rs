use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u32};
use crate::attributes::attribute_info::AttributeInfo;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct IntegerInfo
{
    tag: u8,
    value: i32
}

impl ConstantInfo for IntegerInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl IntegerInfo
{
    pub fn new(data: &[u8]) -> IntegerInfo
    {
        let mut iter = data.iter();
        IntegerInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            value: to_u32(&mut iter).unwrap() as i32
        }
    }
}