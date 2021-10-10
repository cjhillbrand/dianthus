use crate::constants::constant_info::{ConstantInfo, UTF8, INTEGER, FLOAT, DOUBLE, LONG, CLASS,
    STRING, FIELD_REF, METHOD_REF, INTERFACE_METHOD_REF, NAME_AND_TYPE, METHOD_HANDLE, METHOD_TYPE, INVOKE_DYNAMIC};
use crate::constants::utf8_info::Utf8Info;
use crate::constants::integer_info::IntegerInfo;
use crate::constants::float_info::FloatInfo;
use crate::constants::long_info::LongInfo;
use crate::constants::double_info::DoubleInfo;
use crate::constants::class_info::ClassInfo;
use crate::constants::string_info::StringInfo;
use crate::constants::interface_method_ref_info::InterfaceMethodRefInfo;
use crate::constants::field_ref_info::FieldRefInfo;
use crate::constants::method_ref_info::MethodRefInfo;
use crate::constants::name_and_type_info::NameAndTypeInfo;
use crate::constants::method_handle_info::MethodHandleInfo;
use crate::constants::method_type_info::MethodTypeInfo;
use crate::constants::invoke_dynamic_info::InvokeDynamicInfo;
use crate::read_bytes::ReadBytes;
use crate::constants::constant_container::ConstantContainer;

pub fn get_constant_container<T: ReadBytes>(mut data: &mut T) -> ConstantContainer
{
    let tag_value: u8 = data.peek_u8();
    match tag_value
    {
        UTF8    => { ConstantContainer::Utf8Info(Utf8Info::new(data)) },
        INTEGER => { ConstantContainer::IntegerInfo(IntegerInfo::new(data)) },
        FLOAT   => { ConstantContainer::FloatInfo(FloatInfo::new(data)) },
        LONG    => { ConstantContainer::LongInfo(LongInfo::new(data)) },
        DOUBLE  => { ConstantContainer::DoubleInfo(DoubleInfo::new(data)) },
        CLASS   => { ConstantContainer::ClassInfo(ClassInfo::new(data)) },
        STRING  => { ConstantContainer::StringInfo(StringInfo::new(data)) },
        FIELD_REF   => { ConstantContainer::FieldRefInfo(FieldRefInfo::new(data)) },
        METHOD_REF  => { ConstantContainer::MethodRefInfo(MethodRefInfo::new(data)) },
        INTERFACE_METHOD_REF=> { ConstantContainer::InterfaceMethodInfo((InterfaceMethodRefInfo::new(data))) },
        NAME_AND_TYPE       => { ConstantContainer::NameAndTypeInfo(NameAndTypeInfo::new(data)) },
        METHOD_HANDLE       => { ConstantContainer::MethodHandleInfo(MethodHandleInfo::new(data)) },
        METHOD_TYPE         => { ConstantContainer::MethodTypeInfo(MethodTypeInfo::new(data)) },
        INVOKE_DYNAMIC      => { ConstantContainer::InvokeDynamicInfo(InvokeDynamicInfo::new(data)) }
        _ => { panic!("Unidentified constant info: {}.", tag_value) }
    }
}