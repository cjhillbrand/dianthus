use std::collections::VecDeque;

use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use stack_frame::StackFrame;

pub fn bipush(thread_id: usize, runtime_data: &mut RunTimeData, code: &CodeAttribute)
{
	let pc: usize = runtime_data.get_pc(thread_id);
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	let raw_value: i32 = code.get_code()[pc + 1] as i32;
	let jvm_value: JvmValue = JvmValue::Int(raw_value);
	current_stack_frame.push_on_stack(jvm_value);
	runtime_data.increment_pc(thread_id, 2);
}

pub fn iadd(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	let lhs_value: JvmValue = current_stack_frame.pop_stack();
	let rhs_value: JvmValue = current_stack_frame.pop_stack();
	let raw_lhs = match lhs_value {
		JvmValue::Int(v) => v,
		_ => panic!("Expected integer value.")
	};
	let raw_rhs = match rhs_value {
		JvmValue::Int(v) => v,
		_ => panic!("Expected integer value")
	};

	let result: JvmValue = JvmValue::Int(raw_lhs + raw_rhs);
	current_stack_frame.push_on_stack(result);
	runtime_data.increment_pc(thread_id, 1);
}

fn iconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: i32) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};
	let value: JvmValue = JvmValue::Int(value);
	current_stack_frame.push_on_stack(value);
	runtime_data.increment_pc(thread_id, 1);
}

pub fn iconst_m1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, -1)
}

pub fn iconst_0(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, 0)
}

pub fn iconst_1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, 1)
}

pub fn iconst_2(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, 2)
}

pub fn iconst_3(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, 3)
}

pub fn iconst_4(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, 4)
}

pub fn iconst_5(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	iconst_n(thread_id, runtime_data, 5)
}

fn iload_n(thread_id: usize, runtime_data: &mut RunTimeData, index: usize)
{
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	let value: JvmValue = current_stack_frame.get_local_var(index);
	current_stack_frame.push_on_stack(value);
	runtime_data.increment_pc(thread_id, 1);
}

pub fn iload_0(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
	iload_n(thread_id, runtime_data, 0);
}

pub fn iload_1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
	iload_n(thread_id, runtime_data, 1);
}

pub fn iload_2(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
	iload_n(thread_id, runtime_data, 2);
}

pub fn iload_3(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
	iload_n(thread_id, runtime_data, 3);
}

fn istore_n(thread_id: usize, runtime_data: &mut RunTimeData, index: usize) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	let value: JvmValue = current_stack_frame.pop_stack();
	current_stack_frame.set_local_var(value, index);
	runtime_data.increment_pc(thread_id, 1);
}

pub fn istore_0(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	istore_n(thread_id, runtime_data, 0);
}

pub fn istore_1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	istore_n(thread_id, runtime_data, 1);
}

pub fn istore_2(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	istore_n(thread_id, runtime_data, 2);
}

pub fn istore_3(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute) {
	istore_n(thread_id, runtime_data, 3);
}
