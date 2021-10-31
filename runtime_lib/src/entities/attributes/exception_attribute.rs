use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ExceptionAttribute {
	attribute_name: String,
	attribute_length: u32,
	number_of_exceptions: u16,
	exception_index_table: Vec<u16>
}

impl AttributeInfo for ExceptionAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl ExceptionAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> ExceptionAttribute {
		ExceptionAttribute {
			attribute_name: constant_pool[data.pop_u16() as usize].get_string(),
			attribute_length: data.pop_u32(),
			number_of_exceptions: data.peek_u16(),
			exception_index_table: {
				let num_exceptions: u16 = data.pop_u16();
				let mut exceptions: Vec<u16> = Vec::new();
				for _i in 0..num_exceptions {
					exceptions.push(data.pop_u16());
				}
				exceptions
			}
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		attribute_name: String, attribute_length: u32, number_of_exceptions: u16, exception_index_table: u16
	) -> ExceptionAttribute {
		ExceptionAttribute {
			attribute_name,
			attribute_length,
			number_of_exceptions,
			exception_index_table
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::attributes::exception_attribute::ExceptionAttribute;
	use crate::entities::attributes::test_fixture::model_builder::create_exception;

	#[test]
	fn exception_attribute_implements_equality_by_default() {
		let instance1: ExceptionAttribute = Default::default();
		let instance2: ExceptionAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn exception_attribute_implements_equality_correctly() {
		let instance1: ExceptionAttribute = create_exception();
		let instance2: ExceptionAttribute = create_exception();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn exception_attribute_implements_equality_correctly_when_not_equal() {
		let instance1: ExceptionAttribute = create_exception();
		let mut instance2: ExceptionAttribute = create_exception();
		instance2.attribute_length += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn exception_attribute_implements_json_serialization_correctly() -> Result<()> {
		let instance1: ExceptionAttribute = create_exception();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: ExceptionAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
