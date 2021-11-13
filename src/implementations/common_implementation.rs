use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use stack_frame::StackFrame;
use std::collections::VecDeque;

pub fn load_0(thread_id: usize, runtime_data: &mut RunTimeData) {
    load_n(thread_id, runtime_data, 0, 1);
}

pub fn load_1(thread_id: usize, runtime_data: &mut RunTimeData) {
    load_n(thread_id, runtime_data, 1, 1);
}

pub fn load_2(thread_id: usize, runtime_data: &mut RunTimeData) {
    load_n(thread_id, runtime_data, 2, 1);
}

pub fn load_3(thread_id: usize, runtime_data: &mut RunTimeData) {
    load_n(thread_id, runtime_data, 3, 1);
}

fn load_n(thread_id: usize, runtime_data: &mut RunTimeData, index: usize, pc_increment: usize) {
    let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

    let value: JvmValue = stack_frame.get_local_var(index);
    stack_frame.push_on_stack(value);
    stack_frame.increment_pc(pc_increment);
}


pub fn store_0(thread_id: usize, runtime_data: &mut RunTimeData) { store_n(thread_id, runtime_data, 0, 1); }

pub fn store_1(thread_id: usize, runtime_data: &mut RunTimeData) { store_n(thread_id, runtime_data, 1, 1); }

pub fn store_2(thread_id: usize, runtime_data: &mut RunTimeData) { store_n(thread_id, runtime_data, 2, 1); }

pub fn store_3(thread_id: usize, runtime_data: &mut RunTimeData) { store_n(thread_id, runtime_data, 3, 1); }


pub fn store_n(thread_id: usize, runtime_data: &mut RunTimeData, index: usize, pc_increment: usize) {
    let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);

    let value: JvmValue = stack_frame.pop_stack();
    stack_frame.set_local_var(value, index);
    stack_frame.increment_pc(pc_increment);
}

pub fn return_value<T>(thread_id: usize, runtime_data: &mut RunTimeData, cast: T)
    where T : FnOnce(JvmValue) -> JvmValue
{
    let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
    let current_stack_frame = match stack.front_mut() {
        Some(frame) => frame,
        None => {
            panic!("could not resolve stack frame.")
        }
    };

    let return_value: JvmValue = current_stack_frame.pop_stack();
    stack.pop_front();
    let casted_value: JvmValue = cast(return_value);
    let new_stack_frame = match stack.front_mut() {
        Some(frame) => frame,
        None => {
            panic!("could not resolve stack frame.")
        }
    };

    new_stack_frame.push_on_stack(casted_value);
    new_stack_frame.increment_pc(1)
}