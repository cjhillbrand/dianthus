use dispatchers::array_dispatcher::ArrayDispatcher;
use dispatchers::class_dispatcher::ClassDispatcher;
use dispatchers::control_dispatcher::ControlDispatcher;
use dispatchers::double_dispatcher::DoubleDispatcher;
use dispatchers::dup_dispatcher::DupDispatcher;
use dispatchers::float_dispatcher::FloatDispatcher;
use dispatchers::int_dispatcher::IntDispatcher;
use dispatchers::invoke_dispatcher::InvokeDispatcher;
use dispatchers::jump_dispatcher::JumpDispatcher;
use dispatchers::long_dispatcher::LongDispatcher;
use dispatchers::monitor_dispatcher::MonitorDispatcher;
use dispatchers::short_dispatcher::ShortDispatcher;
use dispatchers::dispatcher::Dispatcher;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;

pub enum DispatcherContainer
{
    ARRAY(Box<ArrayDispatcher>),
    CLASS(Box<ClassDispatcher>),
    CONTROL(Box<ControlDispatcher>),
    DOUBLE(Box<DoubleDispatcher>),
    DUP(Box<DupDispatcher>),
    FLOAT(Box<FloatDispatcher>),
    INT(Box<IntDispatcher>),
    INVOKE(Box<InvokeDispatcher>),
    JUMP(Box<JumpDispatcher>),
    LONG(Box<LongDispatcher>),
    MONITOR(Box<MonitorDispatcher>),
    SHORT(Box<ShortDispatcher>),
    NONE
}

impl Dispatcher for DispatcherContainer
{
    fn dispatch(&self, thread_id: usize, runtime_data: &RunTimeData, code: &CodeAttribute) {
        match self {
            DispatcherContainer::ARRAY(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::CLASS(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::CONTROL(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::DOUBLE(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::DUP(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::FLOAT(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::INT(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::INVOKE(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::JUMP(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::LONG(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::MONITOR(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::SHORT(v) => { v.dispatch(thread_id, runtime_data, code) },
            DispatcherContainer::NONE => { panic!("Could not resolve op code ") }
        }
    }
}