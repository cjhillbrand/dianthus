use dispatchers::dispatcher::Dispatcher;
use implementations::common_implementation::*;
use implementations::int_implementation::*;
use opcodes::*;
use run_time_data::RunTimeData;

pub struct IntDispatcher {}

impl Dispatcher for IntDispatcher {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		match self.get_instruction(thread_id, runtime_data) {
			BIPUSH => bipush(thread_id, runtime_data),
			I2B => i2b(thread_id, runtime_data),
			I2C => i2c(thread_id, runtime_data),
			I2D => i2d(thread_id, runtime_data),
			I2F => i2f(thread_id, runtime_data),
			I2L => i2l(thread_id, runtime_data),
			I2S => i2s(thread_id, runtime_data),
			IADD => i_add(thread_id, runtime_data),
			IALOAD => panic!("IALOAD not implemented"),
			IAND => i_and(thread_id, runtime_data),
			IASTORE => panic!("IASTORE not implemented"),
			ICONST_M1 => iconst_m1(thread_id, runtime_data),
			ICONST_0 => iconst_0(thread_id, runtime_data),
			ICONST_1 => iconst_1(thread_id, runtime_data),
			ICONST_2 => iconst_2(thread_id, runtime_data),
			ICONST_3 => iconst_3(thread_id, runtime_data),
			ICONST_4 => iconst_4(thread_id, runtime_data),
			ICONST_5 => iconst_5(thread_id, runtime_data),
			IDIV => i_div(thread_id, runtime_data),
			ILOAD => load(thread_id, runtime_data),
			ILOAD_0 => load_0(thread_id, runtime_data),
			ILOAD_1 => load_1(thread_id, runtime_data),
			ILOAD_2 => load_2(thread_id, runtime_data),
			ILOAD_3 => load_3(thread_id, runtime_data),
			IMUL => i_mul(thread_id, runtime_data),
			INEG => i_neg(thread_id, runtime_data),
			IOR => i_or(thread_id, runtime_data),
			IREM => i_rem(thread_id, runtime_data),
			IRETURN => i_return(thread_id, runtime_data),
			ISHL => i_shl(thread_id, runtime_data),
			ISHR => i_shr(thread_id, runtime_data),
			ISTORE => store(thread_id, runtime_data),
			ISTORE_0 => store_0(thread_id, runtime_data),
			ISTORE_1 => store_1(thread_id, runtime_data),
			ISTORE_2 => store_2(thread_id, runtime_data),
			ISTORE_3 => store_3(thread_id, runtime_data),
			ISUB => i_sub(thread_id, runtime_data),
			IUSHR => i_ushr(thread_id, runtime_data),
			IXOR => i_xor(thread_id, runtime_data),
			_ => {
				return false;
			}
		}

		true
	}
}
