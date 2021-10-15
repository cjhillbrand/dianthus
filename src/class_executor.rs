use run_time_data::RunTimeData;
use runtime_lib::class_loaders::class_loader::ClassLoader;
use runtime_lib::class_loaders::class_loader_container::ClassLoaderContainer;
use runtime_lib::class_loaders::system_class_loader::SystemClassLoader;
use runtime_lib::entities::attributes::attribute_container::AttributeContainer;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use runtime_lib::entities::attributes::constants::CODE;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::method_info::MethodInfo;

const MAIN: &str = "main";

pub struct ClassExecutor {
	run_time_data: RunTimeData,
	class_loader: ClassLoaderContainer
}

impl ClassExecutor {
	pub fn new() -> ClassExecutor {
		ClassExecutor {
			run_time_data: RunTimeData::new(),
			class_loader: ClassLoaderContainer::System(SystemClassLoader {})
		}
	}

	pub fn execute(&mut self, init_class: &str) {
		let class: ClassStruct = self.class_loader.load_class(init_class);
		self.run_time_data.add_class(class);
		let class_ref: &ClassStruct = self.run_time_data.get_class(init_class);
		let main_method: &MethodInfo = class_ref.get_method(MAIN);
		let _entry_point: &CodeAttribute = ClassExecutor::derive_code_attribute(main_method);
	}

	fn derive_code_attribute(method: &MethodInfo) -> &CodeAttribute {
		let code_attrs: Vec<&AttributeContainer> = method.get_attributes(CODE);
		assert!(
			!(code_attrs.len() > 1),
			"There are more than 1 code attributes in the method: {}",
			method.get_name()
		);

		assert!(
			!code_attrs.is_empty(),
			"there are no code attributes in the method: {}",
			method.get_name()
		);

		match &code_attrs[0] {
			AttributeContainer::CodeAttribute(v) => v,
			_ => panic!("Attribute returned is not a code attribute.")
		}
	}
}
