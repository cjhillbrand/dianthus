#![allow(dead_code)]
#![deny(clippy::all)]

mod jvm_value;
mod run_time_data;
mod stack_frame;
mod class_executor;

extern crate runtime_lib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use class_executor::ClassExecutor;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        panic!("Expected exactly one file name to be supplied to the command line.")
    }

    let mut class_executor: ClassExecutor = ClassExecutor::new();

}
