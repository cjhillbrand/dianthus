use std::collections::VecDeque;

use run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub fn nop(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	current_stack_frame.increment_pc(1);
}
