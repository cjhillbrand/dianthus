use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::attributes::code_attribute::CodeAttribute;
use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::entities::attributes::exception_attribute::ExceptionAttribute;
use crate::entities::attributes::inner_class_attribute::InnerClassAttribute;
use crate::entities::attributes::line_number_table_attribute::LineNumberTableAttribute;
use crate::entities::attributes::runtime_visible_attribute::RuntimeVisibleAttribute;
use crate::entities::attributes::signature_attribute::SignatureAttribute;
use crate::entities::attributes::source_file_attribute::SourceFileAttribute;
use crate::entities::attributes::stack_map_table_attribute::StackMapTableAttribute;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum AttributeContainer {
	CodeAttribute(CodeAttribute),
	ConstantAttribute(ConstantValueAttribute),
	DeprecatedAttribute(DeprecatedAttribute),
	SignatureAttribute(SignatureAttribute),
	ExceptionAttribute(ExceptionAttribute),
	LineNumberTableAttribute(LineNumberTableAttribute),
	SourceFileAttribute(SourceFileAttribute),
	StackMapTableAttribute(StackMapTableAttribute),
	RunTimeVisibleAnnotationAttribute(RuntimeVisibleAttribute),
	InnerClassAttribute(InnerClassAttribute)
}

impl AttributeInfo for AttributeContainer {
	fn name(&self) -> &str {
		match self {
			AttributeContainer::CodeAttribute(v) => v.name(),
			AttributeContainer::ConstantAttribute(v) => v.name(),
			AttributeContainer::DeprecatedAttribute(v) => v.name(),
			AttributeContainer::SignatureAttribute(v) => v.name(),
			AttributeContainer::ExceptionAttribute(v) => v.name(),
			AttributeContainer::LineNumberTableAttribute(v) => v.name(),
			AttributeContainer::SourceFileAttribute(v) => v.name(),
			AttributeContainer::StackMapTableAttribute(v) => v.name(),
			AttributeContainer::RunTimeVisibleAnnotationAttribute(v) => v.name(),
			AttributeContainer::InnerClassAttribute(v) => v.name()
		}
	}

	fn attr_length(&self) -> &u32 {
		match self {
			AttributeContainer::CodeAttribute(v) => v.attr_length(),
			AttributeContainer::ConstantAttribute(v) => v.attr_length(),
			AttributeContainer::DeprecatedAttribute(v) => v.attr_length(),
			AttributeContainer::SignatureAttribute(v) => v.attr_length(),
			AttributeContainer::ExceptionAttribute(v) => v.attr_length(),
			AttributeContainer::LineNumberTableAttribute(v) => v.attr_length(),
			AttributeContainer::SourceFileAttribute(v) => v.attr_length(),
			AttributeContainer::StackMapTableAttribute(v) => v.attr_length(),
			AttributeContainer::RunTimeVisibleAnnotationAttribute(v) => v.attr_length(),
			AttributeContainer::InnerClassAttribute(v) => v.attr_length()
		}
	}
}
