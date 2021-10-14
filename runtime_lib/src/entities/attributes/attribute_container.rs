use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::attributes::code_attribute::CodeAttribute;
use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::entities::attributes::exception_attribute::ExceptionAttribute;
use crate::entities::attributes::line_number_table_attribute::LineNumberTableAttribute;
use crate::entities::attributes::signature_attribute::SignatureAttribute;
use crate::entities::attributes::source_file_attribute::SourceFileAttribute;
use crate::entities::constants::constant_container::ConstantContainer;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum AttributeContainer {
	CodeAttribute(CodeAttribute),
	ConstantAttribute(ConstantValueAttribute),
	DeprecatedAttribute(DeprecatedAttribute),
	SignatureAttribute(SignatureAttribute),
	ExceptionAttribute(ExceptionAttribute),
	LineNumberTableAttribute(LineNumberTableAttribute),
	SourceFileAttribute(SourceFileAttribute)
}

impl AttributeContainer
{
	pub fn get_name<'a>(&self, constant_pool: &'a[ConstantContainer]) -> &'a str
	{
		let index: u16 = self.name_index().clone();
		match &constant_pool[index as usize]
		{
			ConstantContainer::Utf8Info(v) => { v.get_string() },
			_ => { panic!("Expected a UTF8Info at index: {}", index) }
		}
	}
}

impl AttributeInfo for AttributeContainer {
	fn name_index(&self) -> &u16 {
		match self {
			AttributeContainer::CodeAttribute(v) => v.name_index(),
			AttributeContainer::ConstantAttribute(v) => v.name_index(),
			AttributeContainer::DeprecatedAttribute(v) => v.name_index(),
			AttributeContainer::SignatureAttribute(v) => v.name_index(),
			AttributeContainer::ExceptionAttribute(v) => v.name_index(),
			AttributeContainer::LineNumberTableAttribute(v) => v.name_index(),
			AttributeContainer::SourceFileAttribute(v) => v.name_index()
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
			AttributeContainer::SourceFileAttribute(v) => v.attr_length()
		}
	}
}
