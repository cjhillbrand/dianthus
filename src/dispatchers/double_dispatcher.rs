use dispatchers::dispatcher::Dispatcher;
use implementations::common_implementation::*;
use implementations::double_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct DoubleDispatcher {}

impl Dispatcher for DoubleDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			D2F => d2f(thread_id, runtime_data),
			D2I => d2i(thread_id, runtime_data),
			D2L => d2l(thread_id, runtime_data),
			DADD => d_add(thread_id, runtime_data),
			DALOAD => {
				panic!("DALOAD not implemented")
			}
			DASTORE => {
				panic!("DASTORE not implemented")
			}
			DCMPG => d_cmp(thread_id, runtime_data),
			DCMPL => d_cmp(thread_id, runtime_data),
			DCONST_0 => dconst_0(thread_id, runtime_data),
			DCONST_1 => dconst_1(thread_id, runtime_data),
			DDIV => d_div(thread_id, runtime_data),
			DLOAD => {
				panic!("DLOAD not implemented")
			}
			DLOAD_0 => load_0(thread_id, runtime_data),
			DLOAD_1 => load_1(thread_id, runtime_data),
			DLOAD_2 => load_2(thread_id, runtime_data),
			DLOAD_3 => load_3(thread_id, runtime_data),
			DMUL => d_mul(thread_id, runtime_data),
			DNEG => d_neg(thread_id, runtime_data),
			DREM => d_rem(thread_id, runtime_data),
			DRETURN => d_return(thread_id, runtime_data),
			DSTORE => {
				panic!("DSTORE not implemented")
			}
			DSTORE_0 => store_0(thread_id, runtime_data),
			DSTORE_1 => store_1(thread_id, runtime_data),
			DSTORE_2 => store_2(thread_id, runtime_data),
			DSTORE_3 => store_3(thread_id, runtime_data),
			DSUB => d_sub(thread_id, runtime_data),
			_ => return false
		}

		return true
	}
}
