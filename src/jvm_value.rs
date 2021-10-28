#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum JvmValue {
	Boolean(bool),
	Byte(u8),
	Char(char),
	Short(i16),
	Int(i32),
	Float(f32),
	Reference(u32),
	ReturnAddress(u32),
	Long(i64),
	Double(f64),
	PlaceHolder
}

pub fn to_int(value: JvmValue) -> JvmValue {
	match value {
		JvmValue::Boolean(v) => JvmValue::Int(v as i32),
		JvmValue::Byte(v) => JvmValue::Int(v as i32),
		JvmValue::Char(v) => JvmValue::Int(v as i32),
		JvmValue::Short(v) => JvmValue::Int(v as i32),
		JvmValue::Int(v) => JvmValue::Int(v as i32),
		JvmValue::Float(v) => JvmValue::Int(v as i32),
		JvmValue::Reference(v) => JvmValue::Int(v as i32),
		JvmValue::ReturnAddress(v) => JvmValue::Int(v as i32),
		JvmValue::Long(v) => JvmValue::Int(v as i32),
		JvmValue::Double(v) => JvmValue::Int(v as i32),
		JvmValue::PlaceHolder => panic!("Cannot convert a placeholder to int")
	}
}

impl Eq for JvmValue {}
