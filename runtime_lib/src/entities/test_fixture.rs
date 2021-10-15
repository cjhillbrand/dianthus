#[cfg(test)]
pub mod model_builder {
	use crate::entities::attributes::test_fixture::model_builder::create_attributes;
	use crate::entities::class_struct::ClassStruct;
	use crate::entities::constants::test_fixture::model_builder::create_constant_pool;
	use crate::entities::field_info::FieldInfo;
	use crate::entities::method_info::MethodInfo;
	use crate::hashmap;

	pub fn create_field() -> FieldInfo {
		FieldInfo::new_test_model(
			0,
			"FIELDINFO".to_string(),
			"descriptor".to_string(),
			create_attributes()
		)
	}

	pub fn create_method() -> MethodInfo {
		MethodInfo::new_test_model(
			0,
			"FIELDINFO".to_string(),
			"descriptor".to_string(),
			create_attributes()
		)
	}

	pub fn create_class() -> ClassStruct {
		ClassStruct::new_test_model(
			1,
			2,
			3,
			create_constant_pool(),
			1,
			1,
			2,
			vec![],
			hashmap!["something".to_string() => create_field()],
			hashmap!["something".to_string() => create_method()],
			create_attributes()
		)
	}
}
