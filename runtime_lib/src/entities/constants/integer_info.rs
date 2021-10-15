use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct IntegerInfo {
	tag: u8,
	value: i32
}

impl ConstantInfo for IntegerInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl IntegerInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> IntegerInfo {
		IntegerInfo {
			tag: data.pop_u8(),
			value: data.pop_u32() as i32
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, value: i32) -> IntegerInfo { IntegerInfo { tag, value } }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::integer_info::IntegerInfo;
	use crate::entities::constants::test_fixture::model_builder::create_integer_info;

	#[test]
	fn class_info_implements_equality_by_default() {
		let instance1: IntegerInfo = Default::default();
		let instance2: IntegerInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn class_info_implements_equality_correctly() {
		let instance1: IntegerInfo = create_integer_info();
		let instance2: IntegerInfo = create_integer_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn class_info_implements_equality_correctly_when_not_equal() {
		let instance1: IntegerInfo = create_integer_info();
		let mut instance2: IntegerInfo = create_integer_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn class_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: IntegerInfo = create_integer_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: IntegerInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
