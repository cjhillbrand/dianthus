use std::collections::HashMap;

use jvm_value::JvmValue;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::field_info::FieldInfo;

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
		let fields: &FieldInfo = class.get_fields();
		let obj: JvmObject = JvmObject::new();
		for field in fields.iter()
		{
			obj.fields.insert(field.get_name(), JvmValue::PlaceHolder);
		}

		obj
	}

	pub fn get_value(&self, name: &str) -> &JvmValue
	{
		self.fields.get(name)
	}
}