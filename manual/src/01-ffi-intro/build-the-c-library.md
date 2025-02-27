# Build the C Library

Sadly, simply having a `.c` file in your project isn't enough to make it work.

> Note: this is where we find out if everyone's laptop has the right software installed!

There's a lot of ways to build a C library. It's one of the reasons to love Cargo!
We're going to use the `cc` crate, which will try its very best to find a workable
C compiler on your system, invoke it to build the library, and link it to your Rust
program.

First, add the `cc` crate to your `Cargo.toml`:

```toml
[build-dependencies]
cc = "1.0"
```

Now we're going to make a `build.rs` file. If you haven't used `build.rs` before,
it's a special file that Cargo will run before building your project.

In the root of your project (**NOT** the `src` directory), make a new file named `build.rs`:

```rust
fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```

Your tree should look like this:

```
.
├── build.rs
├── Cargo.toml
└── src
    ├── hello.c
    └── main.rs
```

So, let's see what happens! If all goes well:

```
cargo run
Hello, world!
```

And hidden in your `target/debug` directory there's even a `libhello.a` - a static C library.

## But What If It Doesn't Work?

This is the fun part. If you don't have a C compiler installed, it's not a C
compiler that Rust can find, you'll have a not overly helpful error message.

* On **Windows**, you need the [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/).
* On **Mac**, you need to install the Xcode Command Line Tools. You can do this by running `xcode-select --install` in your terminal.
* On **Linux**, you need to install `build-essential`. I personally like to install `clang` as well.
