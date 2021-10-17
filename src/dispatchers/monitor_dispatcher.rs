use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct MonitorDispatcher {}

impl Dispatcher for MonitorDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute) -> bool {
		match self.get_instruction(thread_id, runtime_data, code) {
			MONITORENTER => {
				panic!("MONITORENTER is not implemented")
			}
			MONITOREXIT => {
				panic!("MONITOREXIT is not implemented")
			}
			_ => false
		}
	}
}
