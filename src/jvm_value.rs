use jvm_object::JvmObject;
use runtime_lib::entities::constants::constant_container::ConstantContainer;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum JvmValue {
	Boolean(bool),
	Byte(u8),
	Char(char),
	Short(i16),
	Int(i32),
	Float(f32),
	Reference(u64),
	StaticReference(String),
	ReturnAddress(u32),
	Long(i64),
	Double(f64),
	PlaceHolder
}

impl Clone for JvmValue {
	fn clone(&self) -> Self {
		match self {
			JvmValue::Boolean(v) => JvmValue::Boolean(*v),
			JvmValue::Byte(v) => JvmValue::Byte(*v),
			JvmValue::Char(v) => JvmValue::Char(*v),
			JvmValue::Short(v) => JvmValue::Short(*v),
			JvmValue::Int(v) => JvmValue::Int(*v),
			JvmValue::Float(v) => JvmValue::Float(*v),
			JvmValue::Reference(v) => JvmValue::Reference(*v),
			JvmValue::ReturnAddress(v) => JvmValue::ReturnAddress(*v),
			JvmValue::Long(v) => JvmValue::Long(*v),
			JvmValue::Double(v) => JvmValue::Double(*v),
			JvmValue::PlaceHolder => JvmValue::PlaceHolder,
			JvmValue::StaticReference(v) => JvmValue::StaticReference(v.to_string())
		}
	}
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
		JvmValue::PlaceHolder => panic!("Cannot convert a placeholder to int"),
		JvmValue::StaticReference(_) => panic!("Cannot convert a static reference to int")
	}
}

pub fn to_jvm_value(constants: &[ConstantContainer], index: usize) -> (Option<JvmValue>, Option<JvmObject>) {
	let constant: &ConstantContainer = &constants[index];
	match constant {
		ConstantContainer::StringInfo(v) => {
			let utf8_info: &ConstantContainer = &constants[v.get_string_index() as usize];
			let mut contents: Vec<JvmValue> = Vec::new();

			for c in utf8_info.get_string().chars() {
				contents.push(JvmValue::Char(c));
			}

			(None, Some(JvmObject::build_array(contents)))
		}
		_ => panic!("Conversion not defined for {:#?}", constant)
	}
}

impl Eq for JvmValue {}
