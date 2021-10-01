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
use std::collections::VecDeque;

pub fn get_constant(mut data: &mut VecDeque<u8>) -> Box<dyn ConstantInfo>
{
    let tag_value: u8 = data[0].clone();
    match tag_value
    {
        UTF8    =>   { Box::new(Utf8Info::new(&mut data)) },
        INTEGER => { Box::new(IntegerInfo::new(&mut data)) },
        FLOAT   => { Box::new(FloatInfo::new(&mut data)) },
        LONG    => { Box::new(LongInfo::new(&mut data)) },
        DOUBLE  => { Box::new(DoubleInfo::new(&mut data)) },
        CLASS   => { Box::new(ClassInfo::new(&mut data)) },
        STRING  => { Box::new(StringInfo::new(&mut data)) },
        FIELD_REF   => { Box::new(FieldRefInfo::new(&mut data)) },
        METHOD_REF  => { Box::new(MethodRefInfo::new(&mut data)) },
        INTERFACE_METHOD_REF=> { Box::new(InterfaceMethodRefInfo::new(&mut data)) },
        NAME_AND_TYPE       => { Box::new(NameAndTypeInfo::new(&mut data)) },
        METHOD_HANDLE       => { Box::new(MethodHandleInfo::new(&mut data)) },
        METHOD_TYPE         => { Box::new(MethodTypeInfo::new(&mut data)) },
        INVOKE_DYNAMIC      => { Box::new(InvokeDynamicInfo::new(&mut data)) }
        _ => { panic!("Unidentified attribute") }
    }
}