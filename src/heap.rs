use std::collections::HashMap;

use jvm_object::JvmObject;
use rand::{thread_rng, Rng};
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::method_info::MethodInfo;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Heap {
	objects: HashMap<u64, JvmObject>,
	static_objects: HashMap<String, JvmObject>
}

impl Heap {
	pub fn new() -> Heap {
		Heap {
			objects: HashMap::new(),
			static_objects: HashMap::new()
		}
	}

	pub fn get_value(&self, reference: u64) -> &JvmObject {
		match self.objects.get(&reference) {
			Some(v) => v,
			None => panic!("could not find reference {}", reference)
		}
	}

	pub fn get_static_value(&self, class_name: &str) -> &JvmObject
	{
		match self.static_objects.get(class_name) {
			Some(v) => v,
			None => panic!("could not find reference {}", reference)
		}
	}

	pub fn alloc(&mut self, obj: JvmObject) -> u64 {
		let mut rng = thread_rng();
		let reference: u64 = rng.gen();
		self.objects.insert(reference, obj);
		reference
	}

	pub fn alloc_static(&mut self, obj: JvmObject, name: &str)
	{
		self.static_objects.insert(name, obj);
	}

	pub fn dealloc(&mut self, reference: u64) { self.objects.remove(&reference); }
}
