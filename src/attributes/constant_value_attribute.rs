use crate::attributes::attribute_info::AttributeInfo;
use crate::util::{to_u32, to_u16};
use std::any::Any;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ConstantValueAttribute
{
    attribute_name_index: u16,
    attribute_length: u32,
    constant_value_index: u16
}

impl AttributeInfo for ConstantValueAttribute
{
    fn name_index(&self) -> &u16 { &self.attribute_name_index }
    fn attr_length(&self) -> &u32 { &self.attribute_length }
    fn as_any(&self) -> &dyn Any { self }
}

impl ConstantValueAttribute
{
    pub fn new(data: &[u8]) -> ConstantValueAttribute
    {
        let mut iter = data.iter();
        ConstantValueAttribute
        {
            attribute_name_index: to_u16(&mut iter).unwrap(),
            attribute_length: to_u32(&mut iter).unwrap(),
            constant_value_index: to_u16(&mut iter).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::attributes::constant_value_attribute::ConstantValueAttribute;
    use serde_json::Result;

    #[test]
    fn constant_value_attribute_implements_equality_by_default()
    {
        let instance1: ConstantValueAttribute = Default::default();
        let instance2: ConstantValueAttribute = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn constant_value_attribute_constructs_expected_struct()
    {
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result: ConstantValueAttribute = ConstantValueAttribute::new(&data);

        let bit16: u16 = 257;
        let bit32: u32 = 16843009;
        assert_eq!(bit16, result.attribute_name_index);
        assert_eq!(bit32, result.attribute_length);
        assert_eq!(bit16, result.constant_value_index);
    }

    #[test]
    fn constant_value_attribute_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: ConstantValueAttribute = ConstantValueAttribute::new(&data);
        let instance2: ConstantValueAttribute = ConstantValueAttribute::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn constant_value_attribute_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let data2: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: ConstantValueAttribute = ConstantValueAttribute::new(&data1);
        let instance2: ConstantValueAttribute = ConstantValueAttribute::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn constant_value_attribute_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: ConstantValueAttribute = ConstantValueAttribute::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: ConstantValueAttribute = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}

