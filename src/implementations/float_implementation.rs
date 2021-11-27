use implementations::common_implementation::return_value;
use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;

fn fconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: f32) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

	let value: JvmValue = JvmValue::Float(value);
	stack_frame.push_on_stack(value);
	stack_frame.increment_pc(1);
}

pub fn fconst_0(thread_id: usize, runtime_data: &mut RunTimeData) { fconst_n(thread_id, runtime_data, 0.) }

pub fn fconst_1(thread_id: usize, runtime_data: &mut RunTimeData) { fconst_n(thread_id, runtime_data, 1.) }

pub fn fconst_2(thread_id: usize, runtime_data: &mut RunTimeData) { fconst_n(thread_id, runtime_data, 2.) }

pub fn f2i(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Int(operand as i32);
	conversion(stack_frame, op)
}

pub fn f_add(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f32, rhs: f32| lhs + rhs;
	binop(stack_frame, op);
}

pub fn f_sub(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f32, rhs: f32| lhs - rhs;
	binop(stack_frame, op);
}

pub fn f_cmp(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: f32 = lhs.float();
	let rhs_value: f32 = rhs.float();

	let result_value = if lhs_value > rhs_value {
		1
	} else if lhs_value == rhs_value {
		0
	} else {
		-1
	};

	let result = JvmValue::Int(result_value);
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}

pub fn f_div(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f32, rhs: f32| lhs / rhs;
	binop(stack_frame, op);
}

pub fn f_mul(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f32, rhs: f32| lhs * rhs;
	binop(stack_frame, op);
}

pub fn f_neg(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let operand: JvmValue = stack_frame.pop_stack();
	let operand_value: f32 = operand.float();

	let result: JvmValue = JvmValue::Float(-operand_value);
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}

pub fn f_rem(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f32, rhs: f32| lhs % rhs;
	binop(stack_frame, op);
}

pub fn f_return(thread_id: usize, runtime_data: &mut RunTimeData) {
	let cast = |operand: JvmValue| operand.to_float();
	return_value(thread_id, runtime_data, cast);
}

pub fn f2l(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Long(operand as i64);
	conversion(stack_frame, op)
}

fn conversion<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(f32) -> JvmValue {
	let operand: f32 = stack_frame.pop_stack().float();
	stack_frame.push_on_stack(op(operand));
	stack_frame.increment_pc(1);
}

fn binop<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(f32, f32) -> f32 {
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: f32 = lhs.float();
	let rhs_value: f32 = rhs.float();

	let result = JvmValue::Float(op(lhs_value, rhs_value));
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}
