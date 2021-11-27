use implementations::common_implementation::return_value;
use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub fn dconst_0(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

	stack_frame.push_on_stack(JvmValue::Double(0.));
	stack_frame.increment_pc(1);
}

pub fn dconst_1(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

	stack_frame.push_on_stack(JvmValue::Double(1.));
	stack_frame.increment_pc(1);
}

pub fn d2f(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Float(operand as f32);
	conversion(stack_frame, op);
}

pub fn d2i(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Int(operand as i32);
	conversion(stack_frame, op);
}

pub fn d2l(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |operand| JvmValue::Long(operand as i64);
	conversion(stack_frame, op);
}

pub fn d_add(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f64, rhs: f64| lhs + rhs;
	binop(stack_frame, op);
}

pub fn d_cmp(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: f64 = lhs.double();
	let rhs_value: f64 = rhs.double();

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

pub fn d_div(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f64, rhs: f64| lhs / rhs;
	binop(stack_frame, op);
}

pub fn d_mul(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f64, rhs: f64| lhs * rhs;
	binop(stack_frame, op);
}

pub fn d_neg(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let operand: JvmValue = stack_frame.pop_stack();
	let operand_value: f64 = operand.double();

	let result: JvmValue = JvmValue::Double(-operand_value);
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}

pub fn d_rem(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f64, rhs: f64| lhs % rhs;
	binop(stack_frame, op);
}

pub fn d_return(thread_id: usize, runtime_data: &mut RunTimeData) {
	let cast = |operand: JvmValue| operand.to_double();
	return_value(thread_id, runtime_data, cast);
}

pub fn d_sub(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
	let op = |lhs: f64, rhs: f64| lhs - rhs;
	binop(stack_frame, op);
}

fn conversion<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(f64) -> JvmValue {
	let operand: f64 = stack_frame.pop_stack().double();
	stack_frame.push_on_stack(op(operand));
	stack_frame.increment_pc(1);
}

fn binop<T>(stack_frame: &mut StackFrame, op: T)
where T: FnOnce(f64, f64) -> f64 {
	let lhs: JvmValue = stack_frame.pop_stack();
	let rhs: JvmValue = stack_frame.pop_stack();
	let lhs_value: f64 = lhs.double();
	let rhs_value: f64 = rhs.double();

	let result = JvmValue::Double(op(lhs_value, rhs_value));
	stack_frame.push_on_stack(result);
	stack_frame.increment_pc(1);
}
