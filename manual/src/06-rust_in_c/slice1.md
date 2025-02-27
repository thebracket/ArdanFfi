# Slices

Rust has really nice handling of slices of memory.

Let's create a new project:

```bash
cargo new ex18 --lib
```

Now setup a tree like this:

```
.
├── c
│   ├── build_linux.sh
│   ├── ex18.c
├── Cargo.toml
└── src
    └── lib.rs
```

Don't forget that `Cargol.toml` needs to specify your crate type:

```toml
[lib]
crate-type = ["staticlib"] # Will create a .so on Linux, .dylib on Mac, .dll on Windows
```

Our Rust file (`lib.rs`) is designed to illustrate working with slices:

```rust
// We're using "no_mangle" to ensure function names are preserved
#[no_mangle]
// Don't forget extern "C"!
pub extern "C" fn sum_byte_slice(
    ptr: *const u8, // A raw pointer to bytes.
    length: usize, // Length, in bytes.
) -> i32 {
    // Always a good idea to do a null pointer check
    if ptr.is_null() || length == 0 {
        return 0;
    }
    // Slices ARE a pointer and a length!
    let slice = unsafe { std::slice::from_raw_parts(ptr, length) };

    // Now we can work as normal safe code
    let mut sum: i32 = 0;
    for &num in slice {
        sum += num as i32;
    }
    sum
}
```

> Notice that the ONLY unsafe part is the `from_raw_parts`. There's a few caveats with `from_raw_parts`; you need to be using the same memory alignment as the C compiler, the slice can't span multiple allocations. And of course, if the C program sends you an invalid length then you can expect all manner of trouble.