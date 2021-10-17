use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use stack_frame::StackFrame;
use jvm_value::JvmValue;

use std::collections::VecDeque;

fn iconst_n(thread_id: usize, runtime_data: &mut RunTimeData, value: i32)
{
    let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id.clone());
    let current_stack_frame = match stack.front_mut()
    {
        Some(frame) => { frame },
        None => { panic!("could not resolve stack frame.") }
    };
    let value: JvmValue = JvmValue::Int(value);
    current_stack_frame.push_on_stack(value);
    runtime_data.increment_pc(thread_id, 1);
}

pub fn iconst_m1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, -1)
}

pub fn iconst_0(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, 0)
}

pub fn iconst_1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, 1)
}

pub fn iconst_2(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, 2)
}

pub fn iconst_3(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, 3)
}

pub fn iconst_4(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, 4)
}

pub fn iconst_5(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    iconst_n(thread_id, runtime_data, 5)
}

fn istore_n(thread_id: usize, runtime_data: &mut RunTimeData, index: usize)
{
    let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id.clone());
    let current_stack_frame = match stack.front_mut()
    {
        Some(frame) => { frame },
        None => { panic!("could not resolve stack frame.") }
    };

    let value: JvmValue = current_stack_frame.pop_stack();
    current_stack_frame.set_local_var(value, index);
}

pub fn istore_0(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    istore_n(thread_id, runtime_data, 0);
}

pub fn istore_1(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    istore_n(thread_id, runtime_data, 1);
}

pub fn istore_2(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    istore_n(thread_id, runtime_data, 2);
}

pub fn istore_3(thread_id: usize, runtime_data: &mut RunTimeData, _code: &CodeAttribute)
{
    istore_n(thread_id, runtime_data, 3);
}