use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DoubleInfo {
	tag: u8,
	value: f64
}

impl ConstantInfo for DoubleInfo {
	fn tag(&self) -> &u8 { &self.tag }
}

impl Eq for DoubleInfo {}

impl DoubleInfo {
	pub fn new<T: ReadBytes>(data: &mut T) -> DoubleInfo {
		DoubleInfo {
			tag: data.pop_u8(),
			value: {
				let bits: u64 = data.pop_u64();
				DoubleInfo::unsigned_to_float(&bits)
			}
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, value: f64) -> DoubleInfo { DoubleInfo { tag, value } }

	fn unsigned_to_float(bits: &u64) -> f64 {
		let s: f64 = if (bits >> 63) == 0 { 1.0 } else { -1.0 };
		let e = ((bits >> 52) & 0x7ff) as i64;
		let m = if e == 0 {
			(bits & 0xfffffffffffff) << 1
		} else {
			(bits & 0xfffffffffffff) | 0x10000000000000
		};
		let base: f64 = 2.0;
		s * m as f64 * base.powf((e - 1075) as f64)
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::double_info::DoubleInfo;
	use crate::entities::constants::test_fixture::model_builder::create_double_info;

	#[test]
	fn double_info_implements_equality_by_default() {
		let instance1: DoubleInfo = Default::default();
		let instance2: DoubleInfo = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn double_info_implements_equality_correctly() {
		let instance1: DoubleInfo = create_double_info();
		let instance2: DoubleInfo = create_double_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn double_info_implements_equality_correctly_when_not_equal() {
		let instance1: DoubleInfo = create_double_info();
		let mut instance2: DoubleInfo = create_double_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn double_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: DoubleInfo = create_double_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: DoubleInfo = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
