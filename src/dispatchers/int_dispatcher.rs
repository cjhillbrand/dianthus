use dispatchers::dispatcher::Dispatcher;
use implementations::int_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct IntDispatcher {}

impl Dispatcher for IntDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute) -> bool {
		match self.get_instruction(thread_id.clone(), runtime_data, code) {
			I2B => panic!("I2B not implemented"),
			I2C => panic!("I2C not implemented"),
			I2D => panic!("I2D not implemented"),
			I2F => panic!("I2F not implemented"),
			I2L => panic!("I2L not implemented"),
			I2S => panic!("I2S not implemented"),
			IADD => panic!("IADD not implemented"),
			IALOAD => panic!("IALOAD not implemented"),
			IAND => panic!("IAND not implemented"),
			IASTORE => panic!("IASTORE not implemented"),
			ICONST_M1 => iconst_m1(thread_id, runtime_data, code),
			ICONST_0 => iconst_0(thread_id, runtime_data, code),
			ICONST_1 => iconst_1(thread_id, runtime_data, code),
			ICONST_2 => iconst_2(thread_id, runtime_data, code),
			ICONST_3 => iconst_3(thread_id, runtime_data, code),
			ICONST_4 => iconst_4(thread_id, runtime_data, code),
			ICONST_5 => iconst_5(thread_id, runtime_data, code),
			IDIV => panic!("IDIV not implemented"),
			ILOAD => panic!("ILOAD not implemented"),
			ILOAD_N => panic!("ILOAD_N not implemented"),
			IMUL => panic!("IMUL not implemented"),
			INEG => panic!("INEG not implemented"),
			IOR => panic!("IOR not implemented"),
			IREM => panic!("IREM not implemented"),
			IRETURN => panic!("IRETURN not implemented"),
			ISHL => panic!("ISHL not implemented"),
			ISHR => panic!("ISHR not implemented"),
			ISTORE => panic!("ISTORE not implemented"),
			ISTORE_0 => istore_0(thread_id, runtime_data, code),
			ISTORE_1 => istore_1(thread_id, runtime_data, code),
			ISTORE_2 => istore_2(thread_id, runtime_data, code),
			ISTORE_3 => istore_3(thread_id, runtime_data, code),
			ISUB => panic!("ISUB not implemented"),
			IUSHR => panic!("IUSHR not implemented"),
			IXOR => panic!("IXOR not implemented"),
			_ => {
				return false;
			}
		}

		true
	}
}
