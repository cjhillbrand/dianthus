use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct ClassDispatcher {
	next: Box<DispatcherContainer>
}

impl ClassDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> ClassDispatcher
	{
		ClassDispatcher { next }
	}
}

impl Dispatcher for ClassDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			GETFIELD => { panic!("GETFIELD not implemented") },
			GETSTATIC => { panic!("GETSTATIC not implemented") },
			INSTANCEOF => { panic!("INSTANCEOF not implemented") },
			MULTIANEWARRAY => { panic!("MULTIANEWARRAY not implemented") },
			NEW => { panic!("NEW not implemented") },
			NEWARRAY => { panic!("NEWARRAY not implemented") },
			PUTFIELD => { panic!("PUTFIELD not implemented") },
			PUTSTATIC => { panic!("PUTSTATIC not implemented") },
			RET => { panic!("RET not implemented") },
			RETURN => { panic!("RETURN not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
