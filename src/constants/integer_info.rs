use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u32};
use crate::attributes::attribute_info::AttributeInfo;
use std::any::Any;
use std::collections::VecDeque;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct IntegerInfo
{
    tag: u8,
    value: i32
}

impl ConstantInfo for IntegerInfo
{
    fn tag(&self) -> &u8 { &self.tag }
    fn as_any(&self) -> &dyn Any { self }
}

impl IntegerInfo
{
    pub fn new(mut data: &mut VecDeque<u8>) -> IntegerInfo
    {
        IntegerInfo
        {
            tag: to_u8(&mut data),
            value: to_u32(&mut data) as i32
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::integer_info::IntegerInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn class_info_implements_equality_by_default()
    {
        let instance1: IntegerInfo = Default::default();
        let instance2: IntegerInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn class_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
        let result: IntegerInfo = IntegerInfo::new(&mut data);

        let bit8: u8 = 1;
        let bit32: i32 = 16843009;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit32, result.value);
    }

    #[test]
    fn class_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: IntegerInfo = IntegerInfo::new(&mut data);
        let instance2: IntegerInfo = IntegerInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn class_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: IntegerInfo = IntegerInfo::new(&mut data1);
        let instance2: IntegerInfo = IntegerInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn class_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: IntegerInfo = IntegerInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: IntegerInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}