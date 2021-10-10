#![allow(dead_code)]
// Enum variant name is just a naming convention that I like.
// field reassign with defualt cant be mitigated since some structs cant be initialized all
// at once.
#![deny(clippy::all)]
#![allow(clippy::enum_variant_names, clippy::field_reassign_with_default)]
pub mod class_loaders;
pub mod entities;

extern crate serde;
extern crate serde_json;
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
