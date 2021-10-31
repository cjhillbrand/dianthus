use std::collections::VecDeque;

use implementations::invoke_init::invoke_class_init;
use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::constants::constant_container::ConstantContainer;
use runtime_lib::entities::constants::method_ref_info::MethodRefInfo;
use runtime_lib::entities::constants::name_and_type_info::NameAndTypeInfo;
use runtime_lib::entities::method_info::MethodInfo;
use stack_frame::StackFrame;

pub fn invoke_static(thread_id: usize, runtime_data: &mut RunTimeData) {
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
	let method_ref_index: u16 = invoke_byte_1 << 8 | invoke_byte_2;

	let method_ref: &MethodRefInfo = match &constant_pool[method_ref_index as usize] {
		ConstantContainer::MethodRefInfo(v) => v,
		_ => panic!("expected MethodRefInfo. Index: {}", method_ref_index)
	};

	let name_and_type_index: usize = method_ref.get_name_and_type_index() as usize;
	let class_index: usize = method_ref.get_class_index() as usize;
	let name_and_type_info: &NameAndTypeInfo = match &constant_pool[name_and_type_index as usize] {
		ConstantContainer::NameAndTypeInfo(v) => v,
		_ => panic!("expected NameAndTypeInfo. Index: {}", name_and_type_index)
	};

	let name_index: usize = name_and_type_info.get_name_index() as usize;
	let descriptor_index: usize = name_and_type_info.get_descriptor_index() as usize;
	let method_name: String = constant_pool[name_index].get_string();
	let method_signature: String = constant_pool[descriptor_index].get_string();

	let next_class_index: usize = match &constant_pool[class_index] {
		ConstantContainer::ClassInfo(v) => v.name_index() as usize,
		_ => panic!("Expected ClassInfo at index: {}", class_index)
	};

	let next_class_name: &str = &constant_pool[next_class_index].get_string();
	if !runtime_data.is_class_loaded(&class_name) {
		invoke_class_init(thread_id, runtime_data, &next_class_name);
		// places the constructor stack frame on stack. Return to execute that.
		// up until this point the current stack_frame has not mutted so pause - construct
		// then on the next visit this SHOULD return false.
		return;
	}

	let next_class: &ClassStruct = runtime_data.get_class(next_class_name);

	let method: &MethodInfo = next_class.get_method(&method_name);
	let code: &CodeAttribute = method.derive_code_attribute();
	let mut next_frame: StackFrame = StackFrame::create_stack_frame(executing_class, code);

	let mut range = get_args_count(&method_signature);
	for mut i in 0..range {
		let value: &JvmValue = current_stack_frame.get_stack_value(i);
		next_frame.set_local_var(value.clone(), i);
		match value {
			JvmValue::Long(_v) => {
				i += 1;
				range += 1;
				next_frame.set_local_var(JvmValue::PlaceHolder, i);
			}
			JvmValue::Double(_v) => {
				i += 1;
				range += 1;
				next_frame.set_local_var(JvmValue::PlaceHolder, i);
			}
			_ => {}
		};
	}

	let stack_mut: &mut VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);
	let current_stack_frame: &mut StackFrame = match stack_mut.front_mut() {
		Some(frame) => frame,
		None => {
			panic!("could not resolve stack frame.")
		}
	};

	current_stack_frame.increment_pc(3);
	runtime_data.push_stack(next_frame, thread_id);
}

fn get_args_count(signature: &str) -> usize {
	let start_index: usize = match signature.find('(') {
		Some(v) => v,
		None => panic!("Could not find '(' in string {}", signature)
	};

	let end_index: usize = match signature.find(')') {
		Some(v) => v,
		None => panic!("Could not find ')' in string {}", signature)
	};

	end_index - start_index - 1
}
