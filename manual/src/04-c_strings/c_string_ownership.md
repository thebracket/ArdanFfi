# Ownership 1

So what if the C function you are calling takes ownership of the string you pass - by deleting it! 

C doesn't have a formal concept of ownership, and often the way to discover that you need to jump through this hoop is to read the C code. It's caused nightmares for large C projects!

* C will happily let you allocate a buffer on your local stack, pass a pointer to it as a string and then die horribly when that function frees it.
* C won't tell you that you allocated a string and never freed it.
* And so on. Sit an old C programmer down with some free beer if you want to hear more. It'll be a long night.

Let's add another function to the C:

```c
// Header
int string_length_and_delete(char* s);

// Body
int string_length_and_delete(char* s) {
    int len = strlen(s);
    free(s);
    return len;
}
```

Fortunately, `CString` has an `into_raw` function just for this case:

```rust
#[test]
fn test_string_length_and_delete() {
    // `into_raw` consumes the CString and returns a raw pointer
    let s = std::ffi::CString::new("Hello!").unwrap();
    let len = unsafe { string_length_and_delete(s.into_raw()) };
    assert_eq!(6, len);

    // Uncomment to show that this preserves Rust safety
    // println!("{:?}", s); // This will panic, as the CString has been deleted
}
```

If you try and use `s` after `into_raw` - it won't compile. Safety is preserved.

## Taking Ownership of a String

Let's go the other way. A C function allocates a string on the heap, and sends you a pointer to it. The C programmer helpfully noted that you need to delete it when you are done (we may be in a parallel universe). Let's add another C function:

```c
// Header
char * return_hello();

// Body
char * return_hello() {
    char * s = (char *)malloc(6);
    strcpy(s, "Hello");
    return s;
}
```

And let's test it on the Rust side:

```rust
#[test]
fn test_return_hello() {
    let s = unsafe { return_hello() };

    // Convert the C string to a Rust string.
    // from_raw consumes the CString and returns the original String
    let s = unsafe { std::ffi::CString::from_raw(s) };
    let s = s.to_str().unwrap();
    assert_eq!("Hello", s);
}
```

So what's up with the `to_str`, and it being fallible? If the string from C isn't null-terminated, it will fail to convert to a CString. Likewise, if the string isn't `UTF-8` compatible. ASCII is - but who knows what will appear in the buffer?

> When I started helping with the LibreQoS project, a friend of mine - a Linux kernel contributor - expressed concern that my Rust code was passing strings around like candy. I showed him how hard Rust make it to leak a string, or perform the other nightmare scenarios - and to his credit, he tried to learn Rust!
