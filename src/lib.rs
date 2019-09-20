#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");



pub fn rust_foo(x: f32) -> i32 {
    unsafe {
        return 123 as i32;
    }
}
