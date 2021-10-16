use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodTypeInfo {
	tag: u8,
	descriptor_index: u16
}

impl ConstantInfo for MethodTypeInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl MethodTypeInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> MethodTypeInfo {
		MethodTypeInfo {
			tag: data.pop_u8(),
			descriptor_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, descriptor_index: u16) -> MethodTypeInfo {
		MethodTypeInfo { tag, descriptor_index }
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::method_type_info::MethodTypeInfo;
	use crate::entities::constants::test_fixture::model_builder::create_method_type_info;

	#[test]
	fn method_type_info_implements_equality_by_default() {
		let instance1: MethodTypeInfo = Default::default();
		let instance2: MethodTypeInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_type_info_implements_equality_correctly() {
		let instance1: MethodTypeInfo = create_method_type_info();
		let instance2: MethodTypeInfo = create_method_type_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_type_info_implements_equality_correctly_when_not_equal() {
		let instance1: MethodTypeInfo = create_method_type_info();
		let mut instance2: MethodTypeInfo = create_method_type_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn method_type_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: MethodTypeInfo = create_method_type_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: MethodTypeInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
