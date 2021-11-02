use crate::native_methods::java_lang_system::get_java_lang_system_native_method;

pub fn get_method(class_name: &str, method_name: &str) -> fn()
{
    match class_name {
        "java/lang/System" => get_java_lang_system_native_method(method_name),
        _ => panic!("can not link to native methods from class: {}", class_name)
    }
}