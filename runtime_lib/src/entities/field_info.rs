use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::attribute_factory::get_attribute_container;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct FieldInfo {
	access_flags: u16,
	name: String,
	descriptor: String,
	attributes: Vec<AttributeContainer>
}

impl FieldInfo {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> FieldInfo {
		FieldInfo {
			access_flags: data.pop_u16(),
			name: constant_pool[data.pop_u16() as usize].get_string(),
			descriptor: constant_pool[data.pop_u16() as usize].get_string(),
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

	#[cfg(test)]
	pub(crate) fn new_test_model(
		access_flags: u16, name: String, descriptor: String, attributes: Vec<AttributeContainer>
	) -> FieldInfo {
		FieldInfo {
			access_flags,
			name,
			descriptor,
			attributes
		}
	}

	pub fn get_name(&self) -> &str { &self.name }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::field_info::FieldInfo;
	use crate::entities::test_fixture::model_builder::create_field;

	#[test]
	fn field_info_implements_equality_by_default() {
		let instance1: FieldInfo = Default::default();
		let instance2: FieldInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn field_info_implements_equality_correctly() {
		let instance1: FieldInfo = create_field();
		let instance2: FieldInfo = create_field();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn field_info_implements_equality_correctly_when_not_equal() {
		let instance1: FieldInfo = create_field();
		let mut instance2: FieldInfo = create_field();
		instance2.access_flags += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn field_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: FieldInfo = create_field();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: FieldInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
