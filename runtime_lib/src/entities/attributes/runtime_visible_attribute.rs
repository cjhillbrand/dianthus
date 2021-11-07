use crate::entities::attributes::annotation::Annotation;
use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Default)]
pub struct RuntimeVisibleAttribute {
	attribute_name: String,
	attribute_length: u32,
	annotations: Vec<Annotation>
}

impl AttributeInfo for RuntimeVisibleAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl RuntimeVisibleAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> RuntimeVisibleAttribute {
		RuntimeVisibleAttribute {
			attribute_name: {
				let index: usize = data.pop_u16() as usize;
				constant_pool[index].get_string()
			},
			attribute_length: data.pop_u32(),
			annotations: {
				let count: u16 = data.pop_u16();
				let mut result: Vec<Annotation> = Vec::new();
				for _i in 0..count {
					result.push(Annotation::new(data))
				}

				result
			}
		}
	}
}
