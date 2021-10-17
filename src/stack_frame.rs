use std::collections::VecDeque;

use jvm_value::JvmValue;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StackFrame {
	local_variables: Vec<JvmValue>,
	operand_stack: VecDeque<JvmValue>
}

impl StackFrame {
	pub fn new(local_num: usize, max_stack: usize) -> StackFrame {
		StackFrame {
			local_variables: vec![JvmValue::PlaceHolder; local_num],
			operand_stack: VecDeque::with_capacity(max_stack)
		}
	}

	pub fn push_on_stack(&mut self, value: JvmValue)
	{
		self.operand_stack.push_front(value);
	}

	pub fn get_operand_stack_mut(&mut self) -> &mut VecDeque<JvmValue>
	{
		&mut self.operand_stack
	}

	pub fn pop_stack(&mut self) -> JvmValue
	{
		match self.operand_stack.pop_front()
		{
			Some(value) => { value },
			None => panic!("Stack is empty :(")
		}
	}

	pub fn set_local_var(&mut self, value: JvmValue, index: usize)
	{
		self.local_variables[index] = value;
	}
}
