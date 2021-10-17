use std::any::Any;
use std::collections::{HashMap, VecDeque};

use runtime_lib::entities::class_struct::ClassStruct;
use stack_frame::StackFrame;

pub struct RunTimeData {
	program_counters: Vec<usize>,
	stacks: Vec<VecDeque<StackFrame>>,
	heap: Vec<Box<dyn Any>>,
	method_area: HashMap<String, Box<ClassStruct>>
}

impl RunTimeData {
	pub fn new() -> RunTimeData {
		RunTimeData {
			program_counters: Vec::new(),
			stacks: Vec::new(),
			heap: Vec::new(),
			method_area: HashMap::new()
		}
	}

	pub fn add_class(&mut self, class: Box<ClassStruct>) {
		let name = class.get_name().to_string();
		self.method_area.insert(name.clone(), class);
	}

	pub fn get_class(&self, name: &str) -> &ClassStruct {
		match &self.method_area.get(name) {
			Some(class) => class,
			None => panic!("Could not find class: {}", name)
		}
	}

	pub fn new_pc(&mut self) -> usize {
		self.program_counters.push(0);
		self.program_counters.len() - 1
	}

	pub fn set_pc(&mut self, thread: usize, value: usize) { self.program_counters[thread] = value; }

	pub fn increment_pc(&mut self, thread_id: usize, value: usize) { self.program_counters[thread_id] += value }

	pub fn get_pc(&self, thread: usize) -> usize { self.program_counters[thread] }

	pub fn add_stack(&mut self, stack: VecDeque<StackFrame>) { self.stacks.push(stack) }

	pub fn get_stack_mut(&mut self, thread: usize) -> &mut VecDeque<StackFrame> { &mut self.stacks[thread] }

	pub fn print_stacks(&self) {
		println!("{:?}", self.stacks);
	}
}
