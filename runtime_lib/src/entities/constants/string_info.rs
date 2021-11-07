use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StringInfo {
	tag: u8,
	string_index: u16
}

impl ConstantInfo for StringInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl StringInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> StringInfo {
		StringInfo {
			tag: data.pop_u8(),
			string_index: data.pop_u16()
		}
	}

	pub fn get_string_index(&self) -> u16 { self.string_index }

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, string_index: u16) -> StringInfo { StringInfo { tag, string_index } }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::string_info::StringInfo;
	use crate::entities::constants::test_fixture::model_builder::create_string_info;

	#[test]
	fn string_info_implements_equality_by_default() {
		let instance1: StringInfo = Default::default();
		let instance2: StringInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn string_info_implements_equality_correctly() {
		let instance1: StringInfo = create_string_info();
		let instance2: StringInfo = create_string_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn string_info_implements_equality_correctly_when_not_equal() {
		let instance1: StringInfo = create_string_info();
		let mut instance2: StringInfo = create_string_info();
		instance2.string_index += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn string_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: StringInfo = create_string_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: StringInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
