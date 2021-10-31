use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::code_attribute::CodeAttribute;
use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
use crate::entities::attributes::constants::*;
use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
use crate::entities::attributes::exception_attribute::ExceptionAttribute;
use crate::entities::attributes::inner_class_attribute::InnerClassAttribute;
use crate::entities::attributes::line_number_table_attribute::LineNumberTableAttribute;
use crate::entities::attributes::runtime_visible_attribute::RuntimeVisibleAttribute;
use crate::entities::attributes::signature_attribute::SignatureAttribute;
use crate::entities::attributes::source_file_attribute::SourceFileAttribute;
use crate::entities::attributes::stack_map_table_attribute::StackMapTableAttribute;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

pub fn get_attribute_container<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> AttributeContainer {
	let attr_index = data.peek_u16();
	let constant_info = &constant_pool[attr_index as usize];
	let attribute_type: &str = match constant_info {
		ConstantContainer::Utf8Info(v) => v.get_string(),
		_ => panic!("Expected enum value of utf8info.")
	};

	match attribute_type {
		CODE => AttributeContainer::CodeAttribute(CodeAttribute::new(data, constant_pool)),
		CONSTANT_VALUE => AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(data, constant_pool)),
		DEPRECATED => AttributeContainer::DeprecatedAttribute(DeprecatedAttribute::new(data, constant_pool)),
		SIGNATURE => AttributeContainer::SignatureAttribute(SignatureAttribute::new(data, constant_pool)),
		EXCEPTION => AttributeContainer::ExceptionAttribute(ExceptionAttribute::new(data, constant_pool)),
		LINE_NUMBER_TABLE => {
			AttributeContainer::LineNumberTableAttribute(LineNumberTableAttribute::new(data, constant_pool))
		}
		SOURCE_FILE => AttributeContainer::SourceFileAttribute(SourceFileAttribute::new(data, constant_pool)),
		STACK_MAP_TABLE => AttributeContainer::StackMapTableAttribute(StackMapTableAttribute::new(data, constant_pool)),
		RUNTIME_VISIBLE_ANNOTATION => AttributeContainer::RunTimeVisibleAnnotationAttribute(RuntimeVisibleAttribute::new(data, constant_pool)),
		INNER_CLASS_ATTRIBUTE => AttributeContainer::InnerClassAttribute(InnerClassAttribute::new(data, constant_pool)),
		&_ => panic!("Unidentified attribute: {}", attribute_type)
	}
}
