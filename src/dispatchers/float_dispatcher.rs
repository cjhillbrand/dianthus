use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct FloatDispatcher { }

impl Dispatcher for FloatDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			F2I => { panic!("F2I not implemented") },
			F2L => { panic!("F2L not implemented") },
			FADD => { panic!("FADD not implemented") },
			FALOAD => { panic!("FALOAD not implemented") },
			FASTORE => { panic!("FASTORE not implemented") },
			FCMPG => { panic!("FCMPG not implemented") },
			FCMPL => { panic!("FCMPL not implemented") },
			FCONST_0 => { panic!("FCONST_0 not implemented") },
			FCONST_1 => { panic!("FCONST_1 not implemented") },
			FCONST_2 => { panic!("FCONST_2 not implemented") },
			FDIV => { panic!("FDIV not implemented") },
			FLOAD => { panic!("FLOAD not implemented") },
			FLOAD_0 => { panic!("FLOAD_0 not implemented") },
			FLOAD_1 => { panic!("FLOAD_1 not implemented") },
			FLOAD_2 => { panic!("FLOAD_2 not implemented") },
			FLOAD_3 => { panic!("FLOAD_3 not implemented") },
			FMUL => { panic!("FMUL not implemented") },
			FNEG => { panic!("FNEG not implemented") },
			FREM => { panic!("FREM not implemented") },
			FRETURN => { panic!("FRETURN not implemented") },
			FSTORE => { panic!("FSTORE not implemented") },
			FSORE_N => { panic!("FSTORE_N not implemented") },
			FSUB => { panic!("FSUB not implemented") },
			_ => {  }
		}
	}
}
