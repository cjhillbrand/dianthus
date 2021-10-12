use std::collections::VecDeque;

use jvm_value::JvmValue;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StackFrame {
	local_variables: Vec<JvmValue>,
	operand_stack: VecDeque<JvmValue>
}
