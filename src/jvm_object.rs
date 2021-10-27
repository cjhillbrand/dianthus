use jvm_value::JvmValue;
use std::collections::{HashMap};

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct JvmObject
{
    // name index to value.
    fields: HashMap<u16, JvmValue>
}