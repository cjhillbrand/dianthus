use heap::Heap;
use jvm_object::JvmObject;
use jvm_value::JvmValue;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::method_info::MethodInfo;
use stack_frame::StackFrame;

const INIT: &str = "init";
pub fn invoke_init(thread_id: usize, runtime_data: &mut RunTimeData, class_name: &str) -> bool
{
    if !runtime_data.is_class_loaded(class_name) {
        let class: ClassStruct = runtime_data.load_class(class_name);
        runtime_data.add_class(Box::new(class));
        let class_ref: &ClassStruct = runtime_data.get_class(class_name);
        let static_obj: JvmObject = JvmObject::build_obj(class_ref);
        let heap: &mut Heap = runtime_data.get_heap_mut();
        heap.alloc_static(static_obj, class_name);

        let method_info: &MethodInfo = class_ref.get_method(INIT);
        let code: &CodeAttribute = method_info.derive_code_attribute();
        let mut init_frame: StackFrame = StackFrame::create_stack_frame(class_name, code);
        let current_stack: &VecDeque<StackFrame> = runtime_data.get_stack_mut(thread_id);

        init_frame.set_local_var(JvmValue::Reference(class_name.to_string()), 0);
        current_stack.push_front(init_frame);

        true
    }

    false
}