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
        for _i in 0..result.attribute_length.clone()
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

// #[cfg(test)]
// mod tests
// {
//     use serde_json::Result;
//     use crate::attributes::code_attribute::ExceptionInfo;
//
//     #[test]
//     fn exception_info_implements_equality_by_default()
//     {
//         let instance1: ExceptionInfo = Default::default();
//         let instance2: ExceptionInfo = Default::default();
//
//         assert_eq!(instance1, instance2);
//     }
//
//     #[test]
//     fn exception_info_constructs_expected_struct()
//     {
//         let data: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
//         let result: ExceptionInfo = ExceptionInfo::new(&data);
//
//         let bit16: u16 = 257;
//         assert_eq!(bit16, result.start_pc);
//         assert_eq!(bit16, result.end_pc);
//         assert_eq!(bit16, result.handler_pc);
//         assert_eq!(bit16, result.catch_type);
//     }
//
//     #[test]
//     fn exception_info_implements_equality_correctly()
//     {
//         let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
//         let instance1: ExceptionInfo = ExceptionInfo::new(&data);
//         let instance2: ExceptionInfo = ExceptionInfo::new(&data);
//
//         assert_eq!(instance1, instance2);
//     }
//
//     #[test]
//     fn exception_info_implements_equality_correctly_when_not_equal()
//     {
//         let data1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
//         let data2: Vec<u8> = vec![8, 7, 6, 5, 4, 3, 2, 1];
//         let instance1: ExceptionInfo = ExceptionInfo::new(&data1);
//         let instance2: ExceptionInfo = ExceptionInfo::new(&data2);
//
//         assert_ne!(instance1, instance2);
//     }
//
//     #[test]
//     fn exception_info_implements_json_serialization_correctly() -> Result<()>
//     {
//         let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
//         let instance1: ExceptionInfo = ExceptionInfo::new(&data);
//         let instance2 = instance1.clone();
//
//         let json = serde_json::to_string_pretty(&instance1)?;
//         let instance3: ExceptionInfo = serde_json::from_str(&json)?;
//
//         assert_eq!(instance2, instance3);
//         Ok(())
//     }
// }
