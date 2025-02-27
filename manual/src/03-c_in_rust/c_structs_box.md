# Make a Box

Rust gives you another, safer option (that again, only works with normal allocators). `Box` has a function to consume a "naked" pointer, and safely wrap it in a Rust box - so you get automatic deallocation when you are done with it.

```rust
#[test]
fn test_factory_box() {
    let object = unsafe { factory() };
    // No null pointers for us!
    assert_ne!(object, std::ptr::null_mut());

    let mut object = unsafe { Box::from_raw(object) };
    object.byte = 12;
    assert_eq!(object.byte, 12);

    // Let's check that we can still work with it
    let mut result = false;
    let retval = unsafe { is_byte_twelve(object.as_mut(), &mut result) };
    assert_eq!(0, retval);
    assert_eq!(true, result);
}
```

This still gets messy if you have nested objects - you'll need to make your own constructor
that wraps nested objects in `Box`es - but you've come a long way! You can safely consume a
pointer to an object, and dispose of it using normal RAII rules.

> Bonus! `Box::from_raw` doesn't do any copy or move operations. It takes ownership of the pointer, from the Rust view of the world. Since you are literally just reinterpreting some memory and attaching a Box to it, this is *very* fast.
