use heap::Heap;
use implementations::invoke_init::invoke_init;
use jvm_object::JvmObject;
use jvm_value::JvmValue;
use std::collections::VecDeque;

use run_time_data::RunTimeData;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::constants::constant_container::ConstantContainer;
use runtime_lib::entities::constants::field_ref_info::FieldRefInfo;
use runtime_lib::entities::constants::name_and_type_info::NameAndTypeInfo;
use stack_frame::StackFrame;

pub fn return_op(thread_id: usize, runtime_data: &mut RunTimeData) {
	let stack: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	stack.pop_front();
}

pub fn get_static(thread_id: usize, runtime_data: &mut RunTimeData)
{
	let stack: &VecDeque<StackFrame> = runtime_data.get_stack(thread_id);
	let current_stack_frame = match stack.front() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	let executing_class: &str = current_stack_frame.get_executing_class();
	let class: &ClassStruct = runtime_data.get_class(executing_class);
	let constant_pool: &Vec<ConstantContainer> = class.get_constant_pool();
	let pc: usize = current_stack_frame.get_pc();
	let invoke_byte_1: u16 = current_stack_frame.get_code()[pc + 1] as u16;
	let invoke_byte_2: u16 = current_stack_frame.get_code()[pc + 2] as u16;
	let field_ref_index: u16 = invoke_byte_1 << 8 | invoke_byte_2;

	let field_ref: &FieldRefInfo = match &constant_pool[method_ref_index as usize] {
		ConstantContainer::FieldRefInfo(v) => v,
		_ => panic!("expected FieldRefInfo. Index: {}", method_ref_index)
	};

	let name_and_type_index: usize = field_ref.get_name_and_type_index() as usize;
	let class_index: usize = field_ref.get_class_index() as usize;
	let name_and_type_info: &NameAndTypeInfo = match &constant_pool[name_and_type_index as usize] {
		ConstantContainer::NameAndTypeInfo(v) => v,
		_ => panic!("expected NameAndTypeInfo. Index: {}", name_and_type_index)
	};

	let name_index: usize = name_and_type_info.get_name_index() as usize;
	let descriptor_index: usize = name_and_type_info.get_descriptor_index() as usize;
	let field_name: String = constant_pool[name_index].get_string();
	let _field_signature: String = constant_pool[descriptor_index].get_string();

	let next_class_index: usize = match &constant_pool[class_index] {
		ConstantContainer::ClassInfo(v) => v.name_index() as usize,
		_ => panic!("Expected ClassInfo at index: {}", class_index)
	};

	let next_class_name: &str = &constant_pool[next_class_index].get_string();
	if invoke_init(thread_id, runtime_data, next_class_name)
	{
		// places the consturctor stack frame on stack. Return to execute that.
		// up until this point the current stack_frame has not mutted so pause - construct
		// then on the next visit this SHOULD return false.
		return;
	}

	let heap: &Heap = runtime_data.get_heap();
	let obj: &JvmObject = heap.get_static_value(next_class_name);
	let field_value: &JvmValue = obj.get_value(field_name);

	// have to init the class to load values on heap.
	let mut current_stack_frame_mut: &StackFrame = match stack.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	current_stack_frame_mut.push_on_stack(field_value.clone());
	current_stack_frame_mut.increment_pc(1);
}