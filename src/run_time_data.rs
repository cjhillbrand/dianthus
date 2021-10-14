use std::any::Any;
use std::collections::{HashMap, VecDeque};

use runtime_lib::entities::class_struct::ClassStruct;
use stack_frame::StackFrame;

pub struct RunTimeData {
	program_counters: Vec<u32>,
	stacks: Vec<VecDeque<StackFrame>>,
	heap: Vec<Box<dyn Any>>,
	method_area: HashMap<String, ClassStruct>
}

impl RunTimeData
{
	pub fn new() -> RunTimeData
	{
		RunTimeData {
			program_counters: Vec::new(),
			stacks: Vec::new(),
			heap: Vec::new(),
			method_area: HashMap::new()
		}
	}

	pub fn add_class(&self, class: ClassStruct)
	{
		self.method_area.insert(class.get_name(), class);
	}
}