use crate::entities::attributes::element_value::ElementValue;
use crate::entities::read_bytes::ReadBytes;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Default)]
pub struct Annotation {
	type_index: u16,
	element_pairs: Vec<ElementValuePair>
}

impl Annotation {
	pub fn new<T: ReadBytes>(data: &mut T) -> Annotation {
		Annotation {
			type_index: data.pop_u16(),
			element_pairs: {
				let count: u16 = data.pop_u16();
				let mut result: Vec<ElementValuePair> = Vec::new();
				for _i in 0..count {
					result.push(ElementValuePair::new(data))
				}

				result
			}
		}
	}
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Default)]
struct ElementValuePair {
	element_name_index: u16,
	element: ElementValue
}

impl ElementValuePair {
	pub fn new<T: ReadBytes>(data: &mut T) -> ElementValuePair {
		ElementValuePair {
			element_name_index: data.pop_u16(),
			element: ElementValue::new(data)
		}
	}
}
