# Into the Void

If you're familiar with some older-style C (or just "clever" C), you may have run into:

```c
void into_the_void(void * black_hole, int len);
```

> You may have even run into `void **` or `void ***`!

This is a very un-Rustacean way to write things, but the C-world generally doesn't care. So we have to make it work!

## What does this mean?

A `void pointer` is literally a memory address with no assertion as to what it points at. It's really common in older C, and you still find it. I ran into it a bunch when working with `libxdp` --- which is actively maintained!

There's a few reasons it might exist:

* You genuinely want to pass a number of types, often with a flag indicating what you're passing. This is basically poor-man's C++ inheritance.
* Your code-base hasn't adopted the C type system in any real detail.
* Your code-base predates the C type system.
* You have to work with a really broken C compiler for some obscure platform. It's terrifying how often that happens.

## Let's try it!

Let's add another C function:

```c
// header
int extract_from_the_void(void *s);

// body
int extract_from_the_void(void *s) {
    return ((struct MyStruct *)s)->integer;
}
```

Let's see what bindgen makes:

```rust
extern "C" {
    pub fn extract_from_the_void(s: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
```

Oh wow... Rust actually has `void` type, too. And JUST like C, you can cast it into other things:

```rust
#[test]
fn test_extract_from_the_void() {
    let my_struct = MyStruct { integer: 12, byte: 3 };
    let n = unsafe { extract_from_the_void(&my_struct as *const MyStruct as *mut c_void) };
    assert_eq!(n, 12);
}
```