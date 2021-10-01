use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };
use std::any::Any;
use std::collections::VecDeque;
use crate::vecdeque;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodRefInfo
{
    tag: u8,
    class_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for MethodRefInfo
{
    fn tag(&self) -> &u8 { &self.tag }
    fn as_any(&self) -> &dyn Any { self }
}

impl MethodRefInfo
{
    pub fn new(mut data: &mut VecDeque<u8>) -> MethodRefInfo
    {
        MethodRefInfo
        {
            tag: to_u8(&mut data),
            class_index: to_u16(&mut data),
            name_and_type_index: to_u16(&mut data)
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::method_ref_info::MethodRefInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn method_ref_info_implements_equality_by_default()
    {
        let instance1: MethodRefInfo = Default::default();
        let instance2: MethodRefInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn method_ref_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
        let result: MethodRefInfo = MethodRefInfo::new(&mut data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit16, result.class_index);
        assert_eq!(bit16, result.name_and_type_index);
    }

    #[test]
    fn method_ref_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: MethodRefInfo = MethodRefInfo::new(&mut data);
        let instance2: MethodRefInfo = MethodRefInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn method_ref_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: MethodRefInfo = MethodRefInfo::new(&mut data1);
        let instance2: MethodRefInfo = MethodRefInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn method_ref_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: MethodRefInfo = MethodRefInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: MethodRefInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}