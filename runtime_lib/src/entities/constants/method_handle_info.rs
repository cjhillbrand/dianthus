use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct MethodHandleInfo {
	tag: u8,
	reference_kind: u8,
	reference_index: u16
}

impl ConstantInfo for MethodHandleInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl MethodHandleInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> MethodHandleInfo {
		MethodHandleInfo {
			tag: data.pop_u8(),
			reference_kind: data.pop_u8(),
			reference_index: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, reference_kind: u8, reference_index: u16) -> MethodHandleInfo {
		MethodHandleInfo {
			tag,
			reference_kind,
			reference_index
		}
	}
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use serde_json::Result;

	use crate::entities::constants::method_handle_info::MethodHandleInfo;
	use crate::vecdeque;

	#[test]
	fn method_handle_info_implements_equality_by_default() {
		let instance1: MethodHandleInfo = Default::default();
		let instance2: MethodHandleInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_handle_info_constructs_expected_struct() {
		let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
		let result: MethodHandleInfo = MethodHandleInfo::new(&mut data);

		let bit8: u8 = 1;
		let bit16: u16 = 257;
		assert_eq!(bit8, result.tag);
		assert_eq!(bit8, result.reference_kind);
		assert_eq!(bit16, result.reference_index);
	}

	#[test]
	fn method_handle_info_implements_equality_correctly() {
		let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
		let mut data2: VecDeque<u8> = data.clone();
		let instance1: MethodHandleInfo = MethodHandleInfo::new(&mut data);
		let instance2: MethodHandleInfo = MethodHandleInfo::new(&mut data2);

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn method_handle_info_implements_equality_correctly_when_not_equal() {
		let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
		let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
		let instance1: MethodHandleInfo = MethodHandleInfo::new(&mut data1);
		let instance2: MethodHandleInfo = MethodHandleInfo::new(&mut data2);

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn method_handle_info_implements_json_serialization_correctly() -> Result<()> {
		let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
		let instance1: MethodHandleInfo = MethodHandleInfo::new(&mut data);
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: MethodHandleInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
