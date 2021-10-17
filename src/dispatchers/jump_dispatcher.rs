use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct JumpDispatcher { }

impl Dispatcher for JumpDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			GOTO => { panic!("GOTO not implemented") },
			GOTO_W => { panic!("GOTO_W not implemented") },
			_ => {  }
		}
	}
}
