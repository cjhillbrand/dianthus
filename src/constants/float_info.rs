use crate::constants::constant_info::ConstantInfo;
use crate::read_bytes::ReadBytes;

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

impl Eq for FloatInfo { }

impl FloatInfo
{
    pub fn new<T: ReadBytes>(data: &mut T) -> FloatInfo
    {
        FloatInfo
        {
            tag: data.pop_u8(),
            value:  {
                let bits = data.pop_u32();
                FloatInfo::unsigned_to_float(&bits)
            }
        }
    }

    fn unsigned_to_float(bits: &u32) -> f32
    {
        let s: f32 = if bits & 0x80000000 == 0 { 1. } else { -1. };
        let e: i32 = ((bits >> 23) & 0x000000ff) as i32; // & gets rid of leading sign bit
        let m = if e == 0 { (bits & 0x7fffff) << 1 } else { (bits & 0x7fffff) | 0x800000 };
        let base: f32 = 2.0;
        s * m as f32 * base.powf((e - 150) as f32)
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::float_info::FloatInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn float_info_implements_equality_by_default()
    {
        let instance1: FloatInfo = Default::default();
        let instance2: FloatInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn float_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = get_default_veq();
        let result: FloatInfo = FloatInfo::new(&mut data);

        let bit8: u8 = 1;
        assert_eq!(bit8, result.tag);
        assert_eq!(123.45, result.value, f32::EPSILON);
    }

    #[test]
    fn float_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = get_default_veq();
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: FloatInfo = FloatInfo::new(&mut data);
        let instance2: FloatInfo = FloatInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn float_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = get_default_veq();
        let mut data2: VecDeque<u8> = data1.clone();
        data2[0] = data1[0] + 1;
        let instance1: FloatInfo = FloatInfo::new(&mut data1);
        let instance2: FloatInfo = FloatInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn float_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = get_default_veq();
        let instance1: FloatInfo = FloatInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: FloatInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }

    fn get_default_veq() -> VecDeque<u8>
    {
        vecdeque![1, 66, 246, 230, 102] // 123.45
    }
}
