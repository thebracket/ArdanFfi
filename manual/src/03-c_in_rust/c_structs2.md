# Simple Struct Usage

Let's add some C to our testbed header and code file:

```c
struct MyStruct {
    int integer;
    char byte;
};

int copy_struct(struct MyStruct s);
struct MyStruct return_struct(int n, char c);
```

```c
int copy_struct(struct MyStruct s) {
    return s.integer;
}

struct MyStruct return_struct(int n, char c) {
    struct MyStruct s = { n, c };
    return s;
}
```

Bindgen creates a struct that should look familiar. By default, it'll implement `Debug`, `Clone` and `Copy` when possible.

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyStruct {
    pub integer: ::std::os::raw::c_int,
    pub byte: ::std::os::raw::c_char,
}
```

With that in mind, let's add some tests to our library to use these:

```rust
#[test]
fn test_copy_struct() {
    let s = MyStruct { integer: 12, byte: 3 };
    let n = unsafe { copy_struct(s) };
    assert_eq!(n, s.integer);
}

#[test]
fn test_return_struct() {
    let s = unsafe { return_struct(11, 2) };
    assert_eq!(s.byte, 2);
    assert_eq!(s.integer, 11);
}
```

The tests work, and I don't havd any gotcha moments for you! This is one of the best parts: as long as you remember the representation, and are using primitives---C to Rust (and vice versa) just works.