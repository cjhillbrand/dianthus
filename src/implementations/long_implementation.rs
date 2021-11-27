use std::cmp::Ordering;

use implementations::common_implementation::return_value;
use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;

fn lconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: i64) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

	let value: JvmValue = JvmValue::Long(value);
	stack_frame.push_on_stack(value);
	stack_frame.increment_pc(1);
}

pub fn lconst_0(thread_id: usize, runtime_date: &mut RunTimeData) { lconst_n(thread_id, runtime_date, 0); }

pub fn lconst_1(thread_id: usize, runtime_data: &mut RunTimeData) { lconst_n(thread_id, runtime_data, 1); }

pub fn l_return(thread_id: usize, runtime_data: &mut RunTimeData) {
	let cast = |operand: JvmValue| operand.to_long();
	return_value(thread_id, runtime_data, cast);
}

pub fn l2d(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Double(operand as f64);
	conversion(stack_frame, op)
}

pub fn l2f(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Float(operand as f32);
	conversion(stack_frame, op)
}

pub fn l2i(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Int(operand as i32);
	conversion(stack_frame, op)
}

pub fn l_add(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs + rhs;
	binop(stack_frame, op);
}

pub fn l_div(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs / rhs;
	binop(stack_frame, op);
}

pub fn l_mul(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs * rhs;
	binop(stack_frame, op);
}

pub fn l_or(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: i64, rhs: i64| lhs | rhs;
	binop(stack_frame, op);
}

pub fn l_rem(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs % rhs;
	binop(stack_frame, op);
}

pub fn l_shl(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs << rhs;
	binop(stack_frame, op);
}

pub fn l_shr(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs >> rhs;
	binop(stack_frame, op);
}

pub fn l_sub(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs - rhs;
	binop(stack_frame, op);
}

pub fn l_ushr(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: i64, rhs: i64| ((lhs as u64) >> (rhs as u64)) as i64;
	binop(stack_frame, op);
}

pub fn l_xor(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs, rhs| lhs ^ rhs;
	binop(stack_frame, op);
}

pub fn l_and(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: i64, rhs: i64| lhs & rhs;
	binop(stack_frame, op);
}

pub fn l_cmp(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: i64 = lhs.long();
	let rhs_value: i64 = rhs.long();

	let result_value: i32 = match lhs_value.cmp(&rhs_value) {
		Ordering::Greater => 1,
		Ordering::Less => 0,
		Ordering::Equal => -1
	};

	let result = JvmValue::Int(result_value);
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}

pub fn l_neg(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let operand: JvmValue = stack_frame.pop_stack();
	let operand_value: i64 = operand.long();

	let result: JvmValue = JvmValue::Long(-operand_value);
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}

fn conversion<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(i64) -> JvmValue {
	let operand: i64 = stack_frame.pop_stack().long();
	stack_frame.push_on_stack(op(operand));
	stack_frame.increment_pc(1);
}

fn binop<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(i64, i64) -> i64 {
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: i64 = lhs.long();
	let rhs_value: i64 = rhs.long();

	let result = JvmValue::Long(op(lhs_value, rhs_value));
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}
