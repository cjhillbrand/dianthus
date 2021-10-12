#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
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
