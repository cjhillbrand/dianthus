use dispatchers::dispatcher::Dispatcher;
use implementations::common_implementation::*;
use implementations::long_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct LongDispatcher {}

impl Dispatcher for LongDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			L2D => l2d(thread_id, runtime_data),
			L2F => l2f(thread_id, runtime_data),
			L2I => l2i(thread_id, runtime_data),
			LADD => l_add(thread_id, runtime_data),
			LALOAD => panic!("LALOAD not implemented"),
			LAND => l_and(thread_id, runtime_data),
			LASTORE => panic!("LASTORE not implemented"),
			LCMP => l_cmp(thread_id, runtime_data),
			LCONST_0 => lconst_0(thread_id, runtime_data),
			LCONST_1 => lconst_1(thread_id, runtime_data),
			LDIV => l_div(thread_id, runtime_data),
			LLOAD => panic!("LLOAD not implemented"),
			LLOAD_0 => load_0(thread_id, runtime_data),
			LLOAD_1 => load_1(thread_id, runtime_data),
			LLOAD_2 => load_2(thread_id, runtime_data),
			LLOAD_3 => load_3(thread_id, runtime_data),
			LMUL => l_mul(thread_id, runtime_data),
			LNEG => l_neg(thread_id, runtime_data),
			LOR => l_or(thread_id, runtime_data),
			LREM => l_rem(thread_id, runtime_data),
			LRETURN => l_return(thread_id, runtime_data),
			LSHL => l_shl(thread_id, runtime_data),
			LSHR => l_shr(thread_id, runtime_data),
			LSTORE => {
				panic!("LSTORE not implemented")
			}
			LSTORE_0 => store_0(thread_id, runtime_data),
			LSTORE_1 => store_1(thread_id, runtime_data),
			LSTORE_2 => store_2(thread_id, runtime_data),
			LSTORE_3 => store_3(thread_id, runtime_data),
			LSUB => l_sub(thread_id, runtime_data),
			LUSHR => l_ushr(thread_id, runtime_data),
			LXOR => l_xor(thread_id, runtime_data),
			_ => return false
		}

		true
	}
}
