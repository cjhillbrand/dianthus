use crate::constants::constant_info::{ConstantInfo};
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

pub fn get_constant(data: &[u8]) -> Box<dyn ConstantInfo>
{
    let tag_value: u8 = data[0].clone();
    match tag_value
    {
        UTF8    =>   { Box::new(Utf8Info::new(&data)) },
        INTEGER => { Box::new(IntegerInfo::new(&data)) },
        FLOAT   => { Box::new(FloatInfo::new(&data)) },
        LONG    => { Box::new(LongInfo::new(&data)) },
        DOUBLE  => { Box::new(DoubleInfo::new(&data)) },
        CLASS   => { Box::new(ClassInfo::new(&data)) },
        STRING  => { Box::new(StringInfo::new(&data)) },
        FIELD_REF   => { Box::new(FieldRefInfo::new(&data)) },
        METHOD_REF  => { Box::new(MethodRefInfo::new(&data)) },
        INTERFACE_METHOD_REF=> { Box::new(InterfaceMethodRefInfo::new(&data)) },
        NAME_AND_TYPE       => { Box::new(NameAndTypeInfo::new(&data)) },
        METHOD_HANDLE       => { Box::new(MethodHandleInfo::new(&data)) },
        METHOD_TYPE         => { Box::new(MethodTypeInfo::new(&data)) },
        INVOKE_DYNAMIC      => { Box::new(InvokeDynamicInfo::new(&data)) }
        _ => { panic!("Unidentified attribute") }
    }
}