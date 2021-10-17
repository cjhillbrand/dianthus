use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

use crate::run_time_data::RunTimeData;
use dispatchers::dispatcher_container::DispatcherContainer;
use dispatchers::array_dispatcher::ArrayDispatcher;
use dispatchers::class_dispatcher::ClassDispatcher;
use dispatchers::control_dispatcher::ControlDispatcher;
use dispatchers::double_dispatcher::DoubleDispatcher;
use dispatchers::dup_dispatcher::DupDispatcher;
use dispatchers::float_dispatcher::FloatDispatcher;
use dispatchers::int_dispatcher::IntDispatcher;
use dispatchers::invoke_dispatcher::InvokeDispatcher;
use dispatchers::jump_dispatcher::JumpDispatcher;
use dispatchers::long_dispatcher::LongDispatcher;
use dispatchers::monitor_dispatcher::MonitorDispatcher;
use dispatchers::short_dispatcher::ShortDispatcher;

pub trait Dispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute);
	fn get_instruction(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) -> u8 {
		let pc: usize = runtime_data.get_pc(thread_id);
		let code_bytes: &Vec<u8> = code.get_code();
		code_bytes[pc]
	}
}

pub fn create_dispatcher() -> Box<DispatcherContainer>
{
	let none_dispatcher = Box::new(DispatcherContainer::NONE);
	let array_dispatcher = Box::new(DispatcherContainer::ARRAY(Box::new(ArrayDispatcher::new(none_dispatcher))));
	let class_dispatcher = Box::new(DispatcherContainer::CLASS(Box::new(ClassDispatcher::new(array_dispatcher))));
	let control_dispatcher = Box::new(DispatcherContainer::CONTROL(Box::new(ControlDispatcher::new(class_dispatcher))));
	let double_dispatcher = Box::new(DispatcherContainer::DOUBLE(Box::new(DoubleDispatcher::new(control_dispatcher))));
	let dup_dispatcher = Box::new(DispatcherContainer::DUP(Box::new(DupDispatcher::new(double_dispatcher))));
	let float_dispatcher = Box::new(DispatcherContainer::FLOAT(Box::new(FloatDispatcher::new(dup_dispatcher))));
	let int_dispatcher = Box::new(DispatcherContainer::INT(Box::new(IntDispatcher::new(float_dispatcher))));
	let invoke_dispatcher = Box::new(DispatcherContainer::INVOKE(Box::new(InvokeDispatcher::new(int_dispatcher))));
	let jump_dispatcher = Box::new(DispatcherContainer::JUMP(Box::new(JumpDispatcher::new(invoke_dispatcher))));
	let long_dispatcher = Box::new(DispatcherContainer::LONG(Box::new(LongDispatcher::new(jump_dispatcher))));
	let monitor_dispatcher = Box::new(DispatcherContainer::MONITOR(Box::new(MonitorDispatcher::new(long_dispatcher))));
	Box::new(DispatcherContainer::SHORT(Box::new(ShortDispatcher::new(monitor_dispatcher))))
}