# Pop Quiz: Why is `say_hello` wrapped in `unsafe`?

1. C is inherently unsafe.
2. Unsafe improves performance.
3. Unsafe is required for FFI.

![](../images/ScrollTime.png)

The answer is 3, although I really want it to be 1.

`unsafe` doesn't actually mean that a function is unsafe! It means that the function
falls outside what Rust can guarantee is safe. FFI functions are *inherently*
unsafe, because you are calling code outside of Rust's control.
