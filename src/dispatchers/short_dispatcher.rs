use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct ShortDispatcher {}

impl Dispatcher for ShortDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute) -> bool {
		match self.get_instruction(thread_id, runtime_data, code) {
			SALOAD => {
				panic!("SALOAD not implemented")
			}
			SASTORE => {
				panic!("SASTORE not implemented")
			}
			SIPUSH => {
				panic!("SIPUSH not implemented")
			}
			_ => false
		}
	}
}
