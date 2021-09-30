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

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::double_info::DoubleInfo;

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
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let result: DoubleInfo = DoubleInfo::new(&data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        // todo! this needs to be fixed.
        // assert_eq!(bit16, result.value);
    }

    #[test]
    fn double_info_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instance1: DoubleInfo = DoubleInfo::new(&data);
        let instance2: DoubleInfo = DoubleInfo::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn double_info_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let data2: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: DoubleInfo = DoubleInfo::new(&data1);
        let instance2: DoubleInfo = DoubleInfo::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn double_info_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instance1: DoubleInfo = DoubleInfo::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: DoubleInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}