use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct JumpDispatcher {
	next: Box<DispatcherContainer>
}

impl JumpDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> JumpDispatcher
	{
		JumpDispatcher { next }
	}
}

impl Dispatcher for JumpDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			GOTO => { panic!("GOTO not implemented") },
			GOTO_W => { panic!("GOTO_W not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
