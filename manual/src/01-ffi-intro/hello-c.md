# Hello C

Let's start with a little bit of Rust. We'll make a new project.

```bash
cargo new hello_c
```

As usual, you'll have a project skeleton:

```
.
├── Cargo.toml
└── src
    └── main.rs
```

In the `src` directory, make a new file named `hello.c`. This is our revolutionary new C library:

```c
#include <stdio.h>

void say_hello() {
    printf("Hello, world!\n");
}
```

Your directory structure should look like this:

```rust
.
├── Cargo.toml
└── src
    ├── hello.c
    └── main.rs
```

> We're not going to bother making a header file, CMake, Makefile or anything else. Yet.
