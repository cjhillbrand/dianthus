/// The attributes defined by this specification
/// as appearing in the attributes table of a field_info/method_info
///
/// Implemented: 2,3,5,8,9,
/// Annotations: 15,16,17,18,19,20
///
use std::any::Any;
use crate::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::attributes::code_attribute::CodeAttribute;
use crate::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::attributes::exception_attribute::ExceptionAttribute;
use crate::attributes::signature_attribute::SignatureAttribute;

pub trait AttributeInfo
{
    fn name_index(&self) -> &u16;
    fn attr_length(&self) -> &u32;
    fn as_any(&self) -> &dyn Any;
}

/*
TODO: Support Annotations.
pub struct RunTimeVisibleAnnotationsAttribute
{
}

pub struct RunTimeInvisibleAnnotationsAttribute
{
}

pub struct RuntimeVisibleParameterAnnotationsAttribute
{
}

pub struct RuntimeInvisibleParameterAnnotationsAttribute
{
}

pub struct AnnotationsDefaultAttribute
{
}
*/