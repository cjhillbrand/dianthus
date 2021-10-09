use crate::constants::constant_info::ConstantInfo;
use crate::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct InvokeDynamicInfo
{
    tag: u8,
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for InvokeDynamicInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl InvokeDynamicInfo
{
    pub fn new<T: ReadBytes>(mut data: &mut T) -> InvokeDynamicInfo
    {
        InvokeDynamicInfo
        {
            tag: data.pop_u8(),
            bootstrap_method_attr_index: data.pop_u16(),
            name_and_type_index: data.pop_u16()
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::invoke_dynamic_info::InvokeDynamicInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn invoke_dynamic_info_implements_equality_by_default()
    {
        let instance1: InvokeDynamicInfo = Default::default();
        let instance2: InvokeDynamicInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn invoke_dynamic_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
        let result: InvokeDynamicInfo = InvokeDynamicInfo::new(&mut data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit16, result.bootstrap_method_attr_index);
        assert_eq!(bit16, result.name_and_type_index);
    }

    #[test]
    fn invoke_dynamic_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: InvokeDynamicInfo = InvokeDynamicInfo::new(&mut data);
        let instance2: InvokeDynamicInfo = InvokeDynamicInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn invoke_dynamic_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: InvokeDynamicInfo = InvokeDynamicInfo::new(&mut data1);
        let instance2: InvokeDynamicInfo = InvokeDynamicInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn invoke_dynamic_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: InvokeDynamicInfo = InvokeDynamicInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: InvokeDynamicInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}