use runtime_lib::class_loaders::class_loader::{ClassLoader, get_java_home};
use runtime_lib::class_loaders::system_class_loader::SystemClassLoader;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::class_loaders::bootstrap_class_loader::BootStrapClassLoader;
use std::path::PathBuf;

#[test]
#[ignore]
fn system_class_loader_loads_class_in_current_directory() {
	let file: &str = "../resources/ExamplePrintln";
	let loader: SystemClassLoader = SystemClassLoader {};

	let result: ClassStruct = loader.load_class(file);

	println!("{:#?}", result)
}

#[test]
#[ignore]
fn bootstrap_class_loader_loads_rt_jar()
{
	let loader: BootStrapClassLoader = BootStrapClassLoader {};
	let class: ClassStruct = match loader.load_rt_jar("java/io/PrintStream.class")
	{
		Some(c) => c,
		None => panic!("Could not load class")
	};

	println!("{:#?}", class);
}

#[test]
fn bootstrap_class_loader_java_home()
{
	let loader: BootStrapClassLoader = BootStrapClassLoader{ };
	let path: PathBuf = loader.path_buf();
	println!("{:?}", path);
}
