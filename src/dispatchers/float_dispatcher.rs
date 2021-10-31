use dispatchers::dispatcher::Dispatcher;
use implementations::float_implementation::{fconst_0, fconst_1, fconst_2};
use opcodes::*;
use run_time_data::RunTimeData;

pub struct FloatDispatcher {}

impl Dispatcher for FloatDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			F2I => {
				panic!("F2I not implemented")
			}
			F2L => {
				panic!("F2L not implemented")
			}
			FADD => {
				panic!("FADD not implemented")
			}
			FALOAD => {
				panic!("FALOAD not implemented")
			}
			FASTORE => {
				panic!("FASTORE not implemented")
			}
			FCMPG => {
				panic!("FCMPG not implemented")
			}
			FCMPL => {
				panic!("FCMPL not implemented")
			}
			FCONST_0 => {
				fconst_0(thread_id, runtime_data);
			}
			FCONST_1 => {
				fconst_1(thread_id, runtime_data);
			}
			FCONST_2 => {
				fconst_2(thread_id, runtime_data);
			}
			FDIV => {
				panic!("FDIV not implemented")
			}
			FLOAD => {
				panic!("FLOAD not implemented")
			}
			FLOAD_0 => {
				panic!("FLOAD_0 not implemented")
			}
			FLOAD_1 => {
				panic!("FLOAD_1 not implemented")
			}
			FLOAD_2 => {
				panic!("FLOAD_2 not implemented")
			}
			FLOAD_3 => {
				panic!("FLOAD_3 not implemented")
			}
			FMUL => {
				panic!("FMUL not implemented")
			}
			FNEG => {
				panic!("FNEG not implemented")
			}
			FREM => {
				panic!("FREM not implemented")
			}
			FRETURN => {
				panic!("FRETURN not implemented")
			}
			FSTORE => {
				panic!("FSTORE not implemented")
			}
			FSORE_N => {
				panic!("FSTORE_N not implemented")
			}
			FSUB => {
				panic!("FSUB not implemented")
			}
			_ => {
				return false;
			}
		}

		true
	}
}
