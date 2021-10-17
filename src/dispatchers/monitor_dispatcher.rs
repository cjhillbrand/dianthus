use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct MonitorDispatcher { }

impl Dispatcher for MonitorDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			MONITORENTER => { panic!("MONITORENTER is not implemented") },
			MONITOREXIT => { panic!("MONITOREXIT is not implemented") },
			_ => {  }
		}
	}
}
