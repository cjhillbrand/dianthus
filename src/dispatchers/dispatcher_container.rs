use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;

pub struct DispatcherContainer {
	dispatchers: Vec<Box<dyn Dispatcher>>
}

impl DispatcherContainer {
	pub fn new(dispatchers: Vec<Box<dyn Dispatcher>>) -> DispatcherContainer { DispatcherContainer { dispatchers } }
}
impl Dispatcher for DispatcherContainer {
	fn dispatch(&self, thread_id: usize, runtime_data: &mut RunTimeData) -> bool {
		println!("Instrunction: {:#01x}", self.get_instruction(thread_id, runtime_data));
		for dispatcher in &self.dispatchers {
			if dispatcher.dispatch(thread_id, runtime_data) {
				runtime_data.print_stack();
				return true;
			}
		}

		panic!(
			"Could not resolve op: {:#01x}",
			self.get_instruction(thread_id, runtime_data)
		);
	}
}
