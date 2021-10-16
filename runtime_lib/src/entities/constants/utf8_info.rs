use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Utf8Info {
	tag: u8,
	value: String
}

impl ConstantInfo for Utf8Info {
	fn tag(&self) -> &u8 { &self.tag }
}

impl Utf8Info {
	pub fn new<T: ReadBytes>(data: &mut T) -> Utf8Info {
		Utf8Info {
			tag: data.pop_u8(),
			value: {
				let length: usize = data.pop_u16() as usize;
				match std::str::from_utf8(&data.pop_vec(length)) {
					Ok(string) => string.to_string(),
					Err(err) => {
						panic!("Could not parse string value {}", err)
					}
				}
			}
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(tag: u8, value: String) -> Utf8Info { Utf8Info { tag, value } }

	pub fn get_string(&self) -> &str { &self.value }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::constants::test_fixture::model_builder::create_utf8_info;
	use crate::entities::constants::utf8_info::Utf8Info;

	#[test]
	fn utf8_info_implements_equality_by_default() {
		let instance1: Utf8Info = Default::default();
		let instance2: Utf8Info = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn utf8_info_implements_equality_correctly() {
		let instance1: Utf8Info = create_utf8_info();
		let instance2: Utf8Info = create_utf8_info();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn utf8_info_implements_equality_correctly_when_not_equal() {
		let instance1: Utf8Info = create_utf8_info();
		let mut instance2: Utf8Info = create_utf8_info();
		instance2.tag += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn utf8_info_implements_json_serialization_correctly() -> Result<()> {
		let instance1: Utf8Info = create_utf8_info();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: Utf8Info = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
