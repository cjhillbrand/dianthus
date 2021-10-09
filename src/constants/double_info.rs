use crate::constants::constant_info::ConstantInfo;
use crate::read_bytes::ReadBytes;

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

impl Eq for DoubleInfo { }

impl DoubleInfo
{
    pub fn new<T: ReadBytes>(mut data: &mut T) -> DoubleInfo
    {
        DoubleInfo
        {
            tag: data.pop_u8(),
            value: {
                let bits: u64 = data.pop_u64();
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

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::double_info::DoubleInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn double_info_implements_equality_by_default()
    {
        let instance1: DoubleInfo = Default::default();
        let instance2: DoubleInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn double_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let result: DoubleInfo = DoubleInfo::new(&mut data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        // todo! this needs to be fixed.
        // assert_eq!(bit16, result.value);
    }

    #[test]
    fn double_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: DoubleInfo = DoubleInfo::new(&mut data);
        let instance2: DoubleInfo = DoubleInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn double_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut data2: VecDeque<u8> = vecdeque![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: DoubleInfo = DoubleInfo::new(&mut data1);
        let instance2: DoubleInfo = DoubleInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn double_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instance1: DoubleInfo = DoubleInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: DoubleInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}