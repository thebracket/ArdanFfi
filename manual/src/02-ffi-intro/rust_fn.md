# Create A Portable Rust Function

Since we did "Hello World" from C, let's do "Hello World" from Rust:

```rust
#[no_mangle]
pub extern "C" fn say_hello() {
    println!("Hello, World from Rust.")
}
```

There's a bit of extra stuff here!

* `no_mangle` disables "name mangling". Rust (just like C++) adds all kinds of stuff to your function names in libraries if you don't turn mangling off. This helps the compiler build Rust projects, but also makes it pretty much impossible to figure out the function name when linking from C. So you have to turn off mangling.
* `extern "C"` in the function header. This sets the function's calling convention. The optimizer won't helpfully rearrange your arguments, decide to inline your function, or replace any arguments with registers. Without this, Rust reserves the right to do whatever it thinks might help. For once, we dont' want it to be helpful.

## And Build It!

Build it with `cargo build`. In your `target/debug` directory, you should see something like:

```
-rw-r--r--@   1 herbert  staff   160B Jan  8 12:16 libex03_rust_from_c.d
-rwxr-xr-x@   1 herbert  staff   397K Jan  8 12:16 libex03_rust_from_c.dylib
```

> You'll see a `.so` on Linux, a `.dylib` on Mac and a `.dll` on Windows.