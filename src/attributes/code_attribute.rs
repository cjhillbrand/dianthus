use crate::attributes::attribute_info::AttributeInfo;
use crate::read_bytes::ReadBytes;
use crate::constants::constant_info::ConstantInfo;
use crate::attributes::attribute_factory::{get_attribute_container};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::attributes::attribute_container::AttributeContainer;
use crate::constants::constant_container::ConstantContainer;

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct CodeAttribute
{
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u16,
    exception_table: Vec<ExceptionInfo>,
    attribute_count: u16,
    attribute_info: Vec<AttributeContainer>
}

impl AttributeInfo for CodeAttribute
{
    fn name_index(&self) -> &u16 { &self.attribute_name_index }
    fn attr_length(&self) -> &u32 { &self.attribute_length }
}

impl CodeAttribute
{
    pub fn new<T: ReadBytes>(mut data: &mut T, constant_pool: &[ConstantContainer]) -> CodeAttribute
    {
        let mut result: CodeAttribute = Default::default();
        result.attribute_name_index = data.pop_u16();
        result.attribute_length = data.pop_u32();
        result.max_stack = data.pop_u16();
        result.max_locals = data.pop_u16();
        result.code_length = data.pop_u32();
        result.code = data.pop_vec(result.code_length.clone() as usize);
        result.exception_table_length = data.pop_u16();

        result.exception_table = Vec::new();
        for _j in 0..result.exception_table_length
        {
            let exception_info: ExceptionInfo = ExceptionInfo::new(data);
            result.exception_table.push(exception_info);
        }

        result.attribute_count = data.pop_u16();
        result.attribute_info = Vec::new();
        for _i in 0..result.attribute_count.clone()
        {
            result.attribute_info.push(get_attribute_container(data, &constant_pool));
        }

        result
    }
}



#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ExceptionInfo
{
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

impl ExceptionInfo
{
    pub fn new<T: ReadBytes>(mut data: &mut T) -> ExceptionInfo
    {
        ExceptionInfo
        {
            start_pc: data.pop_u16(),
            end_pc: data.pop_u16(),
            handler_pc: data.pop_u16(),
            catch_type: data.pop_u16()
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::attributes::code_attribute::{ExceptionInfo, CodeAttribute};
    use serde_json::Result;
    use std::collections::VecDeque;
    use crate::vecdeque;
    use crate::attributes::attribute_container::AttributeContainer;
    use crate::attributes::constant_value_attribute::ConstantValueAttribute;
    use crate::constants::constant_container::ConstantContainer;
    use crate::constants::class_info::ClassInfo;

    #[test]
    fn exception_info_implements_equality_by_default()
    {
        let instance1: ExceptionInfo = Default::default();
        let instance2: ExceptionInfo = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn exception_info_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 1, 1, 1, 1, 1, 1, 1];
        let result: ExceptionInfo = ExceptionInfo::new(&mut data);

        let bit16: u16 = 257;
        assert_eq!(bit16, result.start_pc);
        assert_eq!(bit16, result.end_pc);
        assert_eq!(bit16, result.handler_pc);
        assert_eq!(bit16, result.catch_type);
    }

    #[test]
    fn exception_info_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = data.clone();
        let instance1: ExceptionInfo = ExceptionInfo::new(&mut data);
        let instance2: ExceptionInfo = ExceptionInfo::new(&mut data2);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn exception_info_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let mut data2: VecDeque<u8> = vecdeque![8, 7, 6, 5, 4, 3, 2, 1];
        let instance1: ExceptionInfo = ExceptionInfo::new(&mut data1);
        let instance2: ExceptionInfo = ExceptionInfo::new(&mut data2);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn exception_info_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = vecdeque![1, 2, 3, 4, 5, 6, 7, 8];
        let instance1: ExceptionInfo = ExceptionInfo::new(&mut data);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: ExceptionInfo = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }

    #[test]
    fn code_attribute_implements_equality_by_default()
    {
        let instance1: CodeAttribute = Default::default();
        let instance2: CodeAttribute = Default::default();

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn code_attribute_constructs_expected_struct()
    {
        let mut data: VecDeque<u8> = get_default_code_attribute_vec();
        let constant_pool: Vec<ConstantContainer> = get_default_constant_container();
        let result: CodeAttribute = CodeAttribute::new(&mut data, constant_pool.as_slice());

        assert_eq!(258, result.attribute_name_index);
        assert_eq!(131077, result.attribute_length);
        assert_eq!(1029, result.max_stack);
        assert_eq!(1537, result.max_locals);
        assert_eq!(4, result.code_length);

        let expected_code: Vec<u8> = vec![1, 2, 3, 4];
        assert_eq!(expected_code, result.code);

        assert_eq!(1, result.exception_table_length);

        let mut expected_exception_vec = vecdeque![1, 1, 1, 2, 2, 5, 3, 4];
        let expected_exception = ExceptionInfo::new(&mut expected_exception_vec);
        assert_eq!(1, result.exception_table.len());
        assert_eq!(expected_exception, result.exception_table[0]);

        assert_eq!(0, result.attribute_count);
        assert_eq!(0, result.attribute_info.len());
    }

    fn code_attribute_implements_equality_correctly()
    {
        let mut data: VecDeque<u8> = get_default_code_attribute_vec();
        let mut data2: VecDeque<u8> = data.clone();
        let constant_pool = get_default_constant_container();
        let instance1: CodeAttribute = CodeAttribute::new(&mut data, &constant_pool);
        let instance2: CodeAttribute = CodeAttribute::new(&mut data2, &constant_pool);

        assert_eq!(instance1, instance2);
    }

    #[test]
    fn code_attribute_implements_equality_correctly_when_not_equal()
    {
        let mut data1: VecDeque<u8> = get_default_code_attribute_vec();
        let mut data2: VecDeque<u8> = get_default_code_attribute_vec();
        data2[0] = data1[0] + 1;
        let constant_pool = get_default_constant_container();
        let instance1: CodeAttribute = CodeAttribute::new(&mut data1, &constant_pool);
        let instance2: CodeAttribute = CodeAttribute::new(&mut data2, &constant_pool);

        assert_ne!(instance1, instance2);
    }

    #[test]
    fn code_attribute_implements_json_serialization_correctly() -> Result<()>
    {
        let mut data: VecDeque<u8> = get_default_code_attribute_vec();
        let constant_pool = get_default_constant_container();

        let instance1: CodeAttribute = CodeAttribute::new(&mut data, &constant_pool);
        let instance2 = instance1.clone();

        let json = serde_json::to_string_pretty(&instance1)?;
        let instance3: CodeAttribute = serde_json::from_str(&json)?;

        assert_eq!(instance2, instance3);
        Ok(())
    }

    fn get_default_code_attribute_vec() -> VecDeque<u8>
    {
        vecdeque![
            1, 2,       // attribute_name_index = 258
            0, 2, 0, 5, // attribute_length = 131077
            4, 5,       // max_stack = 1029
            6, 1,       // max_locals = 1537
            0, 0, 0, 4, // code_length = 4
            1, 2, 3, 4, // code = 1, 2, 3, 4
            0, 1,       // exception table length = 1
                1, 1,   // ExceptionInfo::start_pc = 257
                1, 2,   // ExceptionInfo::end_pc = 258
                2, 5,   // ExceptionInfo::handler_pc = 517
                3, 4,   // ExceptionInfo::catch_type = 772
            0, 0        // attribute_count = 0
        ]
    }

    fn get_default_constant_container() -> Vec<ConstantContainer>
    {
        let const_info: ClassInfo = Default::default();
        vec![ConstantContainer::ClassInfo(const_info)]
    }
}
