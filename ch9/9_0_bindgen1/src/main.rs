use std::ffi::CString;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let c_to_print = CString::new("Hello Rust").expect("CString::new failed");

    unsafe {
        bindings::hello(c_to_print.as_ptr());
    }
}
