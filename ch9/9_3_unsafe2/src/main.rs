use std::mem;

fn main() {
    unsafe {
        let ptr: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        if ptr.is_null() {
            panic!("메모리 할당 실패");
        }

        let val: *mut i32 = ptr; // ptr과 동일한 주소를 갖는 변수 생성
        *val = 123;

        println!("ptr={}", *ptr);

        libc::free(ptr as *mut libc::c_void);
    }
}