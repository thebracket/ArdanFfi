# How to Avoid Doing FFI

> If your goal is not do FFI, this may have been the wrong class! With that said, it's important to think about alternatives.

You could:

* Wrap up the code you need in another language in an executable and run it with `std::process::Command`.
    * Now every call requires that you setup its inputs.
    * Every call requires that the OS load the program, allocate it, etc.
    * Now you have to read the input.
    * And worst of all - you aren't writing Rust!
* You could put it on a microservice
    * Now you pay for a network call every time you need it.
    * You *still* have to wrap the input/output, and
    * You're *still* not writing Rust.
