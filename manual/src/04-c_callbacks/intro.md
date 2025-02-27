# Callbacks and Functions

A lot of C code passes function pointers around. A lot of Rust code does, too - although its often behind nice syntax sugar with closures. It's a *really* powerful pattern.

## C Calling Back into Rust

Let's add a C function that calls another function.

```c
// Header
int call_me(int (*maybe)(int));

// Body
int call_me(int (*maybe)(int)) {
    int sum = 0;
    for (int i = 0; i < 10; i++) {
        sum += maybe(i);
    }
    return sum;
}
```

`bindgen` generates the appropriate Rust bindings:

```rust
extern "C" {
    pub fn call_me(
        maybe: ::std::option::Option<
            unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
```

> Aren't you glad you didn't type that yourself?

You can setup a Rust test to run this:

```rust
#[test]
fn test_call_me_maybe() {
    #[no_mangle]
    unsafe extern "C" fn maybe(n: i32) -> i32 {
        1
    }

    let n = unsafe { call_me(Some(maybe)) };
    assert_eq!(10, n); // Runs 10 times
}
```

Bindgen has helpfully replaced the possibility of `null` with an `Option`, and you're using the function export syntax from earlier.

This is *remarkably* useful when you are linking library services. One side can (possibly asynchronously) iterate through a series of results and pass them across the FFI boundary individually.

> I used this in a client's C# client for LanceDb. The Rust library lazily initializes a Tokio instance, and passes commands from C calls into the executor via a channel. Callbacks for results are passed along, allowing asynchronous functions to run - and stream data to the client. A "oneshot" calls back to the original function to indicate completion and provide a result code.