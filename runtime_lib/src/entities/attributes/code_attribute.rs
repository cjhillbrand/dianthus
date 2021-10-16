use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::attribute_factory::get_attribute_container;
use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct CodeAttribute {
	attribute_name: String,
	attribute_length: u32,
	max_stack: u16,
	max_locals: u16,
	code: Vec<u8>,
	exception_table: Vec<ExceptionInfo>,
	attribute_info: Vec<AttributeContainer>
}

impl AttributeInfo for CodeAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl CodeAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> CodeAttribute {
		let mut result: CodeAttribute = Default::default();
		result.attribute_name = constant_pool[data.pop_u16() as usize].get_string();
		result.attribute_length = data.pop_u32();
		result.max_stack = data.pop_u16();
		result.max_locals = data.pop_u16();
		let code_length: u32 = data.pop_u32();
		result.code = data.pop_vec(code_length as usize);
		let exception_table_length: u16 = data.pop_u16();

		result.exception_table = Vec::new();
		for _j in 0..exception_table_length {
			let exception_info: ExceptionInfo = ExceptionInfo::new(data);
			result.exception_table.push(exception_info);
		}

		let attribute_count: u16 = data.pop_u16();
		result.attribute_info = Vec::new();
		for _i in 0..attribute_count {
			result.attribute_info.push(get_attribute_container(data, constant_pool));
		}

		result
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		attribute_name: String, attribute_length: u32, max_stack: u16, max_locals: u16, code: Vec<u8>,
		exception_table: Vec<ExceptionInfo>, attribute_info: Vec<AttributeContainer>
	) -> CodeAttribute {
		CodeAttribute {
			attribute_name,
			attribute_length,
			max_stack,
			max_locals,
			code,
			exception_table,
			attribute_info
		}
	}

	pub fn get_max_stack(&self) -> u16 { self.max_stack.clone() }

	pub fn get_max_locals(&self) -> u16 { self.max_locals.clone() }

	pub fn get_code(&self) -> &Vec<u8> { &self.code }
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ExceptionInfo {
	start_pc: u16,
	end_pc: u16,
	handler_pc: u16,
	catch_type: u16
}

impl ExceptionInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> ExceptionInfo {
		ExceptionInfo {
			start_pc: data.pop_u16(),
			end_pc: data.pop_u16(),
			handler_pc: data.pop_u16(),
			catch_type: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> ExceptionInfo {
		ExceptionInfo {
			start_pc,
			end_pc,
			handler_pc,
			catch_type
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::attributes::code_attribute::{CodeAttribute, ExceptionInfo};
	use crate::entities::attributes::test_fixture::model_builder::{create_code, create_exception_info};

	#[test]
	fn exception_info_implements_equality_by_default() {
		let instance1: ExceptionInfo = Default::default();
		let instance2: ExceptionInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn exception_info_implements_equality_correctly() {
		let instance1: ExceptionInfo = create_exception_info();
		let instance2: ExceptionInfo = create_exception_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn exception_info_implements_equality_correctly_when_not_equal() {
		let instance1: ExceptionInfo = create_exception_info();
		let mut instance2: ExceptionInfo = create_exception_info();
		instance2.start_pc += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn exception_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: ExceptionInfo = create_exception_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: ExceptionInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}

	#[test]
	fn code_attribute_implements_equality_by_default() {
		let instance1: CodeAttribute = Default::default();
		let instance2: CodeAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	fn code_attribute_implements_equality_correctly() {
		let instance1: CodeAttribute = create_code();
		let instance2: CodeAttribute = create_code();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn code_attribute_implements_equality_correctly_when_not_equal() {
		let instance1: CodeAttribute = create_code();
		let mut instance2: CodeAttribute = create_code();
		instance2.attribute_length += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn code_attribute_implements_json_serialization_correctly() -> Result<()> {
		let instance1: CodeAttribute = create_code();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: CodeAttribute = serde_json::from_str(&json)?;
		assert_eq!(instance2, instance3);
		Ok(())
	}
}
