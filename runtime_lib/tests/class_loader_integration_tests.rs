use runtime_lib::class_loaders::class_loader::ClassLoader;
use runtime_lib::class_loaders::system_class_loader::SystemClassLoader;
use runtime_lib::entities::class_struct::ClassStruct;

#[test]
#[ignore]
fn system_class_loader_loads_class_in_current_directory() {
	let file: &str = "../resources/ExampleMethod2";
	let loader: SystemClassLoader = SystemClassLoader {};

	let result: ClassStruct = loader.load_class(file);

	println!("{:#?}", result)
}
