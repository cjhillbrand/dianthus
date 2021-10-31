use std::collections::VecDeque;

use dispatchers::dispatcher::{create_dispatcher, Dispatcher};
use dispatchers::dispatcher_container::DispatcherContainer;
use run_time_data::RunTimeData;
use runtime_lib::class_loaders::class_loader::ClassLoader;
use runtime_lib::class_loaders::class_loader_container::ClassLoaderContainer;
use runtime_lib::class_loaders::system_class_loader::SystemClassLoader;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::method_info::MethodInfo;
use stack_frame::StackFrame;

const MAIN: &str = "main";
const INIT: &str = "<init>";

pub struct ClassExecutor {
	run_time_data: RunTimeData,
	class_loader: ClassLoaderContainer,
}

impl ClassExecutor {
	pub fn new() -> ClassExecutor {
		ClassExecutor {
			run_time_data: RunTimeData::new(),
			class_loader: ClassLoaderContainer::System(SystemClassLoader {}),
		}
	}

	pub fn execute(&mut self, init_class: &str) {
		let class: ClassStruct = self.class_loader.load_class(init_class);
		let class_ref: Box<ClassStruct> = Box::new(class);
		self.run_time_data.add_class(class_ref.clone()); // does this clone the whole value or just pointer?

		let init_method: &MethodInfo = class_ref.get_method(MAIN);
		let entry_point: &CodeAttribute = init_method.derive_code_attribute();

		let stack_frame = StackFrame::create_stack_frame(init_class, entry_point);
		let mut stack: VecDeque<StackFrame> = VecDeque::new();
		stack.push_front(stack_frame);
		let current_thread = self.run_time_data.new_thread(stack);
		ClassExecutor::execute_code(current_thread, &mut self.run_time_data);
	}

	pub fn execute_code(thread_id: usize, run_time_data: &mut RunTimeData)
	{
		let dispatcher: DispatcherContainer = create_dispatcher();
		loop {
			dispatcher.dispatch(thread_id, run_time_data);
			if run_time_data.is_stack_empty(thread_id) {
				break;
			}
		}
	}
}
