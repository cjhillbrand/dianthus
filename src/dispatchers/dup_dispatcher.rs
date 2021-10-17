use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct DupDispatcher {
	next: Box<DispatcherContainer>
}

impl DupDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> DupDispatcher
	{
		DupDispatcher { next }
	}
}

impl Dispatcher for DupDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			DUP => { panic!("DUP not implemented") },
			DUP_X1 => { panic!("DUP_X1 not implemented") },
			DUP_X2 => { panic!("DUP_X2 not implemented") },
			DUP2 => { panic!("DUP2 not implemented") },
			DUP2_X1 => { panic!("DUP2_X1 not implemented") },
			DUP2_X2 => { panic!("DUP2_X2 not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
