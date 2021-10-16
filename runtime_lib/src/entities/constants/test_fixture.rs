#[cfg(test)]
pub mod model_builder {
	use crate::entities::constants::class_info::ClassInfo;
	use crate::entities::constants::constant_container::ConstantContainer;
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

	pub fn create_class_info() -> ClassInfo { ClassInfo::new_test_model(1, 5) }

	pub fn create_double_info() -> DoubleInfo { DoubleInfo::new_test_model(1, 2.) }

	pub fn create_field_ref_info() -> FieldRefInfo { FieldRefInfo::new_test_model(1, 1, 2) }

	pub fn create_float_info() -> FloatInfo { FloatInfo::new_test_model(1, 1.) }

	pub fn create_integer_info() -> IntegerInfo { IntegerInfo::new_test_model(1, -1) }

	pub fn create_interface_method_ref_info() -> InterfaceMethodRefInfo {
		InterfaceMethodRefInfo::new_test_model(1, 2, 3)
	}

	pub fn create_invoke_dynamic_info() -> InvokeDynamicInfo { InvokeDynamicInfo::new_test_model(1, 2, 3) }

	pub fn create_long_info() -> LongInfo { LongInfo::new_test_model(1, 65537) }

	pub fn create_method_handle_info() -> MethodHandleInfo { MethodHandleInfo::new_test_model(1, 2, 3) }

	pub fn create_method_ref_info() -> MethodRefInfo { MethodRefInfo::new_test_model(1, 2, 3) }

	pub fn create_method_type_info() -> MethodTypeInfo { MethodTypeInfo::new_test_model(1, 2) }

	pub fn create_name_and_type_info() -> NameAndTypeInfo { NameAndTypeInfo::new_test_model(1, 2, 3) }

	pub fn create_string_info() -> StringInfo { StringInfo::new_test_model(1, 1) }

	pub fn create_utf8_info() -> Utf8Info { Utf8Info::new_test_model(1, "UTF".to_string()) }

	pub fn create_constant_pool() -> Vec<ConstantContainer> {
		vec![
			ConstantContainer::Utf8Info(create_utf8_info()),
			ConstantContainer::Utf8Info(create_utf8_info()),
			ConstantContainer::ClassInfo(create_class_info()),
			ConstantContainer::ClassInfo(create_class_info()),
		]
	}
}
