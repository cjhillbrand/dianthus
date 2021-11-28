use dispatchers::dispatcher::Dispatcher;
use implementations::compare_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct CompareDispatcher {}

impl Dispatcher for CompareDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			IF_ACMPEQ => panic!("IFACMPEQ is not implemented"),
			IF_ACMPNE => panic!("IFACMPNE is not implemented"),
			IF_ICMPEQ => if_icmpeq(thread_id, runtime_data),
			IF_ICMPNE => if_icmpne(thread_id, runtime_data),
			IF_ICMPLT => if_icmplt(thread_id, runtime_data),
			IF_ICMPGE => if_icmpge(thread_id, runtime_data),
			IF_ICMPGT => if_icmpgt(thread_id, runtime_data),
			IF_ICMPLE => if_icmple(thread_id, runtime_data),
			IFEQ => if_eq(thread_id, runtime_data),
			IFNE => if_ne(thread_id, runtime_data),
			IFLT => if_lt(thread_id, runtime_data),
			IFLE => if_le(thread_id, runtime_data),
			IFGE => if_ge(thread_id, runtime_data),
			IFGT => if_gt(thread_id, runtime_data),
			IFNONNULL => panic!("IFNONULL is not implemented"),
			IFNULL => panic!("IFNULL is not implemented"),
			_ => { return false }
		};

		return true;
	}
}
