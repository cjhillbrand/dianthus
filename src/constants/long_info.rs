use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u64};
use std::any::Any;
use std::collections::VecDeque;
use crate::vecdeque;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct LongInfo
{
    tag: u8,
    value: i64
}

impl ConstantInfo for LongInfo
{
    fn tag(&self) -> &u8 { &self.tag }
    fn as_any(&self) -> &dyn Any { self }
}

impl LongInfo
{
    pub fn new(mut data: &mut VecDeque<u8>) -> LongInfo
    {
        LongInfo
        {
            tag: to_u8(&mut data),
            value: to_u64(&mut data) as i64
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::long_info::LongInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn long_info_implements_equality_by_default()
    {
        let instance1: LongInfo = Default::default();
        let instance2: LongInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn long_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 0, 0, 0, 0, 0, 1, 0, 1];
        let result: LongInfo = LongInfo::new(&mut data);

        let bit8: u8 = 1;
        let bit64: i64 = 65537;// 1152921504606846976;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit64, result.value);
    }

    #[test]
    fn long_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: LongInfo = LongInfo::new(&mut data);
        let instance2: LongInfo = LongInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn long_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut data2: VecDeque<u8> = vecdeque![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: LongInfo = LongInfo::new(&mut data1);
        let instance2: LongInfo = LongInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn long_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let instance1: LongInfo = LongInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: LongInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}