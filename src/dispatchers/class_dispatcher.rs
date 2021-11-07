use dispatchers::dispatcher::Dispatcher;
use implementations::class_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct ClassDispatcher {}

impl Dispatcher for ClassDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			ALOAD => panic!("ALOAD not implemented"),
			ALOAD_0 => aload_0(thread_id, runtime_data),
			ALOAD_1 => aload_1(thread_id, runtime_data),
			ALOAD_2 => aload_2(thread_id, runtime_data),
			ALOAD_3 => aload_3(thread_id, runtime_data),
			LDC => ldc(thread_id, runtime_data),
			LDC_W => panic!("LDC_W not implemented"),
			LDC2_W => panic!("LDC2_W not implemented"),
			GETFIELD => panic!("GETFIELD not implemented"),
			GETSTATIC => get_static(thread_id, runtime_data),
			INSTANCEOF => panic!("INSTANCEOF not implemented"),
			MULTIANEWARRAY => panic!("MULTIANEWARRAY not implemented"),
			NEW => panic!("NEW not implemented"),
			NEWARRAY => panic!("NEWARRAY not implemented"),
			PUTFIELD => panic!("PUTFIELD not implemented"),
			PUTSTATIC => put_static(thread_id, runtime_data),
			RET => panic!("RET not implemented"),
			RETURN => return_op(thread_id, runtime_data),
			_ => return false
		}

		true
	}
}
