// This line is needed to allow pipeline to build
#![feature(arbitrary_enum_discriminant)]
#![allow(dead_code)]
// Enum variant name is just a naming convention that I like.
// field reassign I cant figure out how to pass the constant pool around in the intializer
#![deny(clippy::all)]
#![allow(
	clippy::enum_variant_names,
	clippy::field_reassign_with_default,
	clippy::too_many_arguments,
	clippy::module_inception
)]
pub mod class_loaders;
pub mod entities;
pub mod native_methods;

extern crate serde;
extern crate serde_json;
extern crate zip;
#[macro_use]
extern crate serde_derive;

#[macro_export]
macro_rules! vecdeque {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vecdeque!(@single $rest)),*]));

    ($($value:expr,)+) => { vecdeque!($($value),+) };
    ($($value:expr),*) => {
        {
            let _cap = vecdeque!(@count $($value),*);
            let mut _map = ::std::collections::VecDeque::with_capacity(_cap);
            $(
                _map.push_back($value);
            )*
            _map
        }
    };
    ($value:expr;$count:expr) => {
        {
            let c = $count;
            let mut _map = ::std::collections::VecDeque::with_capacity(c);
            for _ in 0..c {
                _map.push_back($value);
            }
            _map
        }
    };
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
