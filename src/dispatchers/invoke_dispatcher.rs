use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct InvokeDispatcher {
	next: Box<DispatcherContainer>
}

impl InvokeDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> InvokeDispatcher
	{
		InvokeDispatcher { next }
	}
}

impl Dispatcher for InvokeDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			INVOKE_DYNAMIC => { panic!("INVOKE_DYNAMIC not implemented") },
			INVOKEINTERFACE => { panic!("INVOKEINTERFACE not implemented") },
			INVOKESPECIAL => { panic!("INVOKESPECIAL not implemented") },
			INVOKESTATIC => { panic!("INVOKESTATIC not implemented") },
			INVOKEVIRTUAL => { panic!("INVOKEVIRTUAL not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
