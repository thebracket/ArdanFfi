# Our Porting Environment

> I recommend grabbing this from the repo (`ex05_porting`) rather than retyping the whole thing. We'll use that as a starting point.

Let's take a quick look around what we have:

* `src`
    * `clib.h` - the C library we're going to be working with.
    * `clib.c` - the C header for the library.
    * `lib.rs` - a Rust library you are building, to port your amazing C library.
* `build.rs` - the same Bindgen and CC setup you used before.
* `Cargo.toml` - has `bindgen` and `cc` as dev dependencies.

If you look at `clib.h`, we're starting with a really simple C function:

```c
char double_byte(char n);
```

In `lib.rs`, we've imported the source and setup a simple unit test:

```rust
mod c_lib {
    include!(concat!(env!("OUT_DIR"), "/clib.rs"));
}

#[cfg(test)]
mod tests {
    use super::c_lib::*;

    #[test]
    fn test_double() {
        unsafe {
            assert_eq!(double_byte(2), 4);
        }
    }
}
```

You can even run it, the unit tests pass. Hooray! C and Rut agree, 2 times 2 equals 4.