use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct LineNumberTableAttribute {
	attribute_name: String,
	attribute_length: u32,
	table: Vec<TableElement>
}

impl AttributeInfo for LineNumberTableAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl LineNumberTableAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> LineNumberTableAttribute {
		LineNumberTableAttribute {
			attribute_name: constant_pool[data.pop_u16() as usize].get_string(),
			attribute_length: data.pop_u32(),
			table: {
				let length: u16 = data.pop_u16();
				let mut result: Vec<TableElement> = Vec::new();
				for _i in 0..length {
					result.push(TableElement::new(data));
				}
				result
			}
		}
	}

	#[cfg(test)]
	pub(crate) fn new_test_model(
		attribute_name: String, attribute_length: u32, table: Vec<TableElement>
	) -> LineNumberTableAttribute {
		LineNumberTableAttribute {
			attribute_name,
			attribute_length,
			table
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct TableElement {
	start_pc: u16,
	line_number: u16
}

impl TableElement {
	pub fn new<T: ReadBytes>(data: &mut T) -> TableElement {
		TableElement {
			start_pc: data.pop_u16(),
			line_number: data.pop_u16()
		}
	}

	#[cfg(test)]
	pub fn new_test_model(start_pc: u16, line_number: u16) -> TableElement { TableElement { start_pc, line_number } }
}

#[cfg(test)]
mod tests {
	use serde_json::Result;

	use crate::entities::attributes::line_number_table_attribute::{LineNumberTableAttribute, TableElement};
	use crate::entities::attributes::test_fixture::model_builder::{create_line_number_table, create_table_element};

	#[test]
	fn table_element_implements_equality_by_default() {
		let instance1: TableElement = Default::default();
		let instance2: TableElement = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn table_element_implements_equality_correctly() {
		let instance1: TableElement = create_table_element();
		let instance2: TableElement = create_table_element();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn table_element_implements_equality_correctly_when_not_equal() {
		let instance1: TableElement = create_table_element();
		let mut instance2: TableElement = create_table_element();
		instance2.line_number += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn table_element_implements_json_serialization_correctly() -> Result<()> {
		let instance1: TableElement = create_table_element();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: TableElement = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}

	#[test]
	fn line_number_table_attribute_implements_equality_by_default() {
		let instance1: LineNumberTableAttribute = Default::default();
		let instance2: LineNumberTableAttribute = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn line_number_table_attribute_implements_equality_correctly() {
		let instance1: LineNumberTableAttribute = create_line_number_table();
		let instance2: LineNumberTableAttribute = create_line_number_table();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn line_number_table_attribute_implements_equality_correctly_when_not_equal() {
		let instance1: LineNumberTableAttribute = create_line_number_table();
		let mut instance2: LineNumberTableAttribute = create_line_number_table();
		instance2.attribute_length += 1;

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn line_number_table_attribute_implements_json_serialization_correctly() -> Result<()> {
		let instance1: LineNumberTableAttribute = create_line_number_table();
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: LineNumberTableAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}
}
