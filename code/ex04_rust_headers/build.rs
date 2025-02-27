fn main() {
    // Generate the header file
    cbindgen::generate(".")
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}