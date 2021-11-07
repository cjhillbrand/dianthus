use std::collections::VecDeque;

use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub fn aconst_null(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame: &mut StackFrame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	current_stack_frame.push_on_stack(JvmValue::PlaceHolder);
	current_stack_frame.increment_pc(1);
}
