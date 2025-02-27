extern "C" {
    fn say_hello();
}

/// Safety: C is inherently unsafe!
fn main() {
    // Let's say "hello world" from C.
    unsafe {
        say_hello();
    }
}
