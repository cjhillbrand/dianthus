use crate::entities::constants::class_info::ClassInfo;
use crate::entities::constants::constant_info::ConstantInfo;
use crate::entities::constants::double_info::DoubleInfo;
use crate::entities::constants::field_ref_info::FieldRefInfo;
use crate::entities::constants::float_info::FloatInfo;
use crate::entities::constants::integer_info::IntegerInfo;
use crate::entities::constants::interface_method_ref_info::InterfaceMethodRefInfo;
use crate::entities::constants::invoke_dynamic_info::InvokeDynamicInfo;
use crate::entities::constants::long_info::LongInfo;
use crate::entities::constants::method_handle_info::MethodHandleInfo;
use crate::entities::constants::method_ref_info::MethodRefInfo;
use crate::entities::constants::method_type_info::MethodTypeInfo;
use crate::entities::constants::name_and_type_info::NameAndTypeInfo;
use crate::entities::constants::string_info::StringInfo;
use crate::entities::constants::utf8_info::Utf8Info;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum ConstantContainer {
	ClassInfo(ClassInfo),
	DoubleInfo(DoubleInfo),
	FieldRefInfo(FieldRefInfo),
	FloatInfo(FloatInfo),
	IntegerInfo(IntegerInfo),
	InterfaceMethodInfo(InterfaceMethodRefInfo),
	InvokeDynamicInfo(InvokeDynamicInfo),
	LongInfo(LongInfo),
	MethodHandleInfo(MethodHandleInfo),
	MethodRefInfo(MethodRefInfo),
	MethodTypeInfo(MethodTypeInfo),
	NameAndTypeInfo(NameAndTypeInfo),
	StringInfo(StringInfo),
	Utf8Info(Utf8Info),
	None
}

impl ConstantContainer {
	pub fn get_string(&self) -> String {
		match self {
			ConstantContainer::Utf8Info(v) => v.get_string().to_string(),
			_ => {
				panic!("Not a UTF8info constant: {:#?}", self)
			}
		}
	}
}

impl ConstantInfo for ConstantContainer {
	fn tag(&self) -> &u8 {
		match self {
			ConstantContainer::ClassInfo(v) => v.tag(),
			ConstantContainer::DoubleInfo(v) => v.tag(),
			ConstantContainer::FieldRefInfo(v) => v.tag(),
			ConstantContainer::FloatInfo(v) => v.tag(),
			ConstantContainer::IntegerInfo(v) => v.tag(),
			ConstantContainer::InterfaceMethodInfo(v) => v.tag(),
			ConstantContainer::InvokeDynamicInfo(v) => v.tag(),
			ConstantContainer::LongInfo(v) => v.tag(),
			ConstantContainer::MethodHandleInfo(v) => v.tag(),
			ConstantContainer::MethodRefInfo(v) => v.tag(),
			ConstantContainer::MethodTypeInfo(v) => v.tag(),
			ConstantContainer::NameAndTypeInfo(v) => v.tag(),
			ConstantContainer::StringInfo(v) => v.tag(),
			ConstantContainer::Utf8Info(v) => v.tag(),
			ConstantContainer::None => panic!("Tag not defined for index 0 of constant pool")
		}
	}
}
