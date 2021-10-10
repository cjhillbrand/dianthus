use jbc_parser::class_loaders::class_loader::ClassLoader;
use jbc_parser::class_loaders::system_class_loader::SystemClassLoader;
use jbc_parser::entities::class_struct::ClassStruct;

#[test]
#[ignore]
fn system_class_loader_loads_class_in_current_directory() {
    let file: &str = "resources/Example1";
    let loader: SystemClassLoader = SystemClassLoader {};

    let result: ClassStruct = loader.load_class(file);

    println!("{:#?}", result)
}
