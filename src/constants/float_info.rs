use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16, to_u32};

#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct FloatInfo
{
    tag: u8,
    value: f32
}

impl ConstantInfo for FloatInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl FloatInfo
{
    pub fn new(data: &[u8]) -> FloatInfo
    {
        let mut iter = data.iter();
        FloatInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            value:  {
                let bits = to_u32(&mut iter).unwrap();
                FloatInfo::unsigned_to_float(&bits)
            }
        }
    }

    fn unsigned_to_float(bits: &u32) -> f32
    {
        let s: f32 = if bits & 0x80000000 == 0 { 1. } else { -1. };
        let e = (bits >> 23) & 0xff;
        let m = if e == 0 { (bits & 0x7fffff) << 1 } else { (bits & 0x7fffff) | 0x800000 };
        let base: f32 = 2.0;
        s * m as f32 * base.powf((e - 150) as f32)
    }
}
