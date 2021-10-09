use crate::constants::double_info::DoubleInfo;
use crate::constants::field_ref_info::FieldRefInfo;
use crate::constants::float_info::FloatInfo;
use crate::constants::integer_info::IntegerInfo;
use crate::constants::interface_method_ref_info::InterfaceMethodRefInfo;
use crate::constants::invoke_dynamic_info::InvokeDynamicInfo;
use crate::constants::long_info::LongInfo;
use crate::constants::method_handle_info::MethodHandleInfo;
use crate::constants::method_ref_info::MethodRefInfo;
use crate::constants::method_type_info::MethodTypeInfo;
use crate::constants::name_and_type_info::NameAndTypeInfo;
use crate::constants::string_info::StringInfo;
use crate::constants::utf8_info::Utf8Info;
use crate::constants::class_info::ClassInfo;
use crate::constants::constant_info::ConstantInfo;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum ConstantContainer
{
    ClassInfo(ClassInfo),
    DoubleInfo(DoubleInfo),
    FieldRefInfo(FieldRefInfo),
    FloatInfo(FloatInfo),
    IntegerInfo(IntegerInfo),
    InterfaceMethodInfo(InterfaceMethodRefInfo),
    InvokeDynamicInfo(InvokeDynamicInfo),
    LongInfo(LongInfo),
    MethodHandleInfo(MethodHandleInfo),
    MethodRefInfo(MethodRefInfo),
    MethodTypeInfo(MethodTypeInfo),
    NameAndTypeInfo(NameAndTypeInfo),
    StringInfo(StringInfo),
    Utf8Info(Utf8Info)
}

impl ConstantInfo for ConstantContainer
{
    fn tag(&self) -> &u8 {
        match self
        {
            ConstantContainer::ClassInfo(v) => v.tag(),
            ConstantContainer::DoubleInfo(v) => v.tag(),
            ConstantContainer::FieldRefInfo(v) => v.tag(),
            ConstantContainer::FloatInfo(v) => v.tag(),
            ConstantContainer::IntegerInfo(v) => v.tag(),
            ConstantContainer::InterfaceMethodInfo(v) => v.tag(),
            ConstantContainer::InvokeDynamicInfo(v) => v.tag(),
            ConstantContainer::LongInfo(v) => v.tag(),
            ConstantContainer::MethodHandleInfo(v) => v.tag(),
            ConstantContainer::MethodRefInfo(v) => v.tag(),
            ConstantContainer::MethodTypeInfo(v) => v.tag(),
            ConstantContainer::NameAndTypeInfo(v) => v.tag(),
            ConstantContainer::StringInfo(v) => v.tag(),
            ConstantContainer::Utf8Info(v) => v.tag()
        }
    }
}