use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct DeprecatedAttribute {
	attribute_name: String,
	attribute_length: u32
}

impl AttributeInfo for DeprecatedAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl DeprecatedAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> DeprecatedAttribute {
		DeprecatedAttribute {
			attribute_name: constant_pool[data.pop_u16() as usize].get_string(),
			attribute_length: data.pop_u32()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(attribute_name: String, attribute_length: u32) -> DeprecatedAttribute {
		DeprecatedAttribute {
			attribute_name,
			attribute_length
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
	use crate::entities::attributes::test_fixture::model_builder::create_deprecated;

	#[test]
	fn deprecated_attribute_implements_equality_by_default() {
		let instance1: DeprecatedAttribute = Default::default();
		let instance2: DeprecatedAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn deprecated_attribute_implements_equality_correctly() {
		let instance1: DeprecatedAttribute = create_deprecated();
		let instance2: DeprecatedAttribute = create_deprecated();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn deprecated_attribute_implements_equality_correctly_when_not_equal() {
		let instance1: DeprecatedAttribute = create_deprecated();
		let mut instance2: DeprecatedAttribute = create_deprecated();
		instance2.attribute_length += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn deprecated_attribute_implements_json_serialization_correctly() -> Result<()> {
		let instance1: DeprecatedAttribute = create_deprecated();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: DeprecatedAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
