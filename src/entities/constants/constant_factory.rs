use crate::entities::constants::class_info::ClassInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::constants::constant_info::*;
use crate::entities::constants::double_info::DoubleInfo;
use crate::entities::constants::field_ref_info::FieldRefInfo;
use crate::entities::constants::float_info::FloatInfo;
use crate::entities::constants::integer_info::IntegerInfo;
use crate::entities::constants::interface_method_ref_info::InterfaceMethodRefInfo;
use crate::entities::constants::invoke_dynamic_info::InvokeDynamicInfo;
use crate::entities::constants::long_info::LongInfo;
use crate::entities::constants::method_handle_info::MethodHandleInfo;
use crate::entities::constants::method_ref_info::MethodRefInfo;
use crate::entities::constants::method_type_info::MethodTypeInfo;
use crate::entities::constants::name_and_type_info::NameAndTypeInfo;
use crate::entities::constants::string_info::StringInfo;
use crate::entities::constants::utf8_info::Utf8Info;
use crate::entities::read_bytes::ReadBytes;

pub fn get_constant_container<T: ReadBytes>(data: &mut T) -> ConstantContainer {
    let tag_value: u8 = data.peek_u8();
    match tag_value {
        UTF8 => ConstantContainer::Utf8Info(Utf8Info::new(data)),
        INTEGER => ConstantContainer::IntegerInfo(IntegerInfo::new(data)),
        FLOAT => ConstantContainer::FloatInfo(FloatInfo::new(data)),
        LONG => ConstantContainer::LongInfo(LongInfo::new(data)),
        DOUBLE => ConstantContainer::DoubleInfo(DoubleInfo::new(data)),
        CLASS => ConstantContainer::ClassInfo(ClassInfo::new(data)),
        STRING => ConstantContainer::StringInfo(StringInfo::new(data)),
        FIELD_REF => ConstantContainer::FieldRefInfo(FieldRefInfo::new(data)),
        METHOD_REF => ConstantContainer::MethodRefInfo(MethodRefInfo::new(data)),
        INTERFACE_METHOD_REF => {
            ConstantContainer::InterfaceMethodInfo(InterfaceMethodRefInfo::new(data))
        }
        NAME_AND_TYPE => ConstantContainer::NameAndTypeInfo(NameAndTypeInfo::new(data)),
        METHOD_HANDLE => ConstantContainer::MethodHandleInfo(MethodHandleInfo::new(data)),
        METHOD_TYPE => ConstantContainer::MethodTypeInfo(MethodTypeInfo::new(data)),
        INVOKE_DYNAMIC => ConstantContainer::InvokeDynamicInfo(InvokeDynamicInfo::new(data)),
        _ => {
            panic!("Unidentified constant info: {}.", tag_value)
        }
    }
}
