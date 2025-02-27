# Hello Rust

Sadly, simply having a `.c` file in your project isn't enough to make it work. 
Maybe that'll be a future Cargo feature!

We need to tell Rust that the C function exists. In `src/main.rs`, add the following:

```rust
extern "C" {
    fn say_hello();
}
```

There's a few things here to remember:
* `extern` means that the function is defined elsewhere.
* `"C"` is the calling convention. This is the default for C, and is typically the only one you'll use.
* `fn say_hello();` is the function signature. For now, it's up to you to match `void say_hello()` to `fn say_hello()` (easy enough - just wait...).

Next up, we need to call the function. In `src/main.rs`, add the following:

```rust
/// Safety: C is inherently unsafe!
fn main() {
    // Let's say "hello world" from C.
    unsafe {
        say_hello();
    }
}

```

There's a bit to mention here, too:
* We have a `Safety` comment! If you are using unsafe code, Clippy really wants you to have one of these.
* We're wrapping `say_hello` in `unsafe`.

> For all the eager beavers who ran `cargo run` - it won't work yet! We haven't actually compiled the C code.