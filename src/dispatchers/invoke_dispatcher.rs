use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct InvokeDispatcher { }

impl Dispatcher for InvokeDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			INVOKE_DYNAMIC => { panic!("INVOKE_DYNAMIC not implemented") },
			INVOKEINTERFACE => { panic!("INVOKEINTERFACE not implemented") },
			INVOKESPECIAL => { panic!("INVOKESPECIAL not implemented") },
			INVOKESTATIC => { panic!("INVOKESTATIC not implemented") },
			INVOKEVIRTUAL => { panic!("INVOKEVIRTUAL not implemented") },
			_ => {  }
		}
	}
}
