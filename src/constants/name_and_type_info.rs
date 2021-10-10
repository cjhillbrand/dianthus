use crate::constants::constant_info::ConstantInfo;
use crate::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct NameAndTypeInfo
{
    tag: u8,
    name_index: u16,
    descriptor_index: u16
}

impl ConstantInfo for NameAndTypeInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl NameAndTypeInfo
{
    pub fn new<T: ReadBytes>(data: &mut T) -> NameAndTypeInfo
    {
        NameAndTypeInfo
        {
            tag: data.pop_u8(),
            name_index: data.pop_u16(),
            descriptor_index: data.pop_u16()
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::name_and_type_info::NameAndTypeInfo;
    use std::collections::VecDeque;
    use crate::vecdeque;

    #[test]
    fn name_and_type_info_implements_equality_by_default()
    {
        let instance1: NameAndTypeInfo = Default::default();
        let instance2: NameAndTypeInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn name_and_type_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
        let result: NameAndTypeInfo = NameAndTypeInfo::new(&mut data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit16, result.name_index);
        assert_eq!(bit16, result.descriptor_index);
    }

    #[test]
    fn name_and_type_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: NameAndTypeInfo = NameAndTypeInfo::new(&mut data);
        let instance2: NameAndTypeInfo = NameAndTypeInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn name_and_type_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: NameAndTypeInfo = NameAndTypeInfo::new(&mut data1);
        let instance2: NameAndTypeInfo = NameAndTypeInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn name_and_type_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: NameAndTypeInfo = NameAndTypeInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: NameAndTypeInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}