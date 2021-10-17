use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use dispatchers::dispatcher_container::DispatcherContainer;

pub struct LongDispatcher {
	next: Box<DispatcherContainer>
}

impl LongDispatcher
{
	pub fn new(next: Box<DispatcherContainer>) -> LongDispatcher
	{
		LongDispatcher { next }
	}
}

impl Dispatcher for LongDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			L2D => { panic!("L2D not implemented") },
			L2F => { panic!("L2F not implemented") },
			L2I => { panic!("L2I not implemented") },
			LADD => { panic!("LADD not implemented") },
			LALOAD => { panic!("LALOAD not implemented") },
			LAND => { panic!("LAND not implemented") },
			LASTORE => { panic!("LASTORE not implemented") },
			LCMP => { panic!("LCMP not implemented") },
			LCONST_0 => { panic!("LCONST_0 not implemented") },
			LCONST_1 => { panic!("LCONST_1 not implemented") },
			LDC => { panic!("LDC not implemented") },
			LDC_W => { panic!("LDC_W not implemented") },
			LDC2_W => { panic!("LDC2_W not implemented") },
			LDIV => { panic!("LDIV not implemented") },
			LLOAD => { panic!("LLOAD not implemented") },
			LLOAD_N => { panic!("LLOAD_N not implemented") },
			LMUL => { panic!("LMUL not implemented") },
			LNEG => { panic!("LNEG not implemented") },
			LOR => { panic!("LOR not implemented") },
			LREM => { panic!("LREM not implemented") },
			LRETURN => { panic!("LRETURN not implemented") },
			LSHL => { panic!("LSHL not implemented") },
			LSHR => { panic!("LSHR not implemented") },
			LSTORE => { panic!("LSTORE not implemented") },
			LSTORE_N => { panic!("LSTORE_N not implemented") },
			LSUB => { panic!("LSUB not implemented") },
			LUSHR => { panic!("LUSHR not implemented") },
			LXOR => { panic!("LXOR not implemented") },
			_ => self.next.dispatch(thread_id, runtime_data, code)
		}
	}
}
