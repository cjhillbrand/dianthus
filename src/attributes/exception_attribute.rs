use crate::util::{to_u16, to_u32};
use crate::attributes::attribute_info::AttributeInfo;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ExceptionAttribute
{
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_exceptions: u16,
    exception_index_table: u16
}

impl AttributeInfo for ExceptionAttribute
{
    fn name_index(&self) -> &u16 { &self.attribute_name_index }
    fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl ExceptionAttribute
{
    pub fn new(data: &Vec<u8>) -> ExceptionAttribute
    {
        let mut iter = data.iter();
        ExceptionAttribute
        {
            attribute_name_index: to_u16(&mut iter).unwrap(),
            attribute_length: to_u32(&mut iter).unwrap(),
            number_of_exceptions: to_u16(&mut iter).unwrap(),
            exception_index_table: to_u16(&mut iter).unwrap()
        }
    }
}


#[cfg(test)]
mod tests
{
    use crate::attributes::exception_attribute::ExceptionAttribute;
    use serde_json::Result;

    #[test]
    fn exception_attribute_implements_equality_by_default()
    {
        let instance1: ExceptionAttribute = Default::default();
        let instance2: ExceptionAttribute = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn exception_attribute_constructs_expected_struct()
    {
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let result: ExceptionAttribute = ExceptionAttribute::new(&data);

        let bit16: u16 = 257;
        let bit32: u32 = 16843009;
        assert_eq!(bit16, result.attribute_name_index);
        assert_eq!(bit32, result.attribute_length);
    }

    #[test]
    fn exception_attribute_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let instance1: ExceptionAttribute = ExceptionAttribute::new(&data);
        let instance2: ExceptionAttribute = ExceptionAttribute::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn exception_attribute_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let data2: Vec<u8> = vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: ExceptionAttribute = ExceptionAttribute::new(&data1);
        let instance2: ExceptionAttribute = ExceptionAttribute::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn exception_attribute_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let instance1: ExceptionAttribute = ExceptionAttribute::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: ExceptionAttribute = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}
