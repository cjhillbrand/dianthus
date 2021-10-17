use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct ArrayDispatcher {}

impl Dispatcher for ArrayDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute) -> bool {
		match self.get_instruction(thread_id, runtime_data, code) {
			AALOAD => {
				panic!("AALOAD not implemented")
			}
			AASTORE => {
				panic!("AASOTRE not implemented")
			}
			ACONST_NULL => {
				panic!("ACONST_NULL not implemented")
			}
			ALOAD => {
				panic!("ALOAD not implemented")
			}
			ALOAD_0 => {
				panic!("ALOAD_0 not implemented")
			}
			ALOAD_1 => {
				panic!("ALOAD_1 not implemented")
			}
			ALOAD_2 => {
				panic!("ALOAD_2 not implemented")
			}
			ALOAD_3 => {
				panic!("ALOAD_3 not implemented")
			}
			ANEWARRAY => {
				panic!("ANEWARRAY not implemented")
			}
			ARETURN => {
				panic!("ARETURN not implemented")
			}
			ARRAYLENGTH => {
				panic!("ARRAYLENGTH not implemented")
			}
			ASTORE => {
				panic!("ASTORE not implemented")
			}
			ASTORE_0 => {
				panic!("ASTORE_0 not implemented")
			}
			ASTORE_1 => {
				panic!("ASTORE_1 not implemented")
			}
			ASTORE_2 => {
				panic!("ASTORE_2 not implemented")
			}
			ASTORE_3 => {
				panic!("ASTORE_3 not implemeneted")
			}
			BALOAD => {
				panic!("BALOAD not implemented")
			}
			BASTORE => {
				panic!("BASTORE not implemented")
			}
			BIPUSH => {
				panic!("BIPUSH not implemented")
			}
			CALOAD => {
				panic!("CALOAD not implemented")
			}
			CASTORE => {
				panic!("CASTORE not implemented")
			}
			_ => false
		}
	}
}
