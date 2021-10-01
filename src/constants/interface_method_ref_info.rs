use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };
use std::any::Any;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct InterfaceMethodRefInfo
{
    tag: u8,
    class_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for InterfaceMethodRefInfo
{
    fn tag(&self) -> &u8 { &self.tag }
    fn as_any(&self) -> &dyn Any { self }
}

impl InterfaceMethodRefInfo
{
    pub fn new(data: &[u8]) -> InterfaceMethodRefInfo
    {
        let mut iter = data.iter();
        InterfaceMethodRefInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            class_index: to_u16(&mut iter).unwrap(),
            name_and_type_index: to_u16(&mut iter).unwrap()
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::interface_method_ref_info::InterfaceMethodRefInfo;

    #[test]
    fn interface_method_ref_implements_equality_by_default()
    {
        let instance1: InterfaceMethodRefInfo = Default::default();
        let instance2: InterfaceMethodRefInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn interface_method_ref_constructs_expected_struct()
    {
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result: InterfaceMethodRefInfo = InterfaceMethodRefInfo::new(&data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit16, result.class_index);
        assert_eq!(bit16, result.name_and_type_index);
    }

    #[test]
    fn interface_method_ref_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: InterfaceMethodRefInfo = InterfaceMethodRefInfo::new(&data);
        let instance2: InterfaceMethodRefInfo = InterfaceMethodRefInfo::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn interface_method_ref_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let data2: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: InterfaceMethodRefInfo = InterfaceMethodRefInfo::new(&data1);
        let instance2: InterfaceMethodRefInfo = InterfaceMethodRefInfo::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn interface_method_ref_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: InterfaceMethodRefInfo = InterfaceMethodRefInfo::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: InterfaceMethodRefInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}