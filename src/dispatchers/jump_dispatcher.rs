use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct JumpDispatcher {}

impl Dispatcher for JumpDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			GOTO => {
				panic!("GOTO not implemented")
			}
			GOTO_W => {
				panic!("GOTO_W not implemented")
			}
			_ => false
		}
	}
}
