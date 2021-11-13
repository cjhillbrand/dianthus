use dispatchers::dispatcher::Dispatcher;
use implementations::common_implementation::*;
use implementations::float_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct FloatDispatcher {}

impl Dispatcher for FloatDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			F2I => f2i(thread_id, runtime_data),
			F2L => f2l(thread_id, runtime_data),
			FADD => f_add(thread_id, runtime_data),
			FALOAD => panic!("FALOAD not implemented"),
			FASTORE => panic!("FASTORE not implemented"),
			FCMPG => f_cmp(thread_id, runtime_data),
			FCMPL => f_cmp(thread_id, runtime_data),
			FCONST_0 => fconst_0(thread_id, runtime_data),
			FCONST_1 => fconst_1(thread_id, runtime_data),
			FCONST_2 => fconst_2(thread_id, runtime_data),
			FDIV => f_div(thread_id, runtime_data),
			FLOAD => {
				panic!("FLOAD not implemented")
			}
			FLOAD_0 => load_0(thread_id, runtime_data),
			FLOAD_1 => load_1(thread_id, runtime_data),
			FLOAD_2 => load_2(thread_id, runtime_data),
			FLOAD_3 => load_3(thread_id, runtime_data)
			FMUL => f_mul(thread_id, runtime_data),
			FNEG => f_neg(thread_id, runtime_data),
			FREM => f_rem(thread_id, runtime_data),
			FRETURN => f_return(thread_id, runtime_data),
			FSTORE => {
				panic!("FSTORE not implemented")
			}
			FSTORE_0 => store_0(thread_id, runtime_data),
			FSTORE_1 => store_1(thread_id, runtime_data),
			FSTORE_2 => store_2(thread_id, runtime_data),
			FSTORE_3 => store_3(thread_id, runtime_data),
			FSUB => f_sub(thread_id, runtime_data),
			_ => return false
		}

		true
	}
}
