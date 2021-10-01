use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16, to_u32};
use std::any::Any;

#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct FloatInfo
{
    tag: u8,
    value: f32
}

impl ConstantInfo for FloatInfo
{
    fn tag(&self) -> &u8 { &self.tag }
    fn as_any(&self) -> &dyn Any { self }
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
        let e: u32 = (bits >> 23) & 0xff;
        let m = if e == 0 { (bits & 0x7fffff) << 1 } else { (bits & 0x7fffff) | 0x800000 };
        let base: f32 = 2.0;
        s * m as f32 * base.powf((e - 150) as f32)
    }
}

// Figure out what each bit means in the float and then revisit.
// #[cfg(test)]
// mod tests
// {
//     use serde_json::Result;
//     use crate::constants::float_info::FloatInfo;
//
//     #[test]
//     fn float_info_implements_equality_by_default()
//     {
//         let instance1: FloatInfo = Default::default();
//         let instance2: FloatInfo = Default::default();
//
//         assert_eq!(instance1, instance2);
//     }
//
//     #[test]
//     fn float_info_constructs_expected_struct()
//     {
//         let data: Vec<u8> = vec![1, 0, 0, 0, 1];
//         let result: FloatInfo = FloatInfo::new(&data);
//
//         let bit8: u8 = 1;
//         let bit16: u16 = 257;
//         assert_eq!(bit8, result.tag);
//         // todo! make sure this is implemented correctly.
//         //assert_eq!(bit16, result.name_index);
//     }
//
//     #[test]
//     fn float_info_implements_equality_correctly()
//     {
//         let data: Vec<u8> = vec![1, 0, 0, 0, 1];
//         let instance1: FloatInfo = FloatInfo::new(&data);
//         let instance2: FloatInfo = FloatInfo::new(&data);
//
//         assert_eq!(instance1, instance2);
//     }
//
//     #[test]
//     fn float_info_implements_equality_correctly_when_not_equal()
//     {
//         let data1: Vec<u8> = vec![1, 0, 0, 0, 1];
//         let data2: Vec<u8> = vec![1, 0, 0, 1, 0];
//         let instance1: FloatInfo = FloatInfo::new(&data1);
//         let instance2: FloatInfo = FloatInfo::new(&data2);
//
//         assert_ne!(instance1, instance2);
//     }
//
//     #[test]
//     fn float_info_implements_json_serialization_correctly() -> Result<()>
//     {
//         let data: Vec<u8> = vec![1, 0, 0, 0, 1];
//         let instance1: FloatInfo = FloatInfo::new(&data);
//         let instance2 = instance1.clone();
//
//         let json = serde_json::to_string_pretty(&instance1)?;
//         let instance3: FloatInfo = serde_json::from_str(&json)?;
//
//         assert_eq!(instance2, instance3);
//         Ok(())
//     }
// }
