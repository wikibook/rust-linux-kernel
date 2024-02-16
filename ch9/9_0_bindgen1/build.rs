extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=hello");
    println!("cargo:rerun-if-changed=c_src/hello.h");
    
    let bindings = bindgen::Builder::default()
        .header("c_src/hello.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}