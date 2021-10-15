use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ConstantValueAttribute {
	attribute_name: String,
	attribute_length: u32,
	constant_value_index: u16
}

impl AttributeInfo for ConstantValueAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl ConstantValueAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> ConstantValueAttribute {
		ConstantValueAttribute {
			attribute_name: constant_pool[data.pop_u16() as usize].get_string(),
			attribute_length: data.pop_u32(),
			constant_value_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		attribute_name: String, attribute_length: u32, constant_value_index: u16
	) -> ConstantValueAttribute {
		ConstantValueAttribute {
			attribute_name,
			attribute_length,
			constant_value_index
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
	use crate::entities::attributes::test_fixture::model_builder::create_constant_value;

	#[test]
	fn constant_value_attribute_implements_equality_by_default() {
		let instance1: ConstantValueAttribute = Default::default();
		let instance2: ConstantValueAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn constant_value_attribute_implements_equality_correctly() {
		let instance1: ConstantValueAttribute = create_constant_value();
		let instance2: ConstantValueAttribute = create_constant_value();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn constant_value_attribute_implements_equality_correctly_when_not_equal() {
		let instance1: ConstantValueAttribute = create_constant_value();
		let mut instance2: ConstantValueAttribute = create_constant_value();
		instance2.constant_value_index += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn constant_value_attribute_implements_json_serialization_correctly() -> Result<()> {
		let instance1: ConstantValueAttribute = create_constant_value();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: ConstantValueAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
