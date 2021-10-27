use std::collections::VecDeque;

use jvm_value::JvmValue;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StackFrame {
	local_variables: Vec<JvmValue>,
	operand_stack: VecDeque<JvmValue>,
	code_attribute: Box<CodeAttribute>,
	program_counter: usize,
	executing_class: String
}

impl StackFrame {
	pub fn new(local_num: usize, max_stack: usize, code_attribute: Box<CodeAttribute>, executing_class: String) -> StackFrame {
		StackFrame {
			local_variables: vec![JvmValue::PlaceHolder; local_num],
			operand_stack: VecDeque::with_capacity(max_stack),
			code_attribute,
			program_counter: 0,
			executing_class
		}
	}

	pub fn get_pc(&self) -> usize { self.program_counter }

	pub fn get_executing_class(&self) -> &str { &self.executing_class }

	pub fn increment_pc(&mut self, increment: usize) { self.program_counter += increment }

	pub fn get_code(&self) -> &Vec<u8> { self.code_attribute.get_code() }

	pub fn push_on_stack(&mut self, value: JvmValue) { self.operand_stack.push_front(value); }

	pub fn get_operand_stack_mut(&mut self) -> &mut VecDeque<JvmValue> { &mut self.operand_stack }

	pub fn pop_stack(&mut self) -> JvmValue {
		match self.operand_stack.pop_front() {
			Some(value) => value,
			None => panic!("Stack is empty :(")
		}
	}

	pub fn get_stack_value(&self, index: usize) -> &JvmValue
	{
		&self.operand_stack[index]
	}

	pub fn get_local_var(&self, index: usize) -> JvmValue { self.local_variables[index].clone() }

	pub fn set_local_var(&mut self, value: JvmValue, index: usize) { self.local_variables[index] = value; }

	pub fn create_stack_frame(class_name: &str, code_attribute: &CodeAttribute) -> StackFrame {
		StackFrame::new(
			code_attribute.get_max_locals() as usize,
			code_attribute.get_max_stack() as usize,
			// We should not be cloning here....
			Box::new(code_attribute.clone()),
			class_name.to_string()
		)
	}
}
