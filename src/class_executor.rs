use run_time_data::RunTimeData;
use runtime_lib::class_loaders::class_loader_container::ClassLoaderContainer;
use runtime_lib::class_loaders::system_class_loader::SystemClassLoader;
use runtime_lib::class_loaders::class_loader::ClassLoader;
use runtime_lib::entities::class_struct::ClassStruct;
use main;

pub struct ClassExecutor
{
    run_time_data: RunTimeData,
    class_loader: ClassLoaderContainer
}

const MAIN: &str = "main";
const CODE: &str = "Code";
impl ClassExecutor
{
    pub fn new() -> ClassExecutor
    {
        ClassExecutor
        {
            run_time_data: RunTimeData::new(),
            class_loader: ClassLoaderContainer::System(SystemClassLoader{ })
        }
    }

    pub fn execute(&self, init_class: &str)
    {
        let class: ClassStruct = self.class_loader.load_class(init_class);
        self.run_time_data.add_class(class);
        let main_method: &MethodInfo = class.get_method(MAIN);
        let entry_point: &AttributeContainer = main_method.get_attribute(CODE);
    }
}