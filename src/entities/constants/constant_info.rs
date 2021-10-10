pub trait ConstantInfo
{
    fn tag(&self) -> &u8;
}


pub const UTF8: u8 = 1;
pub const INTEGER: u8 = 3;
pub const FLOAT: u8 = 4;
pub const LONG: u8 = 5;
pub const DOUBLE: u8 = 6;
pub const CLASS: u8 = 7;
pub const STRING: u8 = 8;
pub const FIELD_REF: u8 = 9;
pub const METHOD_REF: u8 = 10;
pub const INTERFACE_METHOD_REF: u8 = 11;
pub const NAME_AND_TYPE: u8 = 12;
pub const METHOD_HANDLE: u8 = 15;
pub const METHOD_TYPE: u8 = 16;
pub const INVOKE_DYNAMIC: u8 = 18;