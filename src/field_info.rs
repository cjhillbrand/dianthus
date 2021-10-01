use crate::attributes::attribute_info::AttributeInfo;
use crate::constants::constant_info::ConstantInfo;
use crate::util::to_u16;
use crate::attributes::attribute_factory::get_attribute;

#[derive(Default)]
pub struct FieldInfo
{
    /// Mask of flags used to denote access permissions to
    /// and properties of ths filed.
    access_flags: u16,

    /// Valid index into the constant_pool table. The entry at the
    /// index must be a CONSTANT_Utf8_info struct.
    name_index: u16,

    /// Valid index into the constant_pool table. The entry at the
    /// index must be a CONSTANT_Utf8_info struct.
    descriptor_index: u16,

    /// Number of additional attributes of this field.
    attributes_count: u16,

    /// Each value of the attributes table must be an
    /// attribute structure. Can have any number of attributes.
    // TODO: implement Clone, Debug, Serialize, Deserialize for this type
    //      so other structs can derive it.
    attributes: Vec<Box<dyn AttributeInfo>>
}

impl FieldInfo
{
    pub fn new(data: &[u8], constant_pool: &[Box<dyn ConstantInfo>]) -> FieldInfo
    {
        let mut iter = data.iter();
        let mut result: FieldInfo = Default::default();
        result.access_flags = to_u16(&mut iter).unwrap();
        result.name_index = to_u16(&mut iter).unwrap();
        result.descriptor_index = to_u16(&mut iter).unwrap();
        result.attributes_count = to_u16(&mut iter).unwrap();

        result.attributes = Vec::new();
        for _i in 0..result.attributes_count.clone()
        {
            result.attributes.push(get_attribute(&data, &constant_pool));
        }

        result
    }
}

#[warn(dead_code)]
pub enum FieldAccessFlags
{
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000
}