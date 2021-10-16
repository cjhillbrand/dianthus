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
			local_variables: Vec::with_capacity(local_num),
			operand_stack: VecDeque::with_capacity(max_stack)
		}
	}
}
