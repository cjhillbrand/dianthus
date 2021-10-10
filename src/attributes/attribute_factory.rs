


use crate::read_bytes::ReadBytes;
use crate::attributes::code_attribute::CodeAttribute;
use crate::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::attributes::signature_attribute::SignatureAttribute;
use crate::attributes::exception_attribute::ExceptionAttribute;
use crate::attributes::attribute_container::AttributeContainer;
use crate::constants::constant_container::ConstantContainer;

const CONSTANT_VALUE: &str = "ConstantValue";
const CODE: &str = "Code";
const DEPRECATED: &str = "Deprecated";
const SIGNATURE: &str = "Signature";
const EXCEPTION: &str = "Exception";

pub fn get_attribute_container<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> AttributeContainer
{
    let attr_index = data.peek_u16();
    let constant_info = &constant_pool[attr_index as usize];
    let attribute_type: &str = match constant_info
    {
        ConstantContainer::Utf8Info( v ) => v.get_string(),
        _ => panic!("Expected enum value of utf8info.")
    };

    match attribute_type
    {
        CODE =>             { AttributeContainer::CodeAttribute(CodeAttribute::new(data, constant_pool)) },
        CONSTANT_VALUE =>   { AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(data)) },
        DEPRECATED =>       { AttributeContainer::DeprecatedAttribute(DeprecatedAttribute::new(data)) },
        SIGNATURE =>        { AttributeContainer::SignatureAttribute(SignatureAttribute::new(data)) },
        EXCEPTION =>        { AttributeContainer::ExceptionAttribute(ExceptionAttribute::new(data)) }
        &_ => panic!("Unidentified attribute")
    }
}