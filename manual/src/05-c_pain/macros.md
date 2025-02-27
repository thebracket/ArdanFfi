# Macros

C macros are nothing like Rust macros. You can `#define true false` if you want to ruin someone's day (and make it conditional!). Rust macros are nice and hygienic.

> A *remarkable* number of production C++ projects include `#define private public` in their unit test code to make it easier to test the innards of classes!

The problem is - the C world *runs* on macros. Take a look at the Linux source code sometime, the macros there will make you weep.

## Macros as Constants

C has perfectly good `const` support, and even `static const` if you want it. A **lot** of C code doesn't do that, preferring instead to `#define` constants. Even better, macros in C don't have types!

So let's add a couple of `#define` style constants to our header:

```c
#define MY_CLIB_VERSION 1
#define MY_CLIB_VERSION_STRING "1.0"
```

We're on solid ground for the integer. Bindgen creates an `i32`:

```rust
fn test_constants() {
    assert_eq!(MY_CLIB_VERSION, 1);
}
```

How about the string? Bindgen has, unfortunately, exported it as:

```rust
pub const MY_CLIB_VERSION_STRING: &[u8; 4] = b"1.0\0";
```

You *can* test it by explicitly turning it into a CStr (it's static):

```rust
#[test]
fn test_constants() {
    assert_eq!(MY_CLIB_VERSION, 1);
    let c_str = unsafe { CStr::from_bytes_with_nul(MY_CLIB_VERSION_STRING)}.unwrap();
    let s = c_str.to_str().unwrap();
    assert_eq!(s, "1.0");
}
```

So it's there - it's just not really friendly. Fortunately, this is good enough to handle the most common usages such as this:

```c
#define BUFFER_SIZE 256
void process_data(char buffer[BUFFER_SIZE], size_t data_length);
```

## Macros as Types

Yes, people really do this. `typedef` is easier...

```c
#define CALLBACK int (*callback)(int)
int call_me_with_callback(CALLBACK);
```

If you search bindgen's output, you won't find a type defined for `CALLBACK`. It has rolled it into the function signature for you:

```rust
extern "C" {
    pub fn call_me_with_callback(
        callback: ::std::option::Option<
            unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
```

So even though it's a potentially evil footgun, it works just fine.

## Macros as Code

You will sometimes run into times where programmers made use of the proprocessor a little too much.

```c
#define SQUARE(x) ((x) * (x))
```

And now the **bad news**. Bindgen *won't* translate this for you. It's nowhere in the generated output. You have to port these on your own.

> One of the hurdles the Rust for Linux people have faced is that Linux does this more than you might expect. Bindgen couldn't handle the macro magic, and chunks have to be ported by hand.
