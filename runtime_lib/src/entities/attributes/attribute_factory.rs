use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::code_attribute::CodeAttribute;
use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::entities::attributes::exception_attribute::ExceptionAttribute;
use crate::entities::attributes::line_number_table_attribute::LineNumberTableAttribute;
use crate::entities::attributes::signature_attribute::SignatureAttribute;
use crate::entities::attributes::source_file_attribute::SourceFileAttribute;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

const CONSTANT_VALUE: &str = "ConstantValue";
const CODE: &str = "Code";
const DEPRECATED: &str = "Deprecated";
const SIGNATURE: &str = "Signature";
const EXCEPTION: &str = "Exception";
const LINE_NUMBER_TABLE: &str = "LineNumberTable";
const SOURCE_FILE: &str = "SourceFile";

pub fn get_attribute_container<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> AttributeContainer {
	let attr_index = data.peek_u16();
	let constant_info = &constant_pool[attr_index as usize];
	let attribute_type: &str = match constant_info {
		ConstantContainer::Utf8Info(v) => v.get_string(),
		_ => panic!("Expected enum value of utf8info.")
	};

	match attribute_type {
		CODE => AttributeContainer::CodeAttribute(CodeAttribute::new(data, constant_pool)),
		CONSTANT_VALUE => AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(data)),
		DEPRECATED => AttributeContainer::DeprecatedAttribute(DeprecatedAttribute::new(data)),
		SIGNATURE => AttributeContainer::SignatureAttribute(SignatureAttribute::new(data)),
		EXCEPTION => AttributeContainer::ExceptionAttribute(ExceptionAttribute::new(data)),
		LINE_NUMBER_TABLE => AttributeContainer::LineNumberTableAttribute(LineNumberTableAttribute::new(data)),
		SOURCE_FILE => AttributeContainer::SourceFileAttribute(SourceFileAttribute::new(data)),
		&_ => panic!("Unidentified attribute: {}", attribute_type)
	}
}