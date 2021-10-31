use crate::entities::attributes::attribute_info::AttributeInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct StackMapTableAttribute {
	attribute_name: String,
	attribute_length: u32,
	number_of_entries: u16,
	entries: Vec<StackMapFrame>
}

impl AttributeInfo for StackMapTableAttribute {
	fn name(&self) -> &str { &self.attribute_name }

	fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl StackMapTableAttribute {
	pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> StackMapTableAttribute {
		let attribute_name: String = constant_pool[data.pop_u16() as usize].get_string();
		let attribute_length: u32 = data.pop_u32();
		let number_of_entries: u16 = data.pop_u16();
		let mut entries: Vec<StackMapFrame> = Vec::new();
		for _i in 0..number_of_entries {
			entries.push(StackMapFrame::new(data));
		}

		StackMapTableAttribute {
			attribute_name,
			attribute_length,
			number_of_entries,
			entries
		}
	}
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
enum StackMapFrame {
	Same(SameFrame),
	SameLocals1StackItem(SameLocals1StackItemFrame),
	SameLocals1StackItemExtended(SameLocals1StackItemFrameExtendedFrame),
	Chop(ChopFrame),
	SameExtended(SameFrameExtended),
	Append(AppendFrame),
	Full(FullFrame)
}

impl StackMapFrame {
	fn new<T: ReadBytes>(data: &mut T) -> StackMapFrame {
		let frame_type: u8 = data.peek_u8();
		match frame_type {
			0..=63 => StackMapFrame::Same(SameFrame::new(data)),
			64..=127 => StackMapFrame::SameLocals1StackItem(SameLocals1StackItemFrame::new(data)),
			247 => StackMapFrame::SameLocals1StackItemExtended(SameLocals1StackItemFrameExtendedFrame::new(data)),
			248..=250 => StackMapFrame::Chop(ChopFrame::new(data)),
			251 => StackMapFrame::SameExtended(SameFrameExtended::new(data)),
			252..=254 => StackMapFrame::Append(AppendFrame::new(data)),
			255 => StackMapFrame::Full(FullFrame::new(data)),
			_ => panic!("could not decide implementation for frame type: {}", frame_type)
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct SameFrame {
	frame_type: u8
}

impl SameFrame {
	fn new<T: ReadBytes>(data: &mut T) -> SameFrame {
		SameFrame {
			frame_type: data.pop_u8()
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct SameLocals1StackItemFrame {
	frame_type: u8,
	stack: Vec<VerificationTypeInfo>
}

impl SameLocals1StackItemFrame {
	fn new<T: ReadBytes>(data: &mut T) -> SameLocals1StackItemFrame {
		SameLocals1StackItemFrame {
			frame_type: data.pop_u8(),
			stack: vec![VerificationTypeInfo::new(data)]
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct SameLocals1StackItemFrameExtendedFrame {
	frame_type: u8,
	offset_delta: u16,
	stack: Vec<VerificationTypeInfo>
}

impl SameLocals1StackItemFrameExtendedFrame {
	fn new<T: ReadBytes>(data: &mut T) -> SameLocals1StackItemFrameExtendedFrame {
		SameLocals1StackItemFrameExtendedFrame {
			frame_type: data.pop_u8(),
			offset_delta: data.pop_u16(),
			stack: vec![VerificationTypeInfo::new(data)]
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct ChopFrame {
	frame_type: u8,
	offset_delta: u16
}

impl ChopFrame {
	fn new<T: ReadBytes>(data: &mut T) -> ChopFrame {
		ChopFrame {
			frame_type: data.pop_u8(),
			offset_delta: data.pop_u16()
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct SameFrameExtended {
	frame_type: u8,
	offset_delta: u16
}

impl SameFrameExtended {
	fn new<T: ReadBytes>(data: &mut T) -> SameFrameExtended {
		SameFrameExtended {
			frame_type: data.pop_u8(),
			offset_delta: data.pop_u16()
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct AppendFrame {
	frame_type: u8,
	offset_delta: u16,
	locals: Vec<VerificationTypeInfo>
}

impl AppendFrame {
	fn new<T: ReadBytes>(data: &mut T) -> AppendFrame {
		let frame_type: u8 = data.pop_u8();
		let offset_delta: u16 = data.pop_u16();
		let mut locals: Vec<VerificationTypeInfo> = Vec::new();
		for _i in 0..frame_type - 251 {
			locals.push(VerificationTypeInfo::new(data))
		}

		AppendFrame {
			frame_type,
			offset_delta,
			locals
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct FullFrame {
	frame_type: u8,
	offset_delta: u16,
	locals: Vec<VerificationTypeInfo>,
	stack: Vec<VerificationTypeInfo>
}

impl FullFrame {
	fn new<T: ReadBytes>(data: &mut T) -> FullFrame {
		FullFrame {
			frame_type: data.pop_u8(),
			offset_delta: data.pop_u16(),
			locals: {
				let num_locals: u16 = data.pop_u16();
				let mut locals: Vec<VerificationTypeInfo> = Vec::new();
				for _i in 0..num_locals {
					locals.push(VerificationTypeInfo::new(data))
				}
				locals
			},
			stack: {
				let num_stack: u16 = data.pop_u16();
				let mut stack: Vec<VerificationTypeInfo> = Vec::new();
				for _i in 0..num_stack {
					stack.push(VerificationTypeInfo::new(data))
				}
				stack
			}
		}
	}
}

const ITEM_TOP: u8 = 0;
const ITEM_INTEGER: u8 = 1;
const ITEM_FLOAT: u8 = 2;
const ITEM_LONG: u8 = 4;
const ITEM_DOUBLE: u8 = 3;
const ITEM_NULL: u8 = 5;
const ITEM_UNINITIALIZED_THIS: u8 = 6;
const ITEM_OBJECT: u8 = 7;
const ITEM_UNINITIALIZED: u8 = 8;
#[repr(u8)]
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
enum VerificationTypeInfo {
	TopVariableInfo = ITEM_TOP,
	IntegerVariableInfo = ITEM_INTEGER,
	FloatVariableInfo = ITEM_FLOAT,
	LongVariableInfo = ITEM_LONG,
	DoubleVariableInfo = ITEM_DOUBLE,
	NullVariableInfo = ITEM_NULL,
	UninitializedThisVariableInfo = ITEM_UNINITIALIZED_THIS,
	ObjectVariableInfo(ObjectVariableInfo),
	UninitializedVariableInfo(UninitializedVariableInfo)
}

impl VerificationTypeInfo {
	fn new<T: ReadBytes>(data: &mut T) -> VerificationTypeInfo {
		let tag: u8 = data.pop_u8();
		match tag {
			ITEM_TOP => VerificationTypeInfo::TopVariableInfo,
			ITEM_INTEGER => VerificationTypeInfo::IntegerVariableInfo,
			ITEM_FLOAT => VerificationTypeInfo::FloatVariableInfo,
			ITEM_LONG => VerificationTypeInfo::LongVariableInfo,
			ITEM_DOUBLE => VerificationTypeInfo::DoubleVariableInfo,
			ITEM_NULL => VerificationTypeInfo::NullVariableInfo,
			ITEM_UNINITIALIZED_THIS => VerificationTypeInfo::UninitializedThisVariableInfo,
			ITEM_OBJECT => {
				let object_info: ObjectVariableInfo = ObjectVariableInfo::new(tag as u8, data.pop_u16());
				VerificationTypeInfo::ObjectVariableInfo(object_info)
			}
			ITEM_UNINITIALIZED => {
				let uninitialized: UninitializedVariableInfo =
					UninitializedVariableInfo::new(tag as u8, data.pop_u16());
				VerificationTypeInfo::UninitializedVariableInfo(uninitialized)
			}
			_ => panic!("Could not construct verification type info from tag: {}", tag)
		}
	}
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct ObjectVariableInfo {
	tag: u8,
	constant_index: u16
}

impl ObjectVariableInfo {
	fn new(tag: u8, constant_index: u16) -> ObjectVariableInfo { ObjectVariableInfo { tag, constant_index } }
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct UninitializedVariableInfo {
	tag: u8,
	offset: u16
}

impl UninitializedVariableInfo {
	fn new(tag: u8, offset: u16) -> UninitializedVariableInfo { UninitializedVariableInfo { tag, offset } }
}
