use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo {
	tag: u8,
	name_index: u16
}

impl ConstantInfo for ClassInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl ClassInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> ClassInfo {
		ClassInfo {
			tag: data.pop_u8(),
			name_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, name_index: u16) -> ClassInfo { ClassInfo { tag, name_index } }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::class_info::ClassInfo;
	use crate::entities::constants::test_fixture::model_builder::create_class_info;

	#[test]
	fn class_info_implements_equality_by_default() {
		let instance1: ClassInfo = Default::default();
		let instance2: ClassInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn class_info_implements_equality_correctly() {
		let instance1: ClassInfo = create_class_info();
		let instance2: ClassInfo = create_class_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn class_info_implements_equality_correctly_when_not_equal() {
		let instance1: ClassInfo = create_class_info();
		let mut instance2: ClassInfo = create_class_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn class_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: ClassInfo = create_class_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: ClassInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
