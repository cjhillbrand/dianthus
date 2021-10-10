
use crate::entities::read_bytes::ReadBytes;
use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::method_info::MethodInfo;
use crate::entities::field_info::FieldInfo;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::constants::constant_factory::get_constant_container;
use crate::entities::attributes::attribute_factory::get_attribute_container;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
struct ClassStruct {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<ConstantContainer>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    field_info: Vec<FieldInfo>,
    methods_count: u16,
    method_info: Vec<MethodInfo>,
    attributes_count: u16,
    attribute_info: Vec<AttributeContainer>
}

impl ClassStruct
{
    pub fn new<T: ReadBytes>(data: &mut T) -> ClassStruct
    {
        let mut result: ClassStruct = Default::default();
        result.magic = data.pop_u32();
        result.minor_version = data.pop_u16();
        result.major_version = data.pop_u16();
        result.constant_pool_count = data.pop_u16();
        result.constant_pool = Vec::new();
        for _i in 0..result.constant_pool_count
        {
            result.constant_pool.push(get_constant_container(data));
        }

        result.access_flags = data.pop_u16();
        result.this_class = data.pop_u16();
        result.super_class = data.pop_u16();
        result.interfaces_count = data.pop_u16();
        result.interfaces = Vec::new();
        for _i in 0..result.interfaces_count
        {
            result.interfaces.push(data.pop_u16());
        }

        result.fields_count = data.pop_u16();
        result.field_info = Vec::new();
        for _i in 0..result.fields_count
        {
            result.field_info.push(FieldInfo::new(data, &result.constant_pool));
        }

        result.methods_count = data.pop_u16();
        result.method_info = Vec::new();
        for _i in 0..result.methods_count
        {
            result.method_info.push(MethodInfo::new(data, &result.constant_pool));
        }

        result.attributes_count = data.pop_u16();
        result.attribute_info = Vec::new();
        for _i in 0..result.attributes_count
        {
            result.attribute_info.push(get_attribute_container(data, &result.constant_pool));
        }

        result
    }
}


#[cfg(test)]
mod tests
{
    use serde_json::Result;
    use crate::vecdeque;
    use std::collections::VecDeque;
    use crate::entities::class_struct::ClassStruct;
    use crate::entities::constants::constant_container::ConstantContainer;
    use crate::entities::constants::utf8_info::Utf8Info;
    use crate::entities::field_info::FieldInfo;
    use crate::entities::method_info::MethodInfo;
    use crate::entities::attributes::attribute_container::AttributeContainer;
    use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;


    #[test]
    fn class_struct_implements_equality_by_default()
    {
        let instance1: ClassStruct = Default::default();
        let instance2: ClassStruct = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn class_struct_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = get_default_vec();
        let result: ClassStruct = ClassStruct::new(&mut data);

        assert_eq!(0xCAFEBABE, result.magic);
        assert_eq!(256, result.minor_version);
        assert_eq!(1, result.major_version);
        assert_eq!(1, result.constant_pool_count);
        assert_eq!(1, result.constant_pool.len());

        let mut constant_pool_vec = vecdeque![
            1,
            0, 13,
            b'C', b'o', b'n', b's',
            b't', b'a', b'n', b't',
            b'V', b'a', b'l', b'u',
            b'e'];
        let expected_constant = ConstantContainer::Utf8Info(Utf8Info::new(&mut constant_pool_vec));
        assert_eq!(expected_constant, result.constant_pool[0]);
        assert_eq!(1, result.access_flags);
        assert_eq!(1, result.this_class);
        assert_eq!(2, result.super_class);
        assert_eq!(2, result.interfaces_count);
        assert_eq!(2, result.interfaces.len());

        let expected_interfaces = vec![2, 4];
        assert_eq!(expected_interfaces, result.interfaces);

        assert_eq!(1, result.fields_count);
        assert_eq!(1, result.field_info.len());

        let mut expected_field_content = vecdeque![
            0, 1, 0, 0, 0, 0, 0, 1,
            0, 0, 2, 2, 0, 0, 1, 1
        ];
        let expected_field = FieldInfo::new(&mut expected_field_content, &result.constant_pool);
        assert_eq!(expected_field, result.field_info[0]);

        assert_eq!(1, result.methods_count);
        assert_eq!(1, result.method_info.len());

        let mut expected_method_content = vecdeque![
            0, 1, 0, 0, 0, 0, 0, 1,
            0, 0, 2, 2, 0, 0, 1, 1
        ];
        let expected_method = MethodInfo::new(&mut expected_method_content, &result.constant_pool);
        assert_eq!(expected_method, result.method_info[0]);

        assert_eq!(1, result.attributes_count);
        assert_eq!(1, result.attribute_info.len());

        let mut expected_attribute_content = vecdeque![
            0, 0, 2, 2, 0, 0, 1, 1
        ];
        let expected_attribute = AttributeContainer::ConstantAttribute(ConstantValueAttribute::new(&mut expected_attribute_content));
        assert_eq!(expected_attribute, result.attribute_info[0]);
    }

