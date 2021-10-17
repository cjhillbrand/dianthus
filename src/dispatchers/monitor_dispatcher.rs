use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct MonitorDispatcher {
	next: Box<DispatcherContainer>
}

impl MonitorDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> MonitorDispatcher
	{
		MonitorDispatcher { next }
	}
}

impl Dispatcher for MonitorDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			MONITORENTER => { panic!("MONITORENTER is not implemented") },
			MONITOREXIT => { panic!("MONITOREXIT is not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
