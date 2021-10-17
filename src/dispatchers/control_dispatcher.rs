use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct ControlDispatcher {
	next: Box<DispatcherContainer>
}

impl ControlDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> ControlDispatcher
	{
		ControlDispatcher { next }
	}
}

impl Dispatcher for ControlDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			ATHROW => { panic!("ATHROW not implemented") },
			CHECKCAST => { panic!("CHECKCAST not implemented") },
			IINC => { panic!("IINC not implemented") },
			LOOKUPSWITCH => { panic!("LOOKUPSWITCH not implemented") },
			NOP => { panic!("NOP not implemented") },
			POP => { panic!("POP not implemented") },
			POP2 => { panic!("POP2 not implemented") },
			SWAP => { panic!("SWAP not implemented") },
			TABLESWITCH => { panic!("TABLESWITCH not implemented") },
			WIDE => { panic!("WIDE not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}