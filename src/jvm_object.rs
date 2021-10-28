use std::collections::HashMap;

use jvm_value::JvmValue;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct JvmObject {
	// name index to value.
	fields: HashMap<u16, JvmValue>
}
