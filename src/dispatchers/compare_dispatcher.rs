use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct CompareDispatcher {}

impl Dispatcher for CompareDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			IF_ACMPEQ => panic!("IFACMPEQ is not implemented"),
			IF_ACMPNE => panic!("IFACMPNE is not implemented"),
			IF_ICMPEQ => panic!("IFICMPEQ is not implemented"),
			IF_ICMPNE => panic!("IFICMPNE is not implemented"),
			IF_ICMPLT => panic!("IFICMPLT is not implemented"),
			IF_ICMPGE => panic!("IFICMPGE is not implemented"),
			IF_ICMPGT => panic!("IFICMPGT is not implemented"),
			IF_ICMPLE => panic!("IFICMPLE is not implemented"),
			IFEQ => panic!("IFEQ is not implemented"),
			IFNE => panic!("IFNE is not implemented"),
			IFLT => panic!("IFLT is not implemented"),
			IFLE => panic!("IFLEis not implemented"),
			IFGE => panic!("IFGE is not implemented"),
			IFGT => panic!("IFGT is not implemented"),
			IFNONNULL => panic!("IFNONULL is not implemented"),
			IFNULL => panic!("IFNULL is not implemented"),
			_ => false
		}
	}
}
