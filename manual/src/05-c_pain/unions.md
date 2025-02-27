# Unions

Unions are pretty much the definition of what Rust doesn't allow in safe code: an
area of memory that can be interpreted to mean multiple things. A union is exactly
the size of its largest variant. They are also really useful!

So let's add a C union to our C header file:

```c
union MyUnion {
    int integer;
    char byte;
};
```

What does `bindgen` come up with?

```rust
#[repr(C)]
#[derive(Copy, Clone)]
pub union MyUnion {
    pub integer: ::std::os::raw::c_int,
    pub byte: ::std::os::raw::c_char,
}
```

Rust actually supports unions! It's just `unsafe` to do very much with them.

You can write to a Rust union safely, just ONLY set the one field and your code is both safe and sound:

```rust
#[test]
fn testing_unions() {
    let u = MyUnion { integer: 12 };
    let u = MyUnion { byte: 1 };
}
```

Accessing a union field is always unsafe, because Unions violate the aliasing rule. It works, though:

```rust
#[test]
fn testing_unions() {
    // This kinda makes sense
    let u = MyUnion { integer: 12 };
    assert_eq!(unsafe { u.integer }, 12);
    assert_eq!(unsafe { u.byte }, 12); // Technically undefined behavior, but it works
}
```

And this kinda works:

```rust
// This is getting weird
let u = MyUnion { integer: 512 };
assert_eq!(unsafe { u.integer }, 512);
assert_eq!(unsafe { u.byte }, 0); // You're just reading the first byte!
```

And please don't do this:

```rust
// This is just wrong
let u = MyUnion { byte: 1 };
assert_eq!(unsafe { u.integer }, 1); // You're reading the first 4 bytes of a single byte!
```

## Unions are REALLY useful

Even though they are unsafe (even C++ tried to restrict them, but the userbase said **NO**), unions can be super useful. Here's the Linux defintion of an IPv6 address:

```c
struct in6_addr
{
        union 
        {
                __u8    u6_addr8[16];
                __be16  u6_addr16[8];
                __be32  u6_addr32[4];
        } in6_u;
};
```

It's 128-bits of data, but you can access it as bytes, 16-bit or 32-bit numbers. Likewise, it's common to use a union of 4 bytes or a 32-bit integer (big endian) for an IP address; access all the octets individually, or as a single number.
