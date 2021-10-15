/*use runtime_lib::entities::attributes::attribute_container::AttributeContainer;
use runtime_lib::entities::attributes::constant_value_attribute::ConstantValueAttribute;
use runtime_lib::entities::method_info::MethodInfo;
use runtime_lib::entities::field_info::FieldInfo;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::constants::constant_container::ConstantContainer;
use runtime_lib::entities::constants::utf8_info::Utf8Info;

#[test]
fn class_struct_constructs_expected_struct() {
	let mut data: VecDeque<u8> = get_default_vec();
	let result: ClassStruct = ClassStruct::new(&mut data);

	assert_eq!(0xCAFEBABE, result.magic);
	assert_eq!(256, result.minor_version);
	assert_eq!(1, result.major_version);
	assert_eq!(2, result.constant_pool_count);
	assert_eq!(2, result.constant_pool.len());

	let mut constant_pool_vec =
		vecdeque![1, 0, 13, b'C', b'o', b'n', b's', b't', b'a', b'n', b't', b'V', b'a', b'l', b'u', b'e'];
	let expected_constant = ConstantContainer::Utf8Info(Utf8Info::new(&mut constant_pool_vec));
	assert_eq!(expected_constant, result.constant_pool[1]);
	assert_eq!(1, result.access_flags);
	assert_eq!(1, result.this_class);
	assert_eq!(2, result.super_class);
	assert_eq!(2, result.interfaces_count);
	assert_eq!(2, result.interfaces.len());

	let expected_interfaces = vec![2, 4];
	assert_eq!(expected_interfaces, result.interfaces);

	assert_eq!(1, result.fields_count);
	assert_eq!(1, result.field_info.len());

	let mut expected_field_content = vecdeque![0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 2, 2, 0, 0, 1, 1];
	let expected_field = FieldInfo::new(&mut expected_field_content, &result.constant_pool);
	assert_eq!(expected_field, result.field_info[0]);

	assert_eq!(1, result.methods_count);
	assert_eq!(1, result.method_info.len());

	let mut expected_method_content = vecdeque![0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 2, 2, 0, 0, 1, 1];
	let expected_method = MethodInfo::new(&mut expected_method_content, &result.constant_pool);
	assert_eq!(expected_method, result.method_info[0]);

	assert_eq!(1, result.attributes_count);
	assert_eq!(1, result.attribute_info.len());

	let mut expected_attribute_content = vecdeque![0, 1, 2, 2, 0, 0, 1, 1];
	let expected_attribute =
		AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(&mut expected_attribute_content));
	assert_eq!(expected_attribute, result.attribute_info[0]);
}
	#[test]
	fn field_info_constructs_expected_struct() {
		let mut data: VecDeque<u8> = get_default_vec();
		let constant_pool = get_default_cp();
		let result: FieldInfo = FieldInfo::new(&mut data, &constant_pool);

		assert_eq!(1, result.access_flags);
		assert_eq!(123, result.name_index);
		assert_eq!(5, result.descriptor_index);
		assert_eq!(1, result.attributes_count);

		assert_eq!(1, result.attributes.len());

		let mut content_vec: VecDeque<u8> = vecdeque![0, 0, 1, 1, 1, 1, 0, 2];
		let expected_attribute: AttributeContainer =
			AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(&mut content_vec));
		assert_eq!(expected_attribute, result.attributes[0]);
	}

	#[test]
	fn method_info_constructs_expected_struct() {
		let mut data: VecDeque<u8> = get_default_vec();
		let constant_pool = get_default_cp();
		let result: MethodInfo = MethodInfo::new(&mut data, &constant_pool);

		assert_eq!(1, result.access_flags);
		assert_eq!(123, result.name_index);
		assert_eq!(5, result.descriptor_index);
		assert_eq!(1, result.attributes_count);

		assert_eq!(1, result.attributes.len());

		let mut content_vec: VecDeque<u8> = vecdeque![0, 0, 1, 1, 1, 1, 0, 2];
		let expected_attribute: AttributeContainer =
			AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(&mut content_vec));
		assert_eq!(expected_attribute, result.attributes[0]);
	}
*/
