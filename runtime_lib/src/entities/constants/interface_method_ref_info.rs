use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct InterfaceMethodRefInfo {
	tag: u8,
	class_index: u16,
	name_and_type_index: u16
}

impl ConstantInfo for InterfaceMethodRefInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl InterfaceMethodRefInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> InterfaceMethodRefInfo {
		InterfaceMethodRefInfo {
			tag: data.pop_u8(),
			class_index: data.pop_u16(),
			name_and_type_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, class_index: u16, name_and_type_index: u16) -> InterfaceMethodRefInfo {
		InterfaceMethodRefInfo {
			tag,
			class_index,
			name_and_type_index
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::interface_method_ref_info::InterfaceMethodRefInfo;
	use crate::entities::constants::test_fixture::model_builder::create_interface_method_ref_info;

	#[test]
	fn interface_method_ref_implements_equality_by_default() {
		let instance1: InterfaceMethodRefInfo = Default::default();
		let instance2: InterfaceMethodRefInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn interface_method_ref_implements_equality_correctly() {
		let instance1: InterfaceMethodRefInfo = create_interface_method_ref_info();
		let instance2: InterfaceMethodRefInfo = create_interface_method_ref_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn interface_method_ref_implements_equality_correctly_when_not_equal() {
		let instance1: InterfaceMethodRefInfo = create_interface_method_ref_info();
		let mut instance2: InterfaceMethodRefInfo = create_interface_method_ref_info();
		instance2.class_index += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn interface_method_ref_implements_json_serialization_correctly() -> Result<()> {
		let instance1: InterfaceMethodRefInfo = create_interface_method_ref_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: InterfaceMethodRefInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
