# Slices with Structs

You often want to pass a blob of structs into Rust.

Let's add a type to our `lib.rs` file:

```rust
#[repr(C)]
#[derive(Debug)]
pub struct MyData {
    pub a: i32,
    pub b: i16,
    pub c: i8,
}
```

> If you forget `repr(C)`, you can expect bizarre things to happen.

## Slicing Raw Bytes

> There's a lot of ways to do this. The `bytemuck` and `zerocopy` crates are popular if you are diving deeply into this.

Let's make a Rust function that takes whatever C gives it as a byte array, performs some safety checks, and treats the byte blob as a slice of `MyStruct`:

```rust
#[no_mangle]
pub extern "C" fn print_slice_of_mydata(ptr: *const u8, length: usize) {
    // Null checks are needed
    if ptr.is_null() || length == 0 {
        return;
    }
    // Check that the alignment means this is possible
    assert_eq!(ptr.align_offset(std::mem::align_of::<MyData>()), 0, "Pointer is not aligned");    
    // If the number of bytes isn't a multiple of the struct size,
    // it's probably not valid
    assert_eq!(length % std::mem::size_of::<MyData>(), 0, "Length is not a multiple of MyData size");

    // Make the slice. Note that we're CASTING the pointer, just like C. `from_raw_parts` likes
    // the number of ELEMENTS, just like C pointer math.
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const MyData, length / size_of::<MyData>()) };

    // Work with it normally
    for data in slice {
        println!("{data:?}");
    }
}
```

Now let's write some C to use it:

```c
#include <stdio.h>
#include <stdint.h> // For easy types

// The struct definition is a 1:1 match
struct MyData {
    int32_t a;
    int16_t b;
    int8_t c;
};

// You can use cbindgen, but this is an "easy" one!
void print_slice_of_mydata(struct MyData* data, size_t len);

int main() {
    // Declare some data (on the stack)
    struct MyData data[] = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };
    printf("Raw byte slice:\n");
    // Call our function
    print_slice_of_mydata(data, sizeof(data));
    printf("\n");

    return 0;
}
```

## Typed Slices

Assuming everything is a byte array is very early-C like, but more modern C likes types. So let's make a more idiomatic version. In `lib.rs`:

```rust
#[no_mangle]
pub extern "C" fn print_slice_nicely(ptr: *const MyData, num_elements: usize) {
    // Still null checking
    if ptr.is_null() || num_elements == 0 {
        return;
    }
    // Still alignment checking
    assert_eq!(ptr.align_offset(std::mem::align_of::<MyData>()), 0, "Pointer is not aligned");
    // No need for sizeof anymore
    let slice = unsafe { std::slice::from_raw_parts(ptr, num_elements) };
    // And now its just a slice
    for data in slice {
        println!("{data:?}");
    }
}
```

There's a bunch of wins here:

* It's now *really obvious* what our function expects.
* There's a little less error checking needed on the Rust side.

The C program changes a little:

```c
printf("Nicely formatted slice:\n");
print_slice_nicely(data, sizeof(data) / sizeof(data[0]));
```

That's right - the C grows! `sizeof(data)` is in *bytes*, so you have to divide it by the element size. It's six of one, half a dozen of the other: one side is going to be doing that!

> Important: make a convention. Are you passing lengths as number of elements or bytes? Number of elements is generally more intuitive, but the C creatures may disagree. Ask them. Nicely.
