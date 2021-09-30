use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u64};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct LongInfo
{
    tag: u8,
    value: i64
}

impl ConstantInfo for LongInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl LongInfo
{
    pub fn new(data: &[u8]) -> LongInfo
    {
        let mut iter = data.iter();
        LongInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            value: to_u64(&mut iter).unwrap() as i64
        }
    }
}