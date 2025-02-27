# C Strings

We've deliberately saved this up - we needed to cover pointers (there will be more on that) first.

C strings are a pointer to a set of memory whose bytes are assumed to be ASCII characters (1 byte each). They might actually be `utf8` encoded, but that's not standard. "Best" of all, C strings don't store a length. Instead, they are assumed to keep going until the next `0` in memory.

So:

```c
const char * hello = "Hello\0";
```

Is guaranteed to be:

|Letter|Byte|
|--|--|
|H|72|
|e|101|
|l|108|
|l|108|
|o|111|
|\0|0|


## So Why Is This a Problem?

History has shown that this was a relatively terrible idea.

* Functions like `strcpy` keep reading until they find a zero. If there isn't one, you can go straight past the end of the buffer. Isn't it nice when sensitive data a few variables down a struct appears in your string?
* Since the exact size of the string isn't readily available, you have to be *really* careful when copying strings into buffers. There are thousands of CVEs (Common Vulnerability) reports resulting from this.

**Pascal** in 1970 (2 years before C!) figured this out, and strings consisted of a LENGTH and the data. Admittedly, length was a byte (no 256 character strings; Turbo Pascal fixed that).

**Rust** actually makes it a little more confusing!

Rust has several types of string. The big ones are:

* `&str` - a pointer to a string in memory that stores *both* the length and the data. No null terminator!
* `string` - a vector of `bytes` types, mapped to `char`s.
* `CowString` - a copy-on-write string.
* There's a few more for things like Path buffers and dealing with C.

On top of that, a Rust `char` is not the same as a C `char`.
* A **C** `char` is typcally one byte (on most platforms).
* A **Rust** `char` is 4 bytes, unless it isn't. All Rust characters are `UTF-8` encoded, which *can* be anywhere from 1 to 4 bytes. Rust strings store a byte array that is interpreted based on "codepoints".

Try this:

```rust
fn main() {
    let s = "I🩷🦀".to_string();
    println!("Length: {}", s.len());
    println!("Character length: {}", s.chars().count());

    // Print byte values
    println!("Bytes:");
    for (i, byte) in s.bytes().enumerate() {
        println!("Byte {}: {}", i, byte);
    }
}
```

That yields:

```
Length: 9
Character length: 3
Bytes:
Byte 0: 73
Byte 1: 240
Byte 2: 159
Byte 3: 169
Byte 4: 183
Byte 5: 240
Byte 6: 159
Byte 7: 166
Byte 8: 128
```

So...

* The string is *technically* incompatible with the C standard, but most compilers will take UTF-8 (whether it will display is another question).
* The string is *definitely* incompatible with the original ASCII (which goes to 128), but everyone really uses ANSI now.
* There isn't a null terminator in sight.

