use crate::attributes::attribute_info::AttributeInfo;
use crate::util::{to_vec, to_u32, to_u16 };
use std::mem::size_of;
use std::any::Any;
use crate::constants::constant_info::ConstantInfo;
use crate::attributes::attribute_factory::get_attribute;

#[derive(Default)]
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
    attribute_info: Vec<Box<dyn AttributeInfo>>
}

impl AttributeInfo for CodeAttribute
{
    fn name_index(&self) -> &u16 { &self.attribute_name_index }
    fn attr_length(&self) -> &u32 { &self.attribute_length }
    fn as_any(&self) -> &dyn Any { self }
}

impl CodeAttribute
{
    pub fn new(data: &[u8], constant_pool: &[Box<dyn ConstantInfo>]) -> CodeAttribute
    {
        let mut iter = data.iter();
        let mut result: CodeAttribute = Default::default();

        result.attribute_name_index = to_u16(&mut iter).unwrap();
        result.attribute_length = to_u32(&mut iter).unwrap();
        result.max_stack = to_u16(&mut iter).unwrap();
        result.code_length = to_u32(&mut iter).unwrap();
        result.code = to_vec(&mut iter, result.code_length.clone() as usize).unwrap();
        result.exception_table_length = to_u16(&mut iter).unwrap();

        let mut exception_table: Vec<ExceptionInfo> = Vec::new();
        for _j in 0..result.exception_table_length
        {
            let exception_info: ExceptionInfo = ExceptionInfo::new(&data);
            exception_table.push(exception_info);
        }

        result.attribute_count = to_u16(&mut iter).unwrap();
        let mut attributes: Vec<Box<dyn AttributeInfo>> = Vec::new();
        for _i in 0..result.attribute_length.clone()
        {
            attributes.push(get_attribute(&data, &constant_pool));
        }

        result.attribute_info = attributes;
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
    pub fn new(data: &[u8]) -> ExceptionInfo
    {
        let mut iter = data.iter();
        ExceptionInfo
        {
            start_pc: to_u16(&mut iter).unwrap(),
            end_pc: to_u16(&mut iter).unwrap(),
            handler_pc: to_u16(&mut iter).unwrap(),
            catch_type: to_u16(&mut iter).unwrap()
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
