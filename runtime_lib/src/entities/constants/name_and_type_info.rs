use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct NameAndTypeInfo {
	tag: u8,
	name_index: u16,
	descriptor_index: u16
}

impl ConstantInfo for NameAndTypeInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl NameAndTypeInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> NameAndTypeInfo {
		NameAndTypeInfo {
			tag: data.pop_u8(),
			name_index: data.pop_u16(),
			descriptor_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, name_index: u16, descriptor_index: u16) -> NameAndTypeInfo {
		NameAndTypeInfo {
			tag,
			name_index,
			descriptor_index
		}
	}

	pub fn get_name_index(&self) -> u16
	{
		self.name_index
	}

	pub fn get_descriptor_index(&self) -> u16
	{
		self.descriptor_index
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::name_and_type_info::NameAndTypeInfo;
	use crate::entities::constants::test_fixture::model_builder::create_name_and_type_info;

	#[test]
	fn name_and_type_info_implements_equality_by_default() {
		let instance1: NameAndTypeInfo = Default::default();
		let instance2: NameAndTypeInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn name_and_type_info_implements_equality_correctly() {
		let instance1: NameAndTypeInfo = create_name_and_type_info();
		let instance2: NameAndTypeInfo = create_name_and_type_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn name_and_type_info_implements_equality_correctly_when_not_equal() {
		let instance1: NameAndTypeInfo = create_name_and_type_info();
		let mut instance2: NameAndTypeInfo = create_name_and_type_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn name_and_type_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: NameAndTypeInfo = create_name_and_type_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: NameAndTypeInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
