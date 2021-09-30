use crate::constants::constant_info::ConstantInfo;
use crate::util::{to_u8, to_u16, to_vec};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Utf8Info
{
    tag: u8,
    length: u16,
    // Mapping this to a string looks complex.
    // refer back to documentation for this.
    value: Vec<u8>
}

impl ConstantInfo for Utf8Info
{
    fn tag(&self) -> &u8 { &self.tag }
}

impl Utf8Info
{
    pub fn new(data: &[u8]) -> Utf8Info
    {
        let mut iter = data.iter();
        let mut result: Utf8Info = Default::default();

        result.tag = to_u8(&mut iter).unwrap();
        let length = to_u16(&mut iter).unwrap();
        result.length = length.clone();
        result.value = to_vec(&mut iter, length as usize).unwrap();

        result
    }
}

#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::constants::utf8_info::Utf8Info;

    #[test]
    fn utf8_info_implements_equality_by_default()
    {
        let instance1: Utf8Info = Default::default();
        let instance2: Utf8Info = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn utf8_info_constructs_expected_struct()
    {
        let data: Vec<u8> = vec![1, 0, 1, 1, 1, 1, 1, 1];
        let result: Utf8Info = Utf8Info::new(&data);

        let bit8: u8 = 1;
        let bit16: u16 = 257;
        assert_eq!(bit8, result.tag);
        assert_eq!(1, result.length);
        assert_eq!(1, result.value.len());
        assert_eq!(1, result.value[0]);
    }

    #[test]
    fn utf8_info_implements_equality_correctly()
    {
        let data: Vec<u8> = vec![1, 0, 1, 4, 5, 6, 7, 8];
        let instance1: Utf8Info = Utf8Info::new(&data);
        let instance2: Utf8Info = Utf8Info::new(&data);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn utf8_info_implements_equality_correctly_when_not_equal()
    {
        let data1: Vec<u8> = vec![1, 0, 1, 4, 5, 6, 7, 8];
        let data2: Vec<u8> = vec![8, 0, 1, 5, 4, 3, 2, 1];
        let instance1: Utf8Info = Utf8Info::new(&data1);
        let instance2: Utf8Info = Utf8Info::new(&data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn utf8_info_implements_json_serialization_correctly() -> Result<()>
    {
        let data: Vec<u8> = vec![1, 0, 1, 4, 5, 6, 7, 8];
        let instance1: Utf8Info = Utf8Info::new(&data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: Utf8Info = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }
}