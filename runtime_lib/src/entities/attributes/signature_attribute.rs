use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct SignatureAttribute {
	attribute_name: String,
	attribute_length: u32,
	signature_index: u16
}

impl AttributeInfo for SignatureAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl SignatureAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> SignatureAttribute {
		SignatureAttribute {
			attribute_name: constant_pool[data.pop_u16() as usize].get_string(),
			attribute_length: data.pop_u32(),
			signature_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		attribute_name: String, attribute_length: u32, signature_index: u16
	) -> SignatureAttribute {
		SignatureAttribute {
			attribute_name,
			attribute_length,
			signature_index
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::attributes::signature_attribute::SignatureAttribute;
	use crate::entities::attributes::test_fixture::model_builder::create_signature;

	#[test]
	fn signature_attribute_implements_equality_by_default() {
		let instance1: SignatureAttribute = Default::default();
		let instance2: SignatureAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn signature_attribute_implements_equality_correctly() {
		let instance1: SignatureAttribute = create_signature();
		let instance2: SignatureAttribute = create_signature();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn signature_attribute_implements_equality_correctly_when_not_equal() {
		let instance1: SignatureAttribute = create_signature();
		let mut instance2: SignatureAttribute = create_signature();
		instance2.signature_index += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn signature_attribute_implements_json_serialization_correctly() -> Result<()> {
		let instance1: SignatureAttribute = create_signature();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: SignatureAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
