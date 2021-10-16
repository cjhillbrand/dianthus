use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

use crate::run_time_data::RunTimeData;

pub trait Dispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute);
	fn get_instruction(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) -> u8 {
		let pc: usize = runtime_data.get_pc(thread_id.clone());
		let code_bytes: &Vec<u8> = code.get_code();
		code_bytes[pc]
	}
}
