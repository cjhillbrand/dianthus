use std::collections::HashMap;

use jvm_value::JvmValue;
use runtime_lib::entities::class_struct::ClassStruct;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum JvmObject {
	Object(HashMap<String, JvmValue>),
	Array(Vec<JvmValue>)
}

impl JvmObject {
	pub fn build_obj(class: &ClassStruct) -> JvmObject {
		let fields: Vec<&String> = class.get_field_names();
		let mut obj: HashMap<String, JvmValue> = HashMap::new();
		for field in fields.iter() {
			obj.insert(field.to_string(), JvmValue::PlaceHolder);
		}

		JvmObject::Object(obj)
	}

	pub fn build_array(array: Vec<JvmValue>) -> JvmObject { JvmObject::Array(array) }

	pub fn get_value(&self, name: &str) -> &JvmValue {
		match &self {
			JvmObject::Object(obj) => match obj.get(name) {
				Some(v) => v,
				None => panic!("Could not find field: {}", name)
			},
			JvmObject::Array(_) => {
				panic!("Can not retrieve field of object that is an array.")
			}
		}
	}

	pub fn set_value(&mut self, name: &str, value: JvmValue) {
		match self {
			JvmObject::Object(obj) => {
				obj.insert(name.to_string(), value);
			}
			JvmObject::Array(_) => {
				panic!("Can not set field of object that is of type array.")
			}
		}
	}
}
