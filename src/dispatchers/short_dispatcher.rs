use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct ShortDispatcher {
	next: Box<DispatcherContainer>
}

impl ShortDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> ShortDispatcher
	{
		ShortDispatcher { next }
	}
}

impl Dispatcher for ShortDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			SALOAD => { panic!("SALOAD not implemented") },
			SASTORE => { panic!("SASTORE not implemented") },
			SIPUSH => { panic!("SIPUSH not implemented") }
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
