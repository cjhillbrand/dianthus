#[cfg(test)]
pub mod model_builder {
	use crate::entities::attributes::code_attribute::{CodeAttribute, ExceptionInfo};
	use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
	use crate::entities::attributes::constants::*;
	use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
	use crate::entities::attributes::exception_attribute::ExceptionAttribute;
	use crate::entities::attributes::line_number_table_attribute::{LineNumberTableAttribute, TableElement};
	use crate::entities::attributes::signature_attribute::SignatureAttribute;
	use crate::entities::attributes::source_file_attribute::SourceFileAttribute;

	pub fn create_code() -> CodeAttribute {
		CodeAttribute::new_test_model(
			CODE.to_string(),
			5,
			5,
			5,
			6,
			vec![0, 1, 2, 3, 4, 5],
			1,
			vec![],
			1,
			vec![]
		)
	}

	pub fn create_exception_info() -> ExceptionInfo { ExceptionInfo::new_test_model(1, 2, 3, 4) }

	pub fn create_constant_value() -> ConstantValueAttribute {
		ConstantValueAttribute::new_test_model(CONSTANT_VALUE.to_string(), 10, 5)
	}

	pub fn create_deprecated() -> DeprecatedAttribute { DeprecatedAttribute::new_test_model(DEPRECATED.to_string(), 4) }

	pub fn create_exception() -> ExceptionAttribute {
		ExceptionAttribute::new_test_model(EXCEPTION.to_string(), 8, 1, 1)
	}

	pub fn create_line_number_table() -> LineNumberTableAttribute {
		LineNumberTableAttribute::new_test_model(LINE_NUMBER_TABLE.to_string(), 10, 1, vec![])
	}

	pub fn create_table_element() -> TableElement { TableElement::new_test_model(1, 2) }

	pub fn create_signature() -> SignatureAttribute { SignatureAttribute::new_test_model(SIGNATURE.to_string(), 4, 1) }

	pub fn create_source_file() -> SourceFileAttribute {
		SourceFileAttribute::new_test_model(SOURCE_FILE.to_string(), 4, 1)
	}
}
