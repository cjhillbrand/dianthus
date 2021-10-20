use dispatchers::array_dispatcher::ArrayDispatcher;
use dispatchers::class_dispatcher::ClassDispatcher;
use dispatchers::compare_dispatcher::CompareDispatcher;
use dispatchers::control_dispatcher::ControlDispatcher;
use dispatchers::dispatcher_container::DispatcherContainer;
use dispatchers::double_dispatcher::DoubleDispatcher;
use dispatchers::dup_dispatcher::DupDispatcher;
use dispatchers::float_dispatcher::FloatDispatcher;
use dispatchers::int_dispatcher::IntDispatcher;
use dispatchers::invoke_dispatcher::InvokeDispatcher;
use dispatchers::jump_dispatcher::JumpDispatcher;
use dispatchers::long_dispatcher::LongDispatcher;
use dispatchers::monitor_dispatcher::MonitorDispatcher;
use dispatchers::short_dispatcher::ShortDispatcher;

use crate::run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub trait Dispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool;
	fn get_instruction(&self, thread_id: usize, runtime_data: &RunTimeData) -> u8 {
		let current_stack = runtime_data.get_stack(thread_id);
		let frame: &StackFrame = match current_stack.front()
		{
			Some(frame) => frame,
			None => panic!("No stack frame on stack: {}", thread_id)
		};

		let pc: usize = frame.get_pc();
		let code_bytes: &Vec<u8> = frame.get_code();
		code_bytes[pc]
	}
}

pub fn create_dispatcher() -> DispatcherContainer {
	DispatcherContainer::new(vec![
		Box::new(ArrayDispatcher {}),
		Box::new(ClassDispatcher {}),
		Box::new(CompareDispatcher {}),
		Box::new(ControlDispatcher {}),
		Box::new(DoubleDispatcher {}),
		Box::new(DupDispatcher {}),
		Box::new(FloatDispatcher {}),
		Box::new(IntDispatcher {}),
		Box::new(InvokeDispatcher {}),
		Box::new(JumpDispatcher {}),
		Box::new(LongDispatcher {}),
		Box::new(MonitorDispatcher {}),
		Box::new(ShortDispatcher {}),
	])
}
