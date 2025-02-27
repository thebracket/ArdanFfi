# C Structures

We've covered most of the primitive types (we're avoid strings for now!). So how about Structs? This will naturally lead us towards the part we're all dreading --- pointers (and strings!). For now, let's have a gentle start.

First of all, some good news. Rust and C structs are really similar. These will have the same representation in memory (assuming you're on a sane platform with `i32` for `int`, and `i8` for `char`!)

<table>
<thead><th>C</th><th>Rust</th></thead>
<tr>
<td>

```c
struct MyStruct {
    int field;
    int field2;
    char field3;
};
```

</td>
<td>

```rust
#[repr(C)]
struct MyStruct {
    field: i32,
    field2: i32,
    field3: i8,
}
```

</td>
</tr>
</table>

> Note the `#[repr(C)]`. If you don't specify this, Rust reserves the right to reorder your struct however it feels. That can sometimes improve performance. Forgetting the `repr` can ruin your day by *sometimes* working.

Sticking with "pure" C (as opposed to C++) for now, you can use structs a lot like you use them in Rust --- with some differences.

```c
#include <stdio.h>
#include <stdlib.h>

struct MyStruct {
    int field;
    int field2;
    char field3;
};

void print(struct MyStruct s) {
    printf("%d, %d, %d\n", s.field, s.field2, s.field3);
}

void print_ptr(struct MyStruct *s) {
    printf("%d, %d, %d\n", s->field, s->field2, s->field3);
}

int main() {
    struct MyStruct s = {
        1, 2, 3
    };
    print(s);
    
    // "s" is not invalidated because it was copied. C doesn't move!
    printf("%d, %d, %d\n", s.field, s.field2, s.field3);
    
    // Grab a pointer to s and it works like a reference
    print_ptr(&s);
    return 0;
}
```

Notice how C doesn't have Rust's "move by default"---it copies. You can take a pointer, and use it.