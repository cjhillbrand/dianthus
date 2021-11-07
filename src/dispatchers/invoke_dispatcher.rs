use dispatchers::dispatcher::Dispatcher;
use implementations::invoke_implementation::{invoke_static, invoke_virtual};
use opcodes::*;
use run_time_data::RunTimeData;

pub struct InvokeDispatcher {}

impl Dispatcher for InvokeDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			INVOKE_DYNAMIC => panic!("INVOKE_DYNAMIC not implemented"),
			INVOKEINTERFACE => panic!("INVOKEINTERFACE not implemented"),
			INVOKESPECIAL => panic!("INVOKESPECIAL not implemented"),
			INVOKESTATIC => invoke_static(thread_id, runtime_data),
			INVOKEVIRTUAL => invoke_virtual(thread_id, runtime_data),
			_ => return false
		}

		true
	}
}
