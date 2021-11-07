use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Default)]
pub struct InnerClassAttribute {
	attribute_name: String,
	attribute_length: u32,
	class_references: Vec<ClassReference>
}

impl AttributeInfo for InnerClassAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl InnerClassAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> InnerClassAttribute {
		InnerClassAttribute {
			attribute_name: constant_pool[data.pop_u16() as usize].get_string(),
			attribute_length: data.pop_u32(),
			class_references: {
				let count: usize = data.pop_u16() as usize;
				let mut result: Vec<ClassReference> = Vec::new();
				for _i in 0..count {
					result.push(ClassReference::new(data));
				}

				result
			}
		}
	}
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Default)]
struct ClassReference {
	inner_class_info_index: u16,
	outer_class_info_index: u16,
	inner_name_index: u16,
	inner_class_access_flags: u16
}

impl ClassReference {
	pub fn new<T: ReadBytes>(data: &mut T) -> ClassReference {
		ClassReference {
			inner_class_info_index: data.pop_u16(),
			outer_class_info_index: data.pop_u16(),
			inner_name_index: data.pop_u16(),
			inner_class_access_flags: data.pop_u16()
		}
	}
}
