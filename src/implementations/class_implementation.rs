use std::collections::VecDeque;

use run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub fn return_op(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	stack.pop_front();
}
