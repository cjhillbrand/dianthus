use crate::attributes::attribute_info::AttributeInfo;



use crate::attributes::code_attribute::CodeAttribute;
use crate::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::attributes::signature_attribute::SignatureAttribute;
use crate::attributes::exception_attribute::ExceptionAttribute;



#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum AttributeContainer
{
    CodeAttribute(CodeAttribute),
    ConstantAttribute(ConstantValueAttribute),
    DeprecatedAttribute(DeprecatedAttribute),
    SignatureAttribute(SignatureAttribute),
    ExceptionAttribute(ExceptionAttribute),
}

impl AttributeInfo for AttributeContainer
{
    fn name_index(&self) -> &u16 {
        match self
        {
            AttributeContainer::CodeAttribute(v) => v.name_index(),
            AttributeContainer::ConstantAttribute(v) => v.name_index(),
            AttributeContainer::DeprecatedAttribute(v) => v.name_index(),
            AttributeContainer::SignatureAttribute(v) => v.name_index(),
            AttributeContainer::ExceptionAttribute(v) => v.name_index()
        }
    }

    fn attr_length(&self) -> &u32 {
        match self
        {
            AttributeContainer::CodeAttribute(v) => v.attr_length(),
            AttributeContainer::ConstantAttribute(v) => v.attr_length(),
            AttributeContainer::DeprecatedAttribute(v) => v.attr_length(),
            AttributeContainer::SignatureAttribute(v) => v.attr_length(),
            AttributeContainer::ExceptionAttribute(v) => v.attr_length()
        }
    }
}
