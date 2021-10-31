use heap::Heap;
use jvm_object::JvmObject;
use run_time_data::RunTimeData;
use runtime_lib::entities::attributes::code_attribute::CodeAttribute;
use runtime_lib::entities::class_struct::ClassStruct;
use runtime_lib::entities::method_info::MethodInfo;
use stack_frame::StackFrame;

const CLASS_INIT: &str = "<clinit>";
pub fn invoke_class_init(thread_id: usize, runtime_data: &mut RunTimeData, class_name: &str) {
	let class: ClassStruct = runtime_data.load_class(class_name);
	println!("{:?}", class_name);
	runtime_data.add_class(Box::new(class));
	let class_ref: &ClassStruct = runtime_data.get_class(class_name);

	let method_info: &MethodInfo = class_ref.get_method(CLASS_INIT);
	let code: &CodeAttribute = method_info.derive_code_attribute();
	let init_frame: StackFrame = StackFrame::create_stack_frame(class_name, code);
	let static_obj: JvmObject = JvmObject::build_obj(class_ref);
	let heap: &mut Heap = runtime_data.get_heap_mut();
	heap.alloc_static(static_obj, class_name);
	runtime_data.push_stack(init_frame, thread_id);
}
