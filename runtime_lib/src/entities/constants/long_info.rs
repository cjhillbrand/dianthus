use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct LongInfo {
	tag: u8,
	value: i64
}

impl ConstantInfo for LongInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl LongInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> LongInfo {
		LongInfo {
			tag: data.pop_u8(),
			value: data.pop_u64() as i64
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, value: i64) -> LongInfo { LongInfo { tag, value } }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::long_info::LongInfo;
	use crate::entities::constants::test_fixture::model_builder::create_long_info;

	#[test]
	fn long_info_implements_equality_by_default() {
		let instance1: LongInfo = Default::default();
		let instance2: LongInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn long_info_implements_equality_correctly() {
		let instance1: LongInfo = create_long_info();
		let instance2: LongInfo = create_long_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn long_info_implements_equality_correctly_when_not_equal() {
		let instance1: LongInfo = create_long_info();
		let mut instance2: LongInfo = create_long_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn long_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: LongInfo = create_long_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: LongInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