    #[test]
    fn class_struct_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = get_default_vec();
        let mut data2: VecDeque<u8> = data.clone();

        let result: ClassStruct = ClassStruct::new(&mut data);
        let result2: ClassStruct = ClassStruct::new(&mut data2);
        assert_eq!(result, result2);
    }

    #[test]
    fn class_struct_implements_equality_correctly_when_not_equal()
    {
        let mut data: VecDeque<u8> = get_default_vec();
        let mut data2: VecDeque<u8> = data.clone();
        data2[0] = data[0] + 1;

        let result: ClassStruct = ClassStruct::new(&mut data);
        let result2: ClassStruct = ClassStruct::new(&mut data2);
        assert_ne!(result, result2);
    }

    #[test]
    fn class_struct_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = get_default_vec();

        let instance1: ClassStruct = ClassStruct::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: ClassStruct = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }

    fn get_default_vec() -> VecDeque<u8>
    {
        vecdeque![
            202, 254, 186, 190, // magic: u32: 3405691582
            1, 0,       // minor_version: u16: 256
            0, 1,       // major_version: u16: 1
            0, 1,       // constant_pool_count: u16: 1
            1,          //      Utf8Info::tag: 1
            0, 13,      //      Utf8Info::length: 13
            b'C', b'o', b'n', b's',
            b't', b'a', b'n', b't',
            b'V', b'a', b'l', b'u',
            b'e',  //      Utf8Info::value: "ConstantValue"
                        // constant_pool: Vec<ConstantContainer>,
            0, 1,       // access_flags: u16: 1
            0, 1,       // this_class: u16: 1
            0, 2,       // super_class: u16: 2
            0, 2,       // interfaces_count: u16: 4
            0, 2, 0, 4, // interfaces: Vec<u16>: [1,2,3,4]
            0, 1,       // fields_count: u16: 1
            0, 1,       //      FieldInfo::access_flags: 1
            0, 0,       //      FieldInfo::name_index: 0
            0, 0,       //      FieldInfo::descriptor_index: 0
            0, 1,       //      FieldInfo::attributes_count: 0
            0, 0,       //          ConstantValueAttribute::attribute_name_index: 0
            2, 2, 0, 0, //          ConstantValueAttribute::attribute_length: 8590065664
            1, 1,       //          ConstantValueAttribute::constant_value_index: 257
                        // field_info: Vec<FieldInfo>,
            0, 1,       // methods_count: u16: 1
            0, 1,       //      MethodInfo::access_flags: 1
            0, 0,       //      MethodInfo::name_index: 0
            0, 0,       //      MethodInfo::descriptor_index: 0
            0, 1,       //      MethodInfo::attributes_count: 1
            0, 0,       //          ConstantValueAttribute::attribute_name_index: 0
            2, 2, 0, 0, //          ConstantValueAttribute::attribute_length: 8590065664
            1, 1,       //          ConstantValueAttribute::constant_value_index: 257
                        // method_info: Vec<MethodInfo>,
            0, 1,       // attributes_count: u16: 1
            0, 0,       //      ConstantValueAttribute::attribute_name_index: 0
            2, 2, 0, 0, //      ConstantValueAttribute::attribute_length: 8590065664
            1, 1,       //      ConstantValueAttribute::constant_value_index: 257
                        // attribute_info: Vec<AttributeContainer>
        ]
    }
}