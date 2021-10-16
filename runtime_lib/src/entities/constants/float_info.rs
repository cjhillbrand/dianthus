use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct FloatInfo {
	tag: u8,
	value: f32
}

impl ConstantInfo for FloatInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl Eq for FloatInfo {}

impl FloatInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> FloatInfo {
		FloatInfo {
			tag: data.pop_u8(),
			value: {
				let bits = data.pop_u32();
				FloatInfo::unsigned_to_float(&bits)
			}
		}
	}

	fn unsigned_to_float(bits: &u32) -> f32 {
		let s: f32 = if bits & 0x80000000 == 0 { 1. } else { -1. };
		let e: i32 = ((bits >> 23) & 0x000000ff) as i32; // & gets rid of leading sign bit
		let m = if e == 0 {
			(bits & 0x7fffff) << 1
		} else {
			(bits & 0x7fffff) | 0x800000
		};
		let base: f32 = 2.0;
		s * m as f32 * base.powf((e - 150) as f32)
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, value: f32) -> FloatInfo { FloatInfo { tag, value } }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::float_info::FloatInfo;
	use crate::entities::constants::test_fixture::model_builder::create_float_info;

	#[test]
	fn float_info_implements_equality_by_default() {
		let instance1: FloatInfo = Default::default();
		let instance2: FloatInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn float_info_implements_equality_correctly() {
		let instance1: FloatInfo = create_float_info();
		let instance2: FloatInfo = create_float_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn float_info_implements_equality_correctly_when_not_equal() {
		let instance1: FloatInfo = create_float_info();
		let mut instance2: FloatInfo = create_float_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn float_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: FloatInfo = create_float_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: FloatInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
