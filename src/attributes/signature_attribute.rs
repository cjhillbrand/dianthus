use crate::attributes::attribute_info::AttributeInfo;
use crate::util::{to_u16, to_u32};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct SignatureAttribute
{
    attribute_name_index: u16,
    attribute_length: u32,
    signature_index: u16
}

impl AttributeInfo for SignatureAttribute
{
    fn name_index(&self) -> &u16 { &self.attribute_name_index }
    fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl SignatureAttribute
{
    pub fn new(data: &Vec<u8>) -> SignatureAttribute
    {
        let mut iter = data.iter();
        SignatureAttribute
        {
            attribute_name_index: to_u16(&mut iter).unwrap(),
            attribute_length: to_u32(&mut iter).unwrap(),
            signature_index: to_u16(&mut iter).unwrap()
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::attributes::signature_attribute::SignatureAttribute;
    use serde_json::Result;

    #[test]
    fn signature_attribute_implements_equality_by_default()
    {
        let instance1: SignatureAttribute = Default::default();
        let instance2: SignatureAttribute = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn signature_attribute_constructs_expected_struct()
    {
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result: SignatureAttribute = SignatureAttribute::new(&data);

        let bit16: u16 = 257;
        let bit32: u32 = 16843009;
        assert_eq!(bit16, result.attribute_name_index);
        assert_eq!(bit32, result.attribute_length);
    }

    #[test]
    fn signature_attribute_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: SignatureAttribute = SignatureAttribute::new(&data);
        let instance2: SignatureAttribute = SignatureAttribute::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn signature_attribute_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let data2: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: SignatureAttribute = SignatureAttribute::new(&data1);
        let instance2: SignatureAttribute = SignatureAttribute::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn signature_attribute_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: SignatureAttribute = SignatureAttribute::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: SignatureAttribute = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}
