use serde_json::de::Read;
use crate::entities::attributes::annotation::Annotation;
use crate::entities::read_bytes::ReadBytes;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ElementValue
{
    tag: u8,
    element_value: ElementValueEnum
}

impl Default for ElementValue
{
    fn default() -> Self
    {
        ElementValue {
            tag: 0,
            element_value: ElementValueEnum::ConstValueIndex(0)
        }
    }
}

impl ElementValue {
    pub fn new<T: ReadBytes>(data: &mut T) -> ElementValue
    {
        ElementValue {
            tag: data.peek_u8(),
            element_value: {
                let tag: u8 = data.pop_u8();
                ElementValueEnum::new(data, tag)
            }
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum ElementValueEnum
{
    ConstValueIndex(u16),
    EnumConstValue(EnumConstValue),
    ClassInfoIndex(u16),
    Annotation(Annotation),
    ArrayValue(Vec<ElementValue>)
}

impl ElementValueEnum {
    pub fn new<T: ReadBytes>(data: &mut T, tag: u8) -> ElementValueEnum {
        let tag_char: char = tag as char;
        match tag_char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' =>
                {
                    ElementValueEnum::ConstValueIndex(data.pop_u16())
                },
            'e' => ElementValueEnum::EnumConstValue(EnumConstValue::new(data)),
            'c' => ElementValueEnum::ClassInfoIndex(data.pop_u16()),
            '@' => ElementValueEnum::Annotation(Annotation::new(data)),
            '[' => {
                let count: u16 = data.pop_u16();
                let mut result: Vec<ElementValue> = Vec::new();
                for _i in 0..count
                {
                    result.push(ElementValue::new(data));
                }

                ElementValueEnum::ArrayValue(result)
            },
            _ => panic!("Invalid char when constructing element value: {}", tag_char)
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Default)]
struct EnumConstValue {
    type_name_index: u16,
    const_name_index: u16
}

impl EnumConstValue
{
    pub fn new<T: ReadBytes>(data: &mut T) -> EnumConstValue
    {
        EnumConstValue {
            type_name_index: data.pop_u16(),
            const_name_index: data.pop_u16()
        }
    }
}