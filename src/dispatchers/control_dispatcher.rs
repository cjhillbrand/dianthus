use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct ControlDispatcher<T: Dispatcher> {
	next: T
}

impl<T: Dispatcher> Dispatcher for ControlDispatcher<T> {
	fn dispatch(&self, thread_id: usize, mut runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			_ => self.next.dispatch(thread_id, &runtime_data, code)
		}
	}
}
