use dispatchers::dispatcher::Dispatcher;
use implementations::long_implementation::{lconst_0, lconst_1};
use opcodes::*;
use run_time_data::RunTimeData;

pub struct LongDispatcher {}

impl Dispatcher for LongDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			L2D => {
				panic!("L2D not implemented")
			}
			L2F => {
				panic!("L2F not implemented")
			}
			L2I => {
				panic!("L2I not implemented")
			}
			LADD => {
				panic!("LADD not implemented")
			}
			LALOAD => {
				panic!("LALOAD not implemented")
			}
			LAND => {
				panic!("LAND not implemented")
			}
			LASTORE => {
				panic!("LASTORE not implemented")
			}
			LCMP => {
				panic!("LCMP not implemented")
			}
			LCONST_0 => lconst_0(thread_id, runtime_data),
			LCONST_1 => lconst_1(thread_id, runtime_data),
			LDIV => {
				panic!("LDIV not implemented")
			}
			LLOAD => {
				panic!("LLOAD not implemented")
			}
			LLOAD_N => {
				panic!("LLOAD_N not implemented")
			}
			LMUL => {
				panic!("LMUL not implemented")
			}
			LNEG => {
				panic!("LNEG not implemented")
			}
			LOR => {
				panic!("LOR not implemented")
			}
			LREM => {
				panic!("LREM not implemented")
			}
			LRETURN => {
				panic!("LRETURN not implemented")
			}
			LSHL => {
				panic!("LSHL not implemented")
			}
			LSHR => {
				panic!("LSHR not implemented")
			}
			LSTORE => {
				panic!("LSTORE not implemented")
			}
			LSTORE_N => {
				panic!("LSTORE_N not implemented")
			}
			LSUB => {
				panic!("LSUB not implemented")
			}
			LUSHR => {
				panic!("LUSHR not implemented")
			}
			LXOR => {
				panic!("LXOR not implemented")
			}
			_ => return false
		}

		true
	}
}
