use std::collections::HashMap;

use jvm_object::JvmObject;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Heap {
	objects: HashMap<u64, JvmObject>
}

impl Heap {
	pub fn new() -> Heap {
		Heap {
			objects: HashMap::new()
		}
	}

	pub fn value(&self, reference: u64) -> &JvmObject {
		match self.objects.get(&reference) {
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

	pub fn dealloc(&mut self, reference: u64) { self.objects.remove(&reference); }
}
