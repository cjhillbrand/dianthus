use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16 };

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StringInfo
{
    tag: u8,
    string_index: u16
}

impl ConstantInfo for StringInfo
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl StringInfo
{
    pub fn new(data: &[u8]) -> StringInfo
    {
        let mut iter = data.iter();
        StringInfo
        {
            tag: to_u8(&mut iter).unwrap(),
            string_index: to_u16(&mut iter).unwrap()
        }
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::string_info::StringInfo;

    #[test]
    fn string_info_implements_equality_by_default()
    {
        let instance1: StringInfo = Default::default();
        let instance2: StringInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn string_info_constructs_expected_struct()
    {
        let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result: StringInfo = StringInfo::new(&data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        assert_eq!(bit16, result.string_index);
    }

    #[test]
    fn string_info_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: StringInfo = StringInfo::new(&data);
        let instance2: StringInfo = StringInfo::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn string_info_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let data2: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: StringInfo = StringInfo::new(&data1);
        let instance2: StringInfo = StringInfo::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn string_info_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: StringInfo = StringInfo::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: StringInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}