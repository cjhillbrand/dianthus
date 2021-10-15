/// The src.entities.attributes defined by this specification
/// as appearing in the src.entities.attributes table of a field_info/method_info
///
/// Implemented: 2,3,5,8,9,
/// Annotations: 15,16,17,18,19,20
pub trait AttributeInfo {
	fn name(&self) -> &str;
	fn attr_length(&self) -> &u32;
}

/*
TODO: Support Annotations.
pub struct RunTimeVisibleAnnotationsAttribute
{
}

pub struct RunTimeInvisibleAnnotationsAttribute
{
}

pub struct RuntimeVisibleParameterAnnotationsAttribute
{
}

pub struct RuntimeInvisibleParameterAnnotationsAttribute
{
}

pub struct AnnotationsDefaultAttribute
{
}
*/
