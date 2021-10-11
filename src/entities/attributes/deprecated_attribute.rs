use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct DeprecatedAttribute {
	attribute_name_index: u16,
	attribute_length: u32
}

impl AttributeInfo for DeprecatedAttribute {
	fn name_index(&self) -> &u16 { &self.attribute_name_index }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl DeprecatedAttribute {
	pub fn new<T: ReadBytes>(data: &mut T) -> DeprecatedAttribute {
		DeprecatedAttribute {
			attribute_name_index: data.pop_u16(),
			attribute_length: data.pop_u32()
		}
	}
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use serde_json::Result;

	use crate::entities::attributes::deprecated_attribute::DeprecatedAttribute;
	use crate::vecdeque;

	#[test]
	fn deprecated_attribute_implements_equality_by_default() {
		let instance1: DeprecatedAttribute = Default::default();
		let instance2: DeprecatedAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn deprecated_attribute_constructs_expected_struct() {
		let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
		let result: DeprecatedAttribute = DeprecatedAttribute::new(&mut data);

		let bit16: u16 = 257;
		let bit32: u32 = 16843009;
		assert_eq!(bit16, result.attribute_name_index);
		assert_eq!(bit32, result.attribute_length);
	}

	#[test]
	fn deprecated_attribute_implements_equality_correctly() {
		let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
		let mut data2: VecDeque<u8> = data.clone();
		let instance1: DeprecatedAttribute = DeprecatedAttribute::new(&mut data);
		let instance2: DeprecatedAttribute = DeprecatedAttribute::new(&mut data2);

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn deprecated_attribute_implements_equality_correctly_when_not_equal() {
		let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
		let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
		let instance1: DeprecatedAttribute = DeprecatedAttribute::new(&mut data1);
		let instance2: DeprecatedAttribute = DeprecatedAttribute::new(&mut data2);

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn deprecated_attribute_implements_json_serialization_correctly() -> Result<()> {
		let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
		let instance1: DeprecatedAttribute = DeprecatedAttribute::new(&mut data);
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: DeprecatedAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
