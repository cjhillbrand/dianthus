use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u32};
use crate::attributes::attribute_info::AttributeInfo;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct IntegerInfo
{
    tag: u8,
    value: i32
}

impl ConstantInfo for IntegerInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl IntegerInfo
{
    pub fn new(data: &[u8]) -> IntegerInfo
    {
        let mut iter = data.iter();
        IntegerInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            value: to_u32(&mut iter).unwrap() as i32
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::integer_info::IntegerInfo;

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
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result: IntegerInfo = IntegerInfo::new(&data);

        let bit8: u8 = 1;
        let bit32: i32 = 16843009;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit32, result.value);
    }

    #[test]
    fn class_info_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: IntegerInfo = IntegerInfo::new(&data);
        let instance2: IntegerInfo = IntegerInfo::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn class_info_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let data2: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: IntegerInfo = IntegerInfo::new(&data1);
        let instance2: IntegerInfo = IntegerInfo::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn class_info_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: IntegerInfo = IntegerInfo::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: IntegerInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}