fn main() {
    let src = [1, 2, 3];
    let mut dest = [0; 3];

    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), src.len());
    }

    println!("dest: {:?}", dest);
}