use run_time_data::RunTimeData;
use stack_frame::StackFrame;
use jvm_value::JvmValue;
use std::collections::VecDeque;

fn lconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: i64)
{
    let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
    let current_stack_frame = match stack.front_mut() {
        Some(frame) => frame,
        None => {
            panic!("could not resolve stack frame.")
        }
    };

    let value: JvmValue = JvmValue::Long(value);
    current_stack_frame.push_on_stack(value);
    current_stack_frame.increment_pc(1);
}

pub fn lconst_0(thread_id: usize, runtime_date: &mut RunTimeData)
{
    lconst_n(thread_id, runtime_date, 0);
}

pub fn lconst_1(thread_id: usize, runtime_data: &mut RunTimeData)
{
    lconst_n(thread_id, runtime_data, 1);
}