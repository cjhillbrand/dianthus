use dispatchers::dispatcher::Dispatcher;
use implementations::array_implementation::aconst_null;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct ArrayDispatcher {}

impl Dispatcher for ArrayDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			AALOAD => panic!("AALOAD not implemented"),
			AASTORE => panic!("AASOTRE not implemented"),
			ACONST_NULL => aconst_null(thread_id, runtime_data),
			ANEWARRAY => panic!("ANEWARRAY not implemented"),
			ARETURN => panic!("ARETURN not implemented"),
			ARRAYLENGTH => panic!("ARRAYLENGTH not implemented"),
			ASTORE => panic!("ASTORE not implemented"),
			ASTORE_0 => panic!("ASTORE_0 not implemented"),
			ASTORE_1 => panic!("ASTORE_1 not implemented"),
			ASTORE_2 => panic!("ASTORE_2 not implemented"),
			ASTORE_3 => panic!("ASTORE_3 not implemeneted"),
			BALOAD => panic!("BALOAD not implemented"),
			BASTORE => panic!("BASTORE not implemented"),
			CALOAD => panic!("CALOAD not implemented"),
			CASTORE => panic!("CASTORE not implemented"),
			_ => return false
		}

		true
	}
}
