use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct ControlDispatcher {}

impl Dispatcher for ControlDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			ATHROW => panic!("ATHROW not implemented"),
			CHECKCAST => panic!("CHECKCAST not implemented"),
			IINC => panic!("IINC not implemented"),
			LOOKUPSWITCH => panic!("LOOKUPSWITCH not implemented"),
			NOP => panic!("NOP not implemented"),
			POP => panic!("POP not implemented"),
			POP2 => panic!("POP2 not implemented"),
			SWAP => panic!("SWAP not implemented"),
			TABLESWITCH => panic!("TABLESWITCH not implemented"),
			WIDE => panic!("WIDE not implemented"),
			_ => false
		}
	}
}
