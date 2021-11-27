use implementations::common_implementation::return_value;
use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub fn bipush(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

	let pc: usize = stack_frame.get_pc();
	let raw_value: i32 = stack_frame.get_code()[pc + 1] as i32;
	let jvm_value: JvmValue = JvmValue::Int(raw_value);
	stack_frame.push_on_stack(jvm_value);
	stack_frame.increment_pc(2);
}

fn iconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: i32) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

	let value: JvmValue = JvmValue::Int(value);
	stack_frame.push_on_stack(value);
	stack_frame.increment_pc(1);
}

pub fn iconst_m1(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, -1) }

pub fn iconst_0(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, 0) }

pub fn iconst_1(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, 1) }

pub fn iconst_2(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, 2) }

pub fn iconst_3(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, 3) }

pub fn iconst_4(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, 4) }

pub fn iconst_5(thread_id: usize, runtime_data: &mut RunTimeData) { iconst_n(thread_id, runtime_data, 5) }

pub fn i_return(thread_id: usize, runtime_data: &mut RunTimeData) {
	let cast = |operand: JvmValue| operand.to_int();
	return_value(thread_id, runtime_data, cast);
}

pub fn i2b(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let value: i32 = stack_frame.pop_stack().i32();

	let byte_value: JvmValue = JvmValue::Byte(value as u8);
	stack_frame.push_on_stack(byte_value);
	stack_frame.increment_pc(1)
}

pub fn i2c(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let value: i32 = stack_frame.pop_stack().i32();

	let byte_value: JvmValue = JvmValue::Char((value as u8) as char);
	stack_frame.push_on_stack(byte_value);
	stack_frame.increment_pc(1)
}

pub fn i2d(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let value: i32 = stack_frame.pop_stack().i32();

	let byte_value: JvmValue = JvmValue::Double(value as f64);
	stack_frame.push_on_stack(byte_value);
	stack_frame.increment_pc(1)
}

pub fn i2f(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let value: i32 = stack_frame.pop_stack().i32();

	let byte_value: JvmValue = JvmValue::Float(value as f32);
	stack_frame.push_on_stack(byte_value);
	stack_frame.increment_pc(1)
}

pub fn i2l(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let value: i32 = stack_frame.pop_stack().i32();

	let byte_value: JvmValue = JvmValue::Long(value as i64);
	stack_frame.push_on_stack(byte_value);
	stack_frame.increment_pc(1)
}

pub fn i2s(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let value: i32 = stack_frame.pop_stack().i32();

	let byte_value: JvmValue = JvmValue::Short(value as i16);
	stack_frame.push_on_stack(byte_value);
	stack_frame.increment_pc(1)
}

pub fn i_add(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs + rhs;
	binop(stack_frame, closure)
}

pub fn i_and(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs & rhs;
	binop(stack_frame, closure)
}

pub fn i_div(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs / rhs;
	binop(stack_frame, closure)
}

pub fn i_mul(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs * rhs;
	binop(stack_frame, closure)
}

pub fn i_or(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs | rhs;
	binop(stack_frame, closure)
}

pub fn i_rem(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs % rhs;
	binop(stack_frame, closure)
}

pub fn i_shl(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs << rhs;
	binop(stack_frame, closure)
}

pub fn i_shr(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs >> rhs;
	binop(stack_frame, closure)
}

pub fn i_sub(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs - rhs;
	binop(stack_frame, closure)
}

pub fn i_ushr(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs: i32, rhs: i32| ((lhs as u32) >> (rhs as u32)) as i32;
	binop(stack_frame, closure)
}

pub fn i_xor(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let closure = |lhs, rhs| lhs ^ rhs;
	binop(stack_frame, closure)
}

pub fn i_neg(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let operand: JvmValue = stack_frame.pop_stack();
	let operand_value: i32 = operand.i32();

	let result: JvmValue = JvmValue::Int(-operand_value);
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}

fn binop<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(i32, i32) -> i32 {
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: i32 = lhs.i32();
	let rhs_value: i32 = rhs.i32();

	let result = JvmValue::Int(op(lhs_value, rhs_value));
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}
