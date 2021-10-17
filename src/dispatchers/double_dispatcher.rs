use dispatchers::dispatcher::Dispatcher;
use opcodes::*;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct DoubleDispatcher {}

impl Dispatcher for DoubleDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute) -> bool {
		match self.get_instruction(thread_id, runtime_data, code) {
			D2F => {
				panic!("D2F not implemented")
			}
			D2I => {
				panic!("D2I not implemented")
			}
			D2L => {
				panic!("D2L not implemented")
			}
			DADD => {
				panic!("DADD not implemented")
			}
			DALOAD => {
				panic!("DALOAD not implemented")
			}
			DASTORE => {
				panic!("DASTORE not implemented")
			}
			DCMPG => {
				panic!("DCMPG not implemented")
			}
			DCMPL => {
				panic!("DCMPL not implemented")
			}
			DCONST_0 => {
				panic!("DCONST_0 not implemented")
			}
			DCONST_1 => {
				panic!("DCONST_1 not implemented")
			}
			DDIV => {
				panic!("DDIV not implemented")
			}
			DLOAD => {
				panic!("DLOAD not implemented")
			}
			DLOAD_0 => {
				panic!("DLOAD_0 not implemented")
			}
			DLOAD_1 => {
				panic!("DLOAD_1 not implemented")
			}
			DLOAD_2 => {
				panic!("DLOAD_2 not implemented")
			}
			DLOAD_3 => {
				panic!("DLOAD_3 not implemented")
			}
			DMUL => {
				panic!("DMUL not implemented")
			}
			DNEG => {
				panic!("DNEG not implemented")
			}
			DREM => {
				panic!("DREM not implemented")
			}
			DRETURN => {
				panic!("DRETURN not implemented")
			}
			DSTORE => {
				panic!("DSTORE not implemented")
			}
			DSTORE_0 => {
				panic!("DSTORE_0 not implemented")
			}
			DSTORE_1 => {
				panic!("DSTORE_1 not implemented")
			}
			DSTORE_2 => {
				panic!("DSTORE_2 not implemented")
			}
			DSTORE_3 => {
				panic!("DSTORE_3 not implemented")
			}
			DSUB => {
				panic!("DSUB not implemented")
			}
			_ => false
		}
	}
}
