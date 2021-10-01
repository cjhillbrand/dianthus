mod class_struct;
mod field_info;
mod attributes;
mod util;
mod constants;
mod method_info;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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