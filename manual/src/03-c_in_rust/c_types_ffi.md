# Matching C Types with FFI Types

When you are dealing with C libraries and may need to support multiple platforms, Rust includes a bunch of "c types" to assist you.

Let's rewrite our Rust implementation of `double_byte` to use one of Rust's FFI types. These are defined for each platform to match the equivalent C code - they are type aliases.

```rust
mod rs {
    use std::ffi::c_char;

    pub fn double_byte(n: c_char) -> c_char { n.wrapping_mul(2) }
}
```

There's a few implications here to ponder:

* You no longer know exactly how big your `char` is on your target platform. So you can no longer make assumptions beyond the lowest common denominator: the C standard does now require that a `char` have at least 8 bits.
* Could you safely assume that it's an `i8`, and write accordingly? In a lot of cases, the answer is "yes". It's up to you.

Let's go and see what `bindgen` created:

```rust
pub fn double_byte(n: ::std::os::raw::c_char) -> ::std::os::raw::c_char;
```

Oh dear. We have `os::raw` *and* `std:ffi`???

## Some of Rust's Murkier Corners

When you dive deeply into the Rust standard library, a few things become less than clear. Here's the definition of `std::os::raw:c_char`:

```rust
macro_rules! alias_core_ffi {
    ($($t:ident)*) => {$(
        #[stable(feature = "raw_os", since = "1.1.0")]
        #[doc = include_str!(concat!("../../../../core/src/ffi/", stringify!($t), ".md"))]
        #[doc(cfg(all()))]
        pub type $t = core::ffi::$t;
    )*}
}

alias_core_ffi! {
    c_char c_schar c_uchar
    c_short c_ushort
    c_int c_uint
    c_long c_ulong
    c_longlong c_ulonglong
    c_float
    c_double
    c_void
}
```

That's a very long way of definiing type aliases into the FFI types we used. So that's ok then! We used the same type. How does FFI define it?

```rust
type_alias! { "c_char.md", c_char = c_char_definition::c_char; #[doc(cfg(all()))] }
```

Oh boy, *another* macro! And now it points into a thing called `c_char_definition`? Following the definition, you see some of the magic that makes Rust work. In `.rustup/toolchains/stable-aarch-apple-darwin/lib/rustlib/src/rust/library/core/src/ffi/mod.rs` (whew!), there's a platform specific definition that maps all of the platform C types to the `FFI` types, which are in turn aliases into `os` types.

> Isn't it nice to not have to worry about this, most of the time? Since the C ABI is the lingua-franca between languages, every single Rust implementation has to provide these.
