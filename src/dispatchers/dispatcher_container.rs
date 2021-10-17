use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub struct DispatcherContainer {
    dispatchers: Vec<Box<dyn Dispatcher>>
}

impl DispatcherContainer
{
    pub fn new(dispatchers: Vec<Box<dyn Dispatcher>>) -> DispatcherContainer
    {
        DispatcherContainer {
            dispatchers
        }
    }

}
impl Dispatcher for DispatcherContainer
{
    fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
        for dispatcher in &self.dispatchers {
            dispatcher.dispatch(thread_id.clone(), runtime_data, code);
        }
    }
}