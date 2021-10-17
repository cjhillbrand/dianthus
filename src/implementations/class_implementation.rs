use run_time_data::RunTimeData;
use stack_frame::StackFrame;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use std::collections::VecDeque;

pub fn return_op(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
    stack.pop_front();
}