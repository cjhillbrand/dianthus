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
	constant_pool_count: u16,
	constant_pool: Vec<ConstantContainer>,
	access_flags: u16,
	this_class: u16,
	super_class: u16,
	interfaces_count: u16,
	interfaces: Vec<u16>,
	fields_count: u16,
	field_info: HashMap<String, FieldInfo>,
	methods_count: u16,
	method_info: HashMap<String, MethodInfo>,
	attributes_count: u16,
	attribute_info: Vec<AttributeContainer>
}

impl ClassStruct {
	pub fn new<T: ReadBytes>(data: &mut T) -> ClassStruct {
		let mut result: ClassStruct = Default::default();
		result.magic = data.pop_u32();
		result.minor_version = data.pop_u16();
		result.major_version = data.pop_u16();
		result.constant_pool_count = data.pop_u16();
		result.constant_pool = Vec::new();
		result.constant_pool.push(ConstantContainer::None);
		for _i in 0..result.constant_pool_count - 1 {
			result.constant_pool.push(get_constant_container(data));
		}

		result.access_flags = data.pop_u16();
		result.this_class = data.pop_u16();
		result.super_class = data.pop_u16();
		result.interfaces_count = data.pop_u16();
		result.interfaces = Vec::new();
		for _i in 0..result.interfaces_count {
			result.interfaces.push(data.pop_u16());
		}

		result.fields_count = data.pop_u16();
		result.field_info = HashMap::new();
		for _i in 0..result.fields_count {
			let field_info: FieldInfo = FieldInfo::new(data, &result.constant_pool);
			result.field_info.insert(field_info.get_name().to_string(), field_info);
		}

		result.methods_count = data.pop_u16();
		result.method_info = HashMap::new();
		for _i in 0..result.methods_count {
			let method_info: MethodInfo = MethodInfo::new(data, &result.constant_pool);
			result
				.method_info
				.insert(method_info.get_name().to_string(), method_info);
		}

		result.attributes_count = data.pop_u16();
		result.attribute_info = Vec::new();
		for _i in 0..result.attributes_count {
			result
				.attribute_info
				.push(get_attribute_container(data, &result.constant_pool));
		}

		result
	}

	pub fn get_name(&self) -> &str {
		let index: u16 = self.this_class.clone();
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
	use std::collections::VecDeque;

	use serde_json::Result;

	use crate::entities::class_struct::ClassStruct;
	use crate::vecdeque;

	#[test]
	fn class_struct_implements_equality_by_default() {
		let instance1: ClassStruct = Default::default();
		let instance2: ClassStruct = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn class_struct_implements_equality_correctly() {
		let mut data: VecDeque<u8> = get_default_vec();
		let mut data2: VecDeque<u8> = data.clone();

		let result: ClassStruct = ClassStruct::new(&mut data);
		let result2: ClassStruct = ClassStruct::new(&mut data2);
		assert_eq!(result, result2);
	}

	#[test]
	fn class_struct_implements_equality_correctly_when_not_equal() {
		let mut data: VecDeque<u8> = get_default_vec();
		let mut data2: VecDeque<u8> = data.clone();
		data2[0] = data[0] + 1;

		let result: ClassStruct = ClassStruct::new(&mut data);
		let result2: ClassStruct = ClassStruct::new(&mut data2);
		assert_ne!(result, result2);
	}

	#[test]
	fn class_struct_implements_json_serialization_correctly() -> Result<()> {
		let mut data: VecDeque<u8> = get_default_vec();

		let instance1: ClassStruct = ClassStruct::new(&mut data);
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: ClassStruct = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}

	fn get_default_vec() -> VecDeque<u8> {
		vecdeque![
			202, 254, 186, 190, // magic: u32: 3405691582
			1, 0, // minor_version: u16: 256
			0, 1, // major_version: u16: 1
			0, 2, // constant_pool_count: u16: 1
			1, //      Utf8Info::tag: 1
			0, 13, //      Utf8Info::length: 13
			b'C', b'o', b'n', b's', b't', b'a', b'n', b't', b'V', b'a', b'l', b'u',
			b'e', //      Utf8Info::value: "ConstantValue"
			// constant_pool: Vec<ConstantContainer>,
			0, 1, // access_flags: u16: 1
			0, 1, // this_class: u16: 1
			0, 2, // super_class: u16: 2
			0, 2, // interfaces_count: u16: 4
			0, 2, 0, 4, // interfaces: Vec<u16>: [1,2,3,4]
			0, 1, // fields_count: u16: 1
			0, 1, //      FieldInfo::access_flags: 1
			0, 1, //      FieldInfo::name_index: 1
			0, 0, //      FieldInfo::descriptor_index: 0
			0, 1, //      FieldInfo::attributes_count: 0
			0, 1, //          ConstantValueAttribute::attribute_name_index: 1
			2, 2, 0, 0, //          ConstantValueAttribute::attribute_length: 8590065664
			1, 1, //          ConstantValueAttribute::constant_value_index: 257
			// field_info: Vec<FieldInfo>,
			0, 1, // methods_count: u16: 1
			0, 1, //      MethodInfo::access_flags: 1
			0, 1, //      MethodInfo::name_index: 1
			0, 0, //      MethodInfo::descriptor_index: 0
			0, 1, //      MethodInfo::attributes_count: 1
			0, 1, //          ConstantValueAttribute::attribute_name_index: 1
			2, 2, 0, 0, //          ConstantValueAttribute::attribute_length: 8590065664
			1, 1, //          ConstantValueAttribute::constant_value_index: 257
			// method_info: Vec<MethodInfo>,
			0, 1, // attributes_count: u16: 1
			0, 1, //      ConstantValueAttribute::attribute_name_index: 1
			2, 2, 0, 0, //      ConstantValueAttribute::attribute_length: 8590065664
			1,
			1, /*      ConstantValueAttribute::constant_value_index: 257
			    * attribute_info: Vec<AttributeContainer> */
		]
	}
}
