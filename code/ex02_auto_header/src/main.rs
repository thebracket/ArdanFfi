// Import the auto-generated header
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

/// Safety: C is inherently unsafe!
fn main() {
    // Let's say "hello world" from C.
    unsafe {
        say_hello();
    }
}
