# What is FFI Anyway?

At the lowest level, FFI is a way to expose functions and data types in a way that other languages can understand. Just about every language out there supports some form of FFI.

You're using FFI, probably right now. C++, Java, Python, C#, etc. need it to call C functions. Every system call via `libc` that your browser makes is going through FFI!

## So Why is it Needed?

* Languages *other than C* mangle names. For example, a Rust library that exports `example_function` may well list it as `_RNvCskwGfYPst2Cb_3foo16example_functionfoo::example_function` in the file header. A C++ function is equally mangled. It's even scarier with Go and other managed languages - they handle functions themselves, and won't even export the function until you tell them to. You *can* decipher that (`rustfilt` exists for it!) - but your link process just became pretty terrifying.
* `struct Foo { i: i32, j: i16 }` looks pretty nice, and is probably `struct Foo { int i, short j };` in C. But it might not be. Rust is allowed to rearrange your functions. C++ can, too. Once again, it's even scarier with managed languages!

But don't despair: this is a well-trodden path, and one that can help you rewrite things in Rust rather than heckling on social media!

For example, here is the output from `nm -an hello_world | grep main` on a Rust binary:

```text
0000000000007a90 t _ZN11hello_world4main17h6a1de0d75d6764daE
```

Notice that the `main` function is mangled? You can't call it from another language without
knowing the mangling scheme - and Rust is allowed to change it at any time, including on
recompilation of the *same file*.

So there's not really a sane way to avoid using FFI if you want to call Rust from another language - or
vice versa.
