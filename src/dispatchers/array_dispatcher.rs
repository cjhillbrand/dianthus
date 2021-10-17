use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use opcodes::*;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct ArrayDispatcher { }

impl Dispatcher for ArrayDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			AALOAD => { panic!("AALOAD not implemented") },
			AASTORE => { panic!("AASOTRE not implemented") },
			ACONST_NULL => { panic!("ACONST_NULL not implemented") },
			ALOAD => { panic!("ALOAD not implemented") },
			ANEWARRAY => { panic!("ANEWARRAY not implemented") },
			ARETURN => { panic!("ARETURN not implemented") },
			ARRAYLENGTH => { panic!("ARRAYLENGTH not implemented") },
			ASTORE => { panic!("ASTORE not implemented") },
			ASTORE_0 => { panic!("ASTORE_0 not implemented") },
			ASTORE_1 => { panic!("ASTORE_1 not implemented") },
			ASTORE_2 => { panic!("ASTORE_2 not implemented") },
			ASTORE_3 => { panic!("ASTORE_3 not implemeneted") },
			BALOAD => { panic!("BALOAD not implemented") },
			BASTORE => { panic!("BASTORE not implemented") },
			BIPUSH => { panic!("BIPUSH not implemented") },
			CALOAD => { panic!("CALOAD not implemented") },
			CASTORE => { panic!("CASTORE not implemented") },
			_ => {  }
		}
	}
}
