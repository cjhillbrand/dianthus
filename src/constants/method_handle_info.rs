use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodHandleInfo
{
    tag: u8,
    reference_kind: u8,
    reference_index: u16
}

impl ConstantInfo for MethodHandleInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl MethodHandleInfo
{
    pub fn new(data: &[u8]) -> MethodHandleInfo
    {
        let mut iter = data.iter();
        MethodHandleInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            reference_kind: to_u8(&mut iter).unwrap(),
            reference_index: to_u16(&mut iter).unwrap()
        }
    }
}