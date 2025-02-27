# Pointers and Ownership

> We lost the word "simple". Sorry everyone, buckle up!

Pure C programs often allocate a bunch of structs, and pass pointers around like crazy. This can be especially challenging while you replace *parts* of a C program in Rust!

Let's start with an all-too-common one. The C program makes a pointer to some data, and it's up to you to dispose of it.

```c
// Header
#include <stdlib.h>

struct MyStruct * factory();

// Body
struct MyStruct * factory() {
    struct MyStruct * s = (struct MyStruct *)malloc(sizeof(struct MyStruct));
    return s;
}
```

> Ignore the hundreds of warnings that just appeared from `bindgen` including links to the C standard library.

So we have a C function handling us a pointer into the heap, the authors didn't provide us with a handy "free" function, and this isn't C++ - so no RAII.

We have a few options. My favourite is to take a moment:

![](./spockpain.jpg)
