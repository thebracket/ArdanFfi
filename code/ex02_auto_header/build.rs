use std::env;
use std::path::PathBuf;

fn main() {
    // Read the header
    let bindings = bindgen::Builder::default()
        .header("src/hello.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Emit the bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("hello.rs"))
        .expect("Couldn't write bindings!");

    // Build the C code
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}