use std::collections::VecDeque;

use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;

fn fconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: f32) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};
	let value: JvmValue = JvmValue::Float(value);
	current_stack_frame.push_on_stack(value);
	current_stack_frame.increment_pc(1);
}

pub fn fconst_0(thread_id: usize, runtime_data: &mut RunTimeData) { fconst_n(thread_id, runtime_data, 0.) }

pub fn fconst_1(thread_id: usize, runtime_data: &mut RunTimeData) { fconst_n(thread_id, runtime_data, 1.) }

pub fn fconst_2(thread_id: usize, runtime_data: &mut RunTimeData) { fconst_n(thread_id, runtime_data, 2.) }
