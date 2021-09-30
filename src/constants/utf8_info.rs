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