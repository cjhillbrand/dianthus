use std::collections::HashMap;

use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::attribute_factory::get_attribute_container;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::constants::constant_factory::get_constant_container;
use crate::entities::field_info::FieldInfo;
use crate::entities::method_info::MethodInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ClassStruct {
	magic: u32,
	minor_version: u16,
	major_version: u16,
	constant_pool: Vec<ConstantContainer>,
	access_flags: u16,
	this_class: u16,
	super_class: u16,
	interfaces: Vec<u16>,
	field_info: HashMap<String, FieldInfo>,
	method_info: HashMap<String, MethodInfo>,
	attribute_info: Vec<AttributeContainer>
}

impl ClassStruct {
	pub fn new<T: ReadBytes>(data: &mut T) -> ClassStruct {
		let mut result: ClassStruct = Default::default();
		result.magic = data.pop_u32();
		result.minor_version = data.pop_u16();
		result.major_version = data.pop_u16();
		let constants_count: u16 = data.pop_u16();
		result.constant_pool = Vec::new();
		result.constant_pool.push(ConstantContainer::None);
		for _i in 0..constants_count - 1 {
			result.constant_pool.push(get_constant_container(data));
		}

		result.access_flags = data.pop_u16();
		result.this_class = data.pop_u16();
		result.super_class = data.pop_u16();
		let interfaces_count: u16 = data.pop_u16();
		result.interfaces = Vec::new();
		for _i in 0..interfaces_count {
			result.interfaces.push(data.pop_u16());
		}

		let fields_count: u16 = data.pop_u16();
		result.field_info = HashMap::new();
		for _i in 0..fields_count {
			let field_info: FieldInfo = FieldInfo::new(data, &result.constant_pool);
			result.field_info.insert(field_info.get_name().to_string(), field_info);
		}

		let methods_count: u16 = data.pop_u16();
		result.method_info = HashMap::new();
		for _i in 0..methods_count {
			let method_info: MethodInfo = MethodInfo::new(data, &result.constant_pool);
			result
				.method_info
				.insert(method_info.get_name().to_string(), method_info);
		}

		let attr_count: u16 = data.pop_u16();
		result.attribute_info = Vec::new();
		for _i in 0..attr_count {
			result
				.attribute_info
				.push(get_attribute_container(data, &result.constant_pool));
		}

		result
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		magic: u32, minor_version: u16, major_version: u16, constant_pool: Vec<ConstantContainer>, access_flags: u16,
		this_class: u16, super_class: u16, interfaces: Vec<u16>, field_info: HashMap<String, FieldInfo>,
		method_info: HashMap<String, MethodInfo>, attribute_info: Vec<AttributeContainer>
	) -> ClassStruct {
		ClassStruct {
			magic,
			minor_version,
			major_version,
			constant_pool,
			access_flags,
			this_class,
			super_class,
			interfaces,
			field_info,
			method_info,
			attribute_info
		}
	}

	pub fn get_name(&self) -> &str {
		let index: u16 = self.this_class;
		match &self.constant_pool[index as usize] {
			ConstantContainer::Utf8Info(v) => v.get_string(),
			_ => {
				panic!("Expected a UTF8Info at index: {}", index)
			}
		}
	}

	pub fn get_method(&self, name: &str) -> &MethodInfo {
		match self.method_info.get(name) {
			Some(method) => method,
			None => panic!("Method not found: {}", name)
		}
	}

	pub fn get_field(&self, name: &str) -> &FieldInfo {
		match self.field_info.get(name) {
			Some(method) => method,
			None => panic!("Method not found: {}", name)
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::class_struct::ClassStruct;
	use crate::entities::test_fixture::model_builder::create_class;

	#[test]
	fn class_struct_implements_equality_by_default() {
		let instance1: ClassStruct = Default::default();
		let instance2: ClassStruct = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn class_struct_implements_equality_correctly() {
		let result: ClassStruct = create_class();
		let result2: ClassStruct = create_class();
		assert_eq!(result, result2);
	}

	#[test]
	fn class_struct_implements_equality_correctly_when_not_equal() {
		let instance1: ClassStruct = create_class();
		let mut instance2: ClassStruct = create_class();
		instance2.this_class += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn class_struct_implements_json_serialization_correctly() -> Result<()> {
		let instance1: ClassStruct = create_class();

		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: ClassStruct = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
