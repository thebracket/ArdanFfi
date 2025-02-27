# Good Old Free

Just in case you were missing the "good old days" of C (!), let's look at some C-like options.

> It's surprising how C-like low-level Rust can get!

Let's start by looking at the interface `bindgen` gives us:

```rust
extern "C" {
    pub fn factory() -> *mut MyStruct;
}
```

That looks a lot like the C interface. A naked, mutable pointer (unsafe if you use it!) to a `MyStruct`.

## We're dealing with C, so null pointers exist!

Rust actually *has* null pointers, just not in normal/idiomatic Rust. Step 1 is to invoke the factory, and makre sure that the pointer we received is actually valid:

```rust
#[test]
    fn test_factory_free() {
        let object = unsafe { factory() };
        // No null pointers for us!
        assert_ne!(object, std::ptr::null_mut());
    }
```

Otherwise, accessing the null pointer would do exactly what it does in C-land: crash your program with a segmentation fault. We're looking to interoperate with C, not emulate it!

## C, the Land of the Free (and the Malloc)

And let's call `free`, just as if we were using C.

```rust
#[test]
    fn test_factory_free() {
        let object = unsafe { factory() };
        // No null pointers for us!
        assert_ne!(object, std::ptr::null_mut());

        // We can call libc directly to free the memory
        // In this case, we've linked libc via the c_lib crate, sometimes
        // you have to import libc directly
        unsafe { free(object as *mut c_void) };
    }
```

This comes with a *lot* of caveats:

* If the C library is using a custom allocator, *don't do this*. The C library pretty much *has* to expose a de-allocator function for you to use, or the memory is going to be leaked.
* If the object contains other pointers, you have to go through and free them all in order. That can be really error-prone.
* As-is, you have a `*mut` pointer to the object. Unless you're only going to be interacting with the C side of the world, you want to create an idiomatic Rust solution!