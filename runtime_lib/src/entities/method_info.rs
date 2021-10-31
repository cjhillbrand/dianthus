use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::attribute_factory::get_attribute_container;
use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::attributes::code_attribute::CodeAttribute;
use crate::entities::attributes::constants::CODE;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodInfo {
	access_flags: u16,
	name: String,
	descriptor: String,
	attributes: Vec<AttributeContainer>
}

impl MethodInfo {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> MethodInfo {
		MethodInfo {
			access_flags: data.pop_u16(),
			name: constant_pool[data.pop_u16() as usize].get_string(),
			descriptor: constant_pool[data.pop_u16() as usize].get_string(),
			attributes: {
				let count = data.pop_u16();
				let mut result: Vec<AttributeContainer> = Vec::new();
				for _i in 0..count {
					let temp: AttributeContainer = get_attribute_container(data, constant_pool);
					result.push(temp);
				}

				result
			}
		}
	}

	pub fn derive_code_attribute(&self) -> &CodeAttribute {
		let code_attrs: Vec<&AttributeContainer> = self.get_attributes(CODE);
		assert!(
			!(code_attrs.len() > 1),
			"There are more than 1 code attributes in the method: {}",
			self.get_name()
		);

		assert!(
			!code_attrs.is_empty(),
			"there are no code attributes in the method: {}",
			self.get_name()
		);

		match &code_attrs[0] {
			AttributeContainer::CodeAttribute(v) => v,
			_ => panic!("Attribute returned is not a code attribute.")
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		access_flags: u16, name: String, descriptor: String, attributes: Vec<AttributeContainer>
	) -> MethodInfo {
		MethodInfo {
			access_flags,
			name,
			descriptor,
			attributes
		}
	}

	pub fn get_attributes(&self, name: &str) -> Vec<&AttributeContainer> {
		self.attributes.iter().filter(|a| a.name() == name).collect()
	}

	pub fn get_name(&self) -> &str { &self.name }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::method_info::MethodInfo;
	use crate::entities::test_fixture::model_builder::create_method;

	#[test]
	fn method_info_implements_equality_by_default() {
		let instance1: MethodInfo = Default::default();
		let instance2: MethodInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_info_implements_equality_correctly() {
		let instance1: MethodInfo = create_method();
		let instance2: MethodInfo = create_method();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_info_implements_equality_correctly_when_not_equal() {
		let instance1: MethodInfo = create_method();
		let mut instance2: MethodInfo = create_method();
		instance2.access_flags += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn method_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: MethodInfo = create_method();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: MethodInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
