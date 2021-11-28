use run_time_data::RunTimeData;
use stack_frame::StackFrame;

pub fn if_icmpeq(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs == rhs;
    cmp(thread_id, runtime_data, comparer);
}

pub fn if_icmpne(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs != rhs;
    cmp(thread_id, runtime_data, comparer);
}

pub fn if_icmplt(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs < rhs;
    cmp(thread_id, runtime_data, comparer);
}

pub fn if_icmpgt(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs > rhs;
    cmp(thread_id, runtime_data, comparer);
}

pub fn if_icmple(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs <= rhs;
    cmp(thread_id, runtime_data, comparer);
}

pub fn if_icmpge(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs >= rhs;
    cmp(thread_id, runtime_data, comparer);
}

pub fn if_eq(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs == rhs;
    cmp_0(thread_id, runtime_data, comparer);
}

pub fn if_ne(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs != rhs;
    cmp_0(thread_id, runtime_data, comparer);
}

pub fn if_lt(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs < rhs;
    cmp_0(thread_id, runtime_data, comparer);
}

pub fn if_gt(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs > rhs;
    cmp_0(thread_id, runtime_data, comparer);
}

pub fn if_le(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs <= rhs;
    cmp_0(thread_id, runtime_data, comparer);
}

pub fn if_ge(thread_id: usize, runtime_data: &mut RunTimeData) {
    let comparer = |lhs: i32, rhs: i32| lhs >= rhs;
    cmp_0(thread_id, runtime_data, comparer);
}

fn cmp<T>(thread_id: usize, runtime_data: &mut RunTimeData, comparer: T)
    where T: FnOnce(i32, i32) -> bool {
    let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
    let rhs: i32 = stack_frame.pop_stack().i32();
    let lhs: i32 = stack_frame.pop_stack().i32();
    let offset: usize = calc_offset(comparer(lhs, rhs), stack_frame);

    stack_frame.increment_pc(offset);
}

fn cmp_0<T>(thread_id: usize, runtime_data: &mut RunTimeData, comparer: T)
    where T: FnOnce(i32, i32) -> bool {
    let stack_frame: &mut StackFrame = runtime_data.get_current_stack_frame_mut(thread_id);
    let rhs: i32 = stack_frame.pop_stack().i32();
    let lhs: i32 = 0;
    let offset: usize = calc_offset(comparer(lhs, rhs), stack_frame);
    stack_frame.increment_pc(offset);
}

fn calc_offset(is_true: bool, stack_frame: &mut StackFrame) -> usize
{
    if is_true {
        let pc: usize = stack_frame.get_pc();
        ((stack_frame.get_code()[pc + 1] as u16) << 8 |
            (stack_frame.get_code()[pc + 2] as u16)) as usize
    } else { 3 }
}