use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct DupDispatcher {}

impl Dispatcher for DupDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute) -> bool {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			DUP => {
				panic!("DUP not implemented")
			}
			DUP_X1 => {
				panic!("DUP_X1 not implemented")
			}
			DUP_X2 => {
				panic!("DUP_X2 not implemented")
			}
			DUP2 => {
				panic!("DUP2 not implemented")
			}
			DUP2_X1 => {
				panic!("DUP2_X1 not implemented")
			}
			DUP2_X2 => {
				panic!("DUP2_X2 not implemented")
			}
			_ => false
		}
	}
}
