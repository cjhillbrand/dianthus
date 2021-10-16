use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct InvokeDynamicInfo {
	tag: u8,
	bootstrap_method_attr_index: u16,
	name_and_type_index: u16
}

impl ConstantInfo for InvokeDynamicInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl InvokeDynamicInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> InvokeDynamicInfo {
		InvokeDynamicInfo {
			tag: data.pop_u8(),
			bootstrap_method_attr_index: data.pop_u16(),
			name_and_type_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		tag: u8, bootstrap_method_attr_index: u16, name_and_type_index: u16
	) -> InvokeDynamicInfo {
		InvokeDynamicInfo {
			tag,
			bootstrap_method_attr_index,
			name_and_type_index
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::invoke_dynamic_info::InvokeDynamicInfo;
	use crate::entities::constants::test_fixture::model_builder::create_invoke_dynamic_info;

	#[test]
	fn invoke_dynamic_info_implements_equality_by_default() {
		let instance1: InvokeDynamicInfo = Default::default();
		let instance2: InvokeDynamicInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn invoke_dynamic_info_implements_equality_correctly() {
		let instance1: InvokeDynamicInfo = create_invoke_dynamic_info();
		let instance2: InvokeDynamicInfo = create_invoke_dynamic_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn invoke_dynamic_info_implements_equality_correctly_when_not_equal() {
		let instance1: InvokeDynamicInfo = create_invoke_dynamic_info();
		let mut instance2: InvokeDynamicInfo = create_invoke_dynamic_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn invoke_dynamic_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: InvokeDynamicInfo = create_invoke_dynamic_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: InvokeDynamicInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
