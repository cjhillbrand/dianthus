use crate::attributes::attribute_info::AttributeInfo;

#[warn(dead_code)]
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
    attributes: Vec<Box<dyn AttributeInfo>>
}
#[warn(dead_code)]
pub enum AccessFlags
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