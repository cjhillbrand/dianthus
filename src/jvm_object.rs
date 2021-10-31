use std::collections::HashMap;

use jvm_value::JvmValue;
use runtime_lib::entities::class_struct::ClassStruct;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct JvmObject {
	fields: HashMap<String, JvmValue>
}

impl JvmObject {
	pub fn new() -> JvmObject
	{
		JvmObject { fields: HashMap::new() }
	}

	pub fn build_obj(class: &ClassStruct) -> JvmObject
	{
		let fields: Vec<&String> = class.get_field_names();
		let mut obj: JvmObject = JvmObject::new();
		for field in fields.iter()
		{
			obj.fields.insert(field.to_string(), JvmValue::PlaceHolder);
		}

		obj
	}

	pub fn get_value(&self, name: &str) -> &JvmValue
	{
		match self.fields.get(name)
		{
			Some(v) => v,
			None => panic!("Could not find field: {}", name)
		}
	}

	pub fn set_value(&mut self, name: &str, value: JvmValue)
	{
		self.fields.insert(name.to_string(), value);
	}
}