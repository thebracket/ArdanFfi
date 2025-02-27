# Simple Struct Pointers

> Yes, I just used "C", "Pointers" and "Simple" in the same sentence. I'll doubtless regret that.

Let's add another item to our header and C body:

```c
int reference_struct(struct MyStruct *s);
```

```c
int reference_struct(struct MyStruct *s) {
    return s->integer;
}
```

This is the easiest case - we're pointing to a structure that already exists. The Rust test is pretty straightforward:

```rust
#[test]
fn test_reference_struct() {
    let mut s = MyStruct { integer: 9, byte: 7 };
    let n = unsafe { reference_struct(&mut s) };
    assert_eq!(n, 9);
}
```

And here we have a whole bunch of things to spot!

* `s` has to be mutable! We didn't specify `const` in C-land, so the function is free to do whatever it feels like to your data.
* We have to borrow with `&mut` for the same reason.

You *can* fix that if you can adjust the C. It's *remarkable* how many C programmers fight doing this, or just forget.

```c
int reference_struct_const(const struct MyStruct *s);

int reference_struct_const(const struct MyStruct *s) {
    return s->integer;
}
```

And now, a Rust test:

```rust
#[test]
fn test_reference_struct_const() {
    let s = MyStruct { integer: 9, byte: 7 };
    let n = unsafe { reference_struct_const(&s) };
    assert_eq!(n, 9);
}
```

> You see? That wasn't so bad! Just wait...

## Out Variables

Since plain C doesn't have tuples or another easy way to return multiple things at once, a LOT of C uses patterns like thsi one:

```c
#include <stdbool.h> // Yes, you really do need a header to use a bool!
int is_byte_twelve(const struct MyStruct* s, bool* val);
```

```c
int is_byte_twelve(const struct MyStruct* s, bool* val) {
    int errnum;
    if (!s) {
        errnum = -1;
        goto error; // Goto is really common in C for error handling.
    }
    *val = s->byte == 12 ? true : false;
    return 0;

error:
    return errnum;
}
```

The idea here is that something more complicated than checking for 12 occurs, and either true or false is returned. But the function is fallible! In Rust terms, it's `Result<bool, MyError>` - where `MyError` is in this case an int referencing some pretend documentation.

So what's the issue here?
* We have to think about the ownership of `val`. In this case, it's up to us to create it and pass a pointer. Passing NULL will ruin your day.
* It's a bit messy to follow the logic if you aren't used to it.
* It's not really a Rust paradigm, so you'll dance around a bit.

Here's a Rust test for it:

```rust
#[test]
fn test_is_byte_twelve() {
    let s = MyStruct { integer: 0, byte: 12 };
    let mut result = false; // Rust won't let you not initialize it
    let retval = unsafe { is_byte_twelve(&s, &mut result) };
    assert_eq!(0, retval);
    assert_eq!(true, result);

    let s = MyStruct { integer: 0, byte: 13 };
    let retval = unsafe { is_byte_twelve(&s, &mut result) };
    assert_eq!(0, retval);
    assert_eq!(false, result);
}
```

> OK, that was a long one. Drink some coffee, and we're going to dive into ownership.