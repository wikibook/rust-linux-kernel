fn main() {
    let message = "Hello, world!\0".as_ptr() as *const libc::c_char;
    unsafe {
        libc::printf(message);
    }
}