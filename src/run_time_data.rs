use std::any::Any;
use std::collections::{HashMap, VecDeque};

use runtime_lib::class_loaders::class_loader_container::ClassLoaderContainer;
use runtime_lib::class_loaders::system_class_loader::SystemClassLoader;
use runtime_lib::entities::class_struct::ClassStruct;
use stack_frame::StackFrame;

pub struct RunTimeData {
	stacks: Vec<VecDeque<StackFrame>>,
	class_loader: ClassLoaderContainer,
	heap: Vec<Box<dyn Any>>,
	method_area: HashMap<String, Box<ClassStruct>>
}

impl RunTimeData {
	pub fn new() -> RunTimeData {
		RunTimeData {
			stacks: Vec::new(),
			class_loader: ClassLoaderContainer::System(SystemClassLoader {}),
			heap: Vec::new(),
			method_area: HashMap::new()
		}
	}

	pub fn add_class(&mut self, class: Box<ClassStruct>) {
		let name = class.get_name().to_string();
		self.method_area.insert(name, class);
	}

	pub fn get_class(&self, name: &str) -> &ClassStruct {
		match &self.method_area.get(name) {
			Some(class) => class,
			None => panic!("Could not find class: {}", name)
		}
	}

	pub fn new_thread(&mut self, stack: VecDeque<StackFrame>) -> usize {
		self.stacks.push(stack);
		self.stacks.len() - 1
	}

	pub fn is_stack_empty(&self, thread: usize) -> bool { self.stacks[thread].len() == 0 }

	pub fn get_stack_mut(&mut self, thread: usize) -> &mut VecDeque<StackFrame> { &mut self.stacks[thread] }

	pub fn get_stack(&self, thread: usize) -> &VecDeque<StackFrame> { &self.stacks[thread] }

	pub fn print_stacks(&self) {
		println!("{:?}", self.stacks);
	}
}
