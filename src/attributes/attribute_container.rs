use crate::attributes::attribute_info::AttributeInfo;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use crate::constants::constant_info::ConstantInfo;
use crate::constants::utf8_info::Utf8Info;
use crate::attributes::code_attribute::CodeAttribute;
use crate::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::attributes::signature_attribute::SignatureAttribute;
use crate::attributes::exception_attribute::ExceptionAttribute;
use std::any::Any;
use serde::de::Error;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum AttributeContainer
{
    CodeAttribute(CodeAttribute),
    ConstantAttribute(ConstantValueAttribute),
    DeprecatedAttribute(DeprecatedAttribute),
    SignatureAttribute(SignatureAttribute),
    ExceptionAttribute(ExceptionAttribute),
}

