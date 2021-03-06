#![allow(dead_code)]
#![deny(clippy::all)]
#![allow(clippy::mut_range_bound)]

mod class_executor;
mod dispatchers;
mod heap;
mod implementations;
mod jvm_object;
mod jvm_value;
mod opcodes;
pub mod run_time_data;
mod stack_frame;

extern crate rand;
extern crate runtime_lib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;

use class_executor::ClassExecutor;

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);

	if args.len() < 2 {
		panic!("Expected exactly one file name to be supplied to the command line.")
	}

	let mut class_executor: ClassExecutor = ClassExecutor::new();
	class_executor.execute(&args[1]);
}
