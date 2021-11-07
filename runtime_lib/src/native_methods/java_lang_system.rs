pub fn get_java_lang_system_native_method(method: &str) -> fn() {
	match method {
		"registerNatives" => java_lang_system_register_natives,
		_ => panic!("Method {} not registered for Java/Lang/System", method)
	}
}

pub fn java_lang_system_register_natives() {}
