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
