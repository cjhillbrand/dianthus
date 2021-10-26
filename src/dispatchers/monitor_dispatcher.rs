use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct MonitorDispatcher {}

impl Dispatcher for MonitorDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
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
