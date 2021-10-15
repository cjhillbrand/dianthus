use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::attribute_factory::get_attribute_container;
use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodInfo {
	access_flags: u16,
	name: String,
	descriptor_index: u16,
	attributes_count: u16,
	attributes: Vec<AttributeContainer>
}

impl MethodInfo {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> MethodInfo {
		MethodInfo {
			access_flags: data.pop_u16(),
			name: constant_pool[data.pop_u16() as usize].get_string(),
			descriptor_index: data.pop_u16(),
			attributes_count: data.peek_u16(),
			attributes: {
				let count = data.pop_u16();
				let mut result: Vec<AttributeContainer> = Vec::new();
				for _i in 0..count {
					result.push(get_attribute_container(data, constant_pool));
				}

				result
			}
		}
	}

	pub fn get_attributes(&self, name: &str) -> Vec<&AttributeContainer> {
		self.attributes.iter().filter(|a| a.name() == name).collect()
	}

	pub fn get_name(&self) -> &str { &self.name }
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use serde_json::Result;

	use crate::entities::constants::constant_container::ConstantContainer;
	use crate::entities::constants::utf8_info::Utf8Info;
	use crate::entities::method_info::MethodInfo;
	use crate::vecdeque;

	#[test]
	fn method_info_implements_equality_by_default() {
		let instance1: MethodInfo = Default::default();
		let instance2: MethodInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_info_implements_equality_correctly() {
		let mut data: VecDeque<u8> = get_default_vec();
		let mut data2: VecDeque<u8> = data.clone();
		let constant_pool = get_default_cp();
		let instance1: MethodInfo = MethodInfo::new(&mut data, &constant_pool);
		let instance2: MethodInfo = MethodInfo::new(&mut data2, &constant_pool);

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_info_implements_equality_correctly_when_not_equal() {
		let mut data: VecDeque<u8> = get_default_vec();
		let mut data2: VecDeque<u8> = data.clone();
		data2[0] = data[0] + 1;
		let constant_pool = get_default_cp();
		let instance1: MethodInfo = MethodInfo::new(&mut data, &constant_pool);
		let instance2: MethodInfo = MethodInfo::new(&mut data2, &constant_pool);

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn method_info_implements_json_serialization_correctly() -> Result<()> {
		let mut data: VecDeque<u8> = get_default_vec();
		let constant_pool = get_default_cp();

		let instance1: MethodInfo = MethodInfo::new(&mut data, &constant_pool);
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: MethodInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}

	fn get_default_vec() -> VecDeque<u8> {
		vecdeque![
			0, 1, // access_flags
			0, 123, // name_index
			0, 5, // descriptor_index
			0, 1, // attributes_count
			0, 0, // ConstantValueAttribute::attribute_name_index
			1, 1, 1, 1, // ConstantValueAttribute::attribute_length
			0, 2 // ConstantValueAttribute::constant_value_index
		]
	}

	fn get_default_cp() -> Vec<ConstantContainer> {
		let attr_name: &str = "ConstantValue";
		let mut attr_contents = vecdeque![
			0, // tag
			0, 13, // length
		];
		attr_contents.extend(attr_name.as_bytes().iter().copied());

		vec![ConstantContainer::Utf8Info(Utf8Info::new(&mut attr_contents))]
	}
}
