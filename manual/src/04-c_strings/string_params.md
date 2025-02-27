# Sending Strings to C Functions

Let's go back to our project. Let's add another C function.

```c
// Header
int string_length(const char* s);

// Body
#include <string.h>

int string_length(const char* s) {
    return strlen(s);
}
```

This function will explode nicely if we don't null-terminate our string - and yield a length otherwise. Let's handle some ways to make a string to pass to it:

For constants, my favourite is the relatively new `c literal` syntax.

```rust
#[test]
fn test_string_length() {
    // Use a Rust "C string literal" to create a null-terminated string
    let s = c"Hello!";
    let len = unsafe { string_length(s.as_ptr()) };
    assert_eq!(6, len);
}
```

Prior to this code, you'd need:

```rust
let s = "Hello!";
let s_c = std::ffi::CString::new(s).unwrap();
let len = unsafe { string_length(s_c.as_ptr()) };
assert_eq!(6, len);
```

Go ahead and test that, too. You can use that with any Rust type that breaks down to `&str`, so now you can make C strings dynamically.

> The `unwrap` is there because `CString` checks that you haven't included any null/0 bytes in the string. That would be bad on the C side. It also guarantees that it will add the zero to the end. So you're adding safety!
