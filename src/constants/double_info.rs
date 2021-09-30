use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u64 };

#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DoubleInfo
{
    tag: u8,
    value: f64
}

impl ConstantInfo for DoubleInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl DoubleInfo
{
    pub fn new(data: &[u8]) -> DoubleInfo
    {
        let mut iter = data.iter();
        DoubleInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            value: {
                let bits = to_u64(&mut iter).unwrap();
                DoubleInfo::unsigned_to_float(&bits)
            }
        }
    }

    fn unsigned_to_float(bits: &u64) -> f64
    {
        let s: f64 = if (bits >> 63) == 0 { 1.0 } else { -1.0 };
        let e = ((bits >> 52) & 0x7ff) as i64;
        let m = if e == 0 { (bits & 0xfffffffffffff) << 1 }
            else { (bits & 0xfffffffffffff) | 0x10000000000000 };
        let base: f64 = 2.0;
        s * m as f64 * base.powf((e - 1075) as f64)
    }
}