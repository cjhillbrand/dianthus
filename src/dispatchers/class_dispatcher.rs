use dispatchers::dispatcher::Dispatcher;
use implementations::class_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct ClassDispatcher {}

impl Dispatcher for ClassDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			GETFIELD => panic!("GETFIELD not implemented"),
			GETSTATIC => panic!("GETSTATIC not implemented"),
			INSTANCEOF => panic!("INSTANCEOF not implemented"),
			MULTIANEWARRAY => panic!("MULTIANEWARRAY not implemented"),
			NEW => panic!("NEW not implemented"),
			NEWARRAY => panic!("NEWARRAY not implemented"),
			PUTFIELD => panic!("PUTFIELD not implemented"),
			PUTSTATIC => panic!("PUTSTATIC not implemented"),
			RET => panic!("RET not implemented"),
			RETURN => return_op(thread_id, runtime_data),
			_ => return false
		}

		true
	}
}
