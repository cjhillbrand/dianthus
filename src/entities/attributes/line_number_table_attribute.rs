use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct LineNumberTableAttribute {
	attribute_name_index: u16,
	attribute_length: u32,
	table_length: u16,
	table: Vec<TableElement>
}

impl AttributeInfo for LineNumberTableAttribute {
	fn name_index(&self) -> &u16 { &self.attribute_name_index }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl LineNumberTableAttribute {
	pub fn new<T: ReadBytes>(data: &mut T) -> LineNumberTableAttribute {
		LineNumberTableAttribute {
			attribute_name_index: data.pop_u16(),
			attribute_length: data.pop_u32(),
			table_length: data.peek_u16(),
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
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use serde_json::Result;

	use crate::entities::attributes::line_number_table_attribute::{LineNumberTableAttribute, TableElement};
	use crate::vecdeque;

	#[test]
	fn table_element_implements_equality_by_default() {
		let instance1: TableElement = Default::default();
		let instance2: TableElement = Default::default();

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn table_element_constructs_expected_struct() {
		let mut data: VecDeque<u8> = get_default_table_elem_vec();
		let result: TableElement = TableElement::new(&mut data);

		let bit16: u16 = 257;
		assert_eq!(bit16, result.start_pc);
		assert_eq!(bit16, result.line_number);
	}

	#[test]
	fn table_element_implements_equality_correctly() {
		let mut data: VecDeque<u8> = get_default_table_elem_vec();
		let mut data2: VecDeque<u8> = data.clone();
		let instance1: TableElement = TableElement::new(&mut data);
		let instance2: TableElement = TableElement::new(&mut data2);

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn table_element_implements_equality_correctly_when_not_equal() {
		let mut data1: VecDeque<u8> = get_default_table_elem_vec();
		let mut data2: VecDeque<u8> = data1.clone();
		data2[0] = data1[0] + 1;
		let instance1: TableElement = TableElement::new(&mut data1);
		let instance2: TableElement = TableElement::new(&mut data2);

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn table_element_implements_json_serialization_correctly() -> Result<()> {
		let mut data: VecDeque<u8> = get_default_table_elem_vec();
		let instance1: TableElement = TableElement::new(&mut data);
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
	fn line_number_table_attribute_constructs_expected_struct() {
		let mut data: VecDeque<u8> = get_default_line_table_vec();
		let result: LineNumberTableAttribute = LineNumberTableAttribute::new(&mut data);

		let bit16: u16 = 257;
		let bit32: u32 = 16843009;
		assert_eq!(bit16, result.attribute_name_index);
		assert_eq!(bit32, result.attribute_length);
		assert_eq!(1, result.table_length);
		assert_eq!(1, result.table.len());

		let mut expected_table_elem_content = get_default_table_elem_vec();
		let expected_table_elem: TableElement = TableElement::new(&mut expected_table_elem_content);
		assert_eq!(expected_table_elem, result.table[0]);
	}

	#[test]
	fn line_number_table_attribute_implements_equality_correctly() {
		let mut data: VecDeque<u8> = get_default_line_table_vec();
		let mut data2: VecDeque<u8> = data.clone();
		let instance1: LineNumberTableAttribute = LineNumberTableAttribute::new(&mut data);
		let instance2: LineNumberTableAttribute = LineNumberTableAttribute::new(&mut data2);

		assert_eq!(instance1, instance2);
	}

	#[test]
	fn line_number_table_attribute_implements_equality_correctly_when_not_equal() {
		let mut data1: VecDeque<u8> = get_default_line_table_vec();
		let mut data2: VecDeque<u8> = data1.clone();
		data2[0] = data1[0] + 1;
		let instance1: LineNumberTableAttribute = LineNumberTableAttribute::new(&mut data1);
		let instance2: LineNumberTableAttribute = LineNumberTableAttribute::new(&mut data2);

		assert_ne!(instance1, instance2);
	}

	#[test]
	fn line_number_table_attribute_implements_json_serialization_correctly() -> Result<()> {
		let mut data: VecDeque<u8> = get_default_line_table_vec();
		let instance1: LineNumberTableAttribute = LineNumberTableAttribute::new(&mut data);
		let instance2 = instance1.clone();

		let json = serde_json::to_string_pretty(&instance1)?;
		let instance3: LineNumberTableAttribute = serde_json::from_str(&json)?;

		assert_eq!(instance2, instance3);
		Ok(())
	}

	fn get_default_line_table_vec() -> VecDeque<u8> { vecdeque![1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1] }

	fn get_default_table_elem_vec() -> VecDeque<u8> { vecdeque![1, 1, 1, 1] }
}
