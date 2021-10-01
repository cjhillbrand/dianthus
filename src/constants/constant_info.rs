use std::any::Any;

pub trait ConstantInfo
{
    fn tag(&self) -> &u8;
    fn as_any(&self) -> &dyn Any;
}

pub enum ConstantFlags
{
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18
}
