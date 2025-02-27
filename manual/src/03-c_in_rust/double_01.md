# Porting Our First Function

So we have the devilishly hard C function:

```c
char double_byte(char n) {
    return n * 2;
}
```

Let's stretch our Rust abilities and write a Rust equivalent. We'll put it in a `rs` module, just to help keep the two separated.

```rust
mod rs {
    pub fn double_byte(n: i8) -> i8 { n * 2 }
}

pub use rs::*;
```

Nothing special here. We are putting our ported Rust into a module (you'd probably use separate files in a real project),
and since it's a library - we're exporting things. Mostly because squiqqly "unused" warnings annoy me.

## A Simple Test

There's almost no point in testing something this simple....

```rust
#[test]
fn test_double() {
    unsafe {
        assert_eq!(double_byte(2), 4);
    }
    assert_eq!(super::rs::double_byte(2), 4);
}
```

Run the test, and 2 times 2 equals 4.

## That Wasn't a Gotcha - Let's Test a Bit More!

Let's write a much more comprehensive test, since we're dealing with such a small range of values:

```rust
#[test]
fn range_test_double() {
    for n in c_char::MIN .. c_char::MAX {
        let c_result = unsafe { double_byte(n) };
        let rust_result = super::rs::double_byte(n);
        assert_eq!(c_result, rust_result);
    }
}
```

Run the test, and it panics. And it panics in the *Rust* side. How could doubling a byte possibly be unsafe? *Overflow*.

> There's another gotcha here! On an M1 Mini in Docker, `char` is *unsigned*. This is a great example of the perils of FFI! Let's fire up the Docker version and see what happens...

## Overflow

In C, numeric operations overflow (it can be undefined behavior on some editions of C, but the behavior is consistent enough
that everyone expects it). So for a signed 8-bit number, `64 * 2 = -128` (the first bit is used to indicate sign - so it overflows and sets the negative bit).

Now, as the Rust porter - you have to try to figure out what the C programmer was thinking.

### Did they WANT overflow?

Some algorithms use numeric overflow. Rust can do that:

```rust
pub fn double_byte(n: i8) -> i8 { n.wrapping_mul(2) }
```

Now your program won't panic, and *it is clearly documented that you intend to wrap*. That's more important than you might think. In a few decades when Rust programmers are grumbling that some new language is taking our jobs - it's now **obvious** to the maintainer what you intended. More reasonably, it's obvious to you when you come back in a few weeks.

### Did they NOT want overflow?

Sometimes, you are lucky and you will find a C programmer who remembered to add some guard code to handle unintended overflow. If you are really lucky, you'll see something like this:

```c
#include <limits.h>
#include <stdbool.h>

bool double_byte(char n, char *result) {
    // Check for overflow before multiplying
    if (n > CHAR_MAX / 2 || n < CHAR_MIN / 2) {
        return false; // Indicate overflow
    }
    *result = n * 2;
    return true; // Indicate success
}
```

> Note: there are *many* different approaches to this in common use, some of them compiler specific, and some of them don't actually work...

Usage for this is fun, because regular C doesn't support tuples or other multi-returns.

```c
#include <stdio.h>

int main() {
    char n = 64;
    char result;

    if (double_byte(n, &result)) {
        printf("Doubled value: %d\n", result);
    } else {
        printf("Overflow occurred!\n");
    }

    return 0;
}
```

A literal port would give you:

```rust
fn double_byte(n: i8, result: &mut bool) -> i8
```

You *could* port it like that, but if you can deduce the intent --- you are far better off porting to idiomatic Rust:

```rust
fn double_byte(n: i8) -> Option<i8> {
    n.checked_mul(n)
}
```

## What if you have NO IDEA what they wanted?

This happens a LOT more than you might think. The bad news is that you need to either:

* Pick one.
* Trace the original function usage, and find out if anything relies upon wrapping behavior.

It's quite common to discover that your decades old code has had a bug for years and years. It's sadly common to discover that you have been relying on that bug!