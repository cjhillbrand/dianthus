use crate::entities::attributes::attribute_container::AttributeContainer;
use crate::entities::attributes::attribute_factory::get_attribute_container;
use crate::entities::constants::constant_container::ConstantContainer;
use crate::entities::read_bytes::ReadBytes;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeContainer>,
}

impl FieldInfo {
    pub fn new<T: ReadBytes>(data: &mut T, constant_pool: &[ConstantContainer]) -> FieldInfo {
        let mut result: FieldInfo = Default::default();
        result.access_flags = data.pop_u16();
        result.name_index = data.pop_u16();
        result.descriptor_index = data.pop_u16();
        result.attributes_count = data.pop_u16();

        result.attributes = Vec::new();
        for _i in 0..result.attributes_count {
            result
                .attributes
                .push(get_attribute_container(data, constant_pool));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::attributes::attribute_container::AttributeContainer;
    use crate::entities::attributes::constant_value_attribute::ConstantValueAttribute;
    use crate::entities::constants::constant_container::ConstantContainer;
    use crate::entities::constants::utf8_info::Utf8Info;
    use crate::entities::field_info::FieldInfo;
    use crate::vecdeque;
    use serde_json::Result;
    use std::collections::VecDeque;

    #[test]
    fn field_info_implements_equality_by_default() {
        let instance1: FieldInfo = Default::default();
        let instance2: FieldInfo = Default::default();

        assert_eq!(instance1, instance2);
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
    fn field_info_implements_equality_correctly() {
        let mut data: VecDeque<u8> = get_default_vec();
        let mut data2: VecDeque<u8> = data.clone();
        let constant_pool = get_default_cp();
        let instance1: FieldInfo = FieldInfo::new(&mut data, &constant_pool);
        let instance2: FieldInfo = FieldInfo::new(&mut data2, &constant_pool);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn field_info_implements_equality_correctly_when_not_equal() {
        let mut data: VecDeque<u8> = get_default_vec();
        let mut data2: VecDeque<u8> = data.clone();
        data2[0] = data[0] + 1;
        let constant_pool = get_default_cp();
        let instance1: FieldInfo = FieldInfo::new(&mut data, &constant_pool);
        let instance2: FieldInfo = FieldInfo::new(&mut data2, &constant_pool);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn field_info_implements_json_serialization_correctly() -> Result<()> {
        let mut data: VecDeque<u8> = get_default_vec();
        let constant_pool = get_default_cp();

        let instance1: FieldInfo = FieldInfo::new(&mut data, &constant_pool);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: FieldInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }

    fn get_default_vec() -> VecDeque<u8> {
        vecdeque![
            0, 1, // access_flags
            0, 123, // name_index
            0, 5, // descriptor_index
            0, 1, // attributes_count
            0, 0, // ConstantValueAttribute::attribute_name_index
            1, 1, 1, 1, // ConstantValueAttribute::attribute_length
            0, 2 // ConstantValueAttribute::constant_value_index
        ]
    }

    fn get_default_cp() -> Vec<ConstantContainer> {
        let attr_name: &str = "ConstantValue";
        let mut attr_contents = vecdeque![
            0, // tag
            0, 13, // length
        ];
        attr_contents.extend(attr_name.as_bytes().iter().copied());

        vec![ConstantContainer::Utf8Info(Utf8Info::new(
            &mut attr_contents,
        ))]
    }
}
